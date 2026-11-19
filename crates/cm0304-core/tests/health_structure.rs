use cm0304_core::{container, database};
use std::{fs, path::PathBuf};

// Structural research regression, not a declaration that healing is supported.
#[test]
fn removing_one_injury_preserves_all_player_values_and_unrelated_payloads() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap();
    let (db, base) = database::load(&input).unwrap();
    let person = db.people.iter().find(|p| p.uid == 356).unwrap();
    let health = &person.playing.as_ref().unwrap().health_records;
    assert_eq!(health.injuries.len(), 1);
    let count = health.injury_count_offset.unwrap();
    let record = &health.injuries[0];
    let before_container = container::read_container(&input).unwrap();
    let block = before_container
        .blocks
        .iter()
        .find(|b| b.name == "game_db" && b.extension == ".dat")
        .unwrap();
    let mut payload = input[base..base + block.size].to_vec();
    payload.drain(record.offset..record.offset + record.bytes.len());
    payload[count] = 0;
    let output = container::replace_block(&input, "game_db", ".dat", &payload).unwrap();
    let (actual, report) = cm0304_core::health::remove_injuries(&input, &[356]).unwrap();
    assert_eq!(actual, output);
    assert_eq!(report.removals.len(), 1);
    assert_eq!(report.removals[0].removed_bytes, record.bytes.len());
    let (after, new_base) = database::load(&output).unwrap();
    assert_eq!(base, new_base);
    assert_eq!(output.len() + record.bytes.len(), input.len());
    assert_eq!(db.people.len(), after.people.len());
    let healed = after
        .people
        .iter()
        .find(|p| p.uid == 356)
        .unwrap()
        .playing
        .as_ref()
        .unwrap();
    assert!(healed.health_records.injuries.is_empty());
    assert_eq!(
        health.injury_status.as_ref().unwrap().bytes,
        healed.health_records.injury_status.as_ref().unwrap().bytes
    );
    let semantic_player = |p: &database::Player| {
        let mut value = serde_json::to_value(p).unwrap();
        value.as_object_mut().unwrap().remove("offsets");
        for field in value["fields"].as_object_mut().unwrap().values_mut() {
            field.as_object_mut().unwrap().remove("offset");
        }
        value
    };
    let old_players = db.players(base).unwrap();
    let new_players = after.players(new_base).unwrap();
    assert_eq!(old_players.len(), new_players.len());
    for (old, new) in old_players.iter().zip(&new_players) {
        assert_eq!(
            semantic_player(old),
            semantic_player(new),
            "UID {}",
            old.uid
        );
    }
    let new_container = container::read_container(&output).unwrap();
    for (old, new) in before_container.blocks.iter().zip(&new_container.blocks) {
        if old.name != "game_db" || old.extension != ".dat" {
            assert_eq!(old.sha256, new.sha256);
        }
    }
}

#[test]
fn injury_removal_handles_multiple_players_noops_and_invalid_selections() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_modificado.cm4"),
    )
    .unwrap();
    let (db, _) = database::load(&input).unwrap();
    let mut selected = db
        .people
        .iter()
        .filter(|p| {
            p.playing
                .as_ref()
                .is_some_and(|v| !v.health_records.injuries.is_empty())
        })
        .take(3)
        .map(|p| p.uid)
        .collect::<Vec<_>>();
    assert_eq!(selected.len(), 3);
    selected.reverse(); // Input order must not become splice order.
    let (out, report) = cm0304_core::health::remove_injuries(&input, &selected).unwrap();
    let (after, _) = database::load(&out).unwrap();
    assert_eq!(report.removals.len(), 3);
    assert_eq!(
        input.len() - out.len(),
        report
            .removals
            .iter()
            .map(|r| r.removed_bytes)
            .sum::<usize>()
    );
    let after_people = after
        .people
        .iter()
        .map(|p| (p.uid, p))
        .collect::<std::collections::BTreeMap<_, _>>();
    for person in &db.people {
        let Some(old) = &person.playing else { continue };
        let new = after_people[&person.uid].playing.as_ref().unwrap();
        assert_eq!(
            old.health_records
                .bans
                .iter()
                .map(|r| &r.bytes)
                .collect::<Vec<_>>(),
            new.health_records
                .bans
                .iter()
                .map(|r| &r.bytes)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            old.health_records.injury_status.as_ref().map(|r| &r.bytes),
            new.health_records.injury_status.as_ref().map(|r| &r.bytes)
        );
        if selected.contains(&person.uid) {
            assert!(new.health_records.injuries.is_empty());
        } else {
            assert_eq!(
                old.health_records
                    .injuries
                    .iter()
                    .map(|r| &r.bytes)
                    .collect::<Vec<_>>(),
                new.health_records
                    .injuries
                    .iter()
                    .map(|r| &r.bytes)
                    .collect::<Vec<_>>()
            );
        }
    }
    let healthy = db
        .people
        .iter()
        .find(|p| {
            p.playing
                .as_ref()
                .is_some_and(|v| v.health_records.injuries.is_empty())
        })
        .unwrap()
        .uid;
    assert_eq!(
        cm0304_core::health::remove_injuries(&input, &[healthy])
            .unwrap()
            .0,
        input
    );
    assert!(cm0304_core::health::remove_injuries(&input, &[]).is_err());
    assert!(cm0304_core::health::remove_injuries(&input, &[selected[0], selected[0]]).is_err());
    assert!(cm0304_core::health::remove_injuries(&input, &[selected[0], -12345]).is_err());
}

#[test]
fn combined_suspension_and_injury_removal_preserves_bookings_and_status() {
    use cm0304_core::health::{self, RemovalKind};
    use std::collections::{BTreeMap, BTreeSet};
    for sample in ["save_inicial.cm4", "save_modificado.cm4"] {
        let input = fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../game/user data/games")
                .join(sample),
        )
        .unwrap();
        let (db, _) = database::load(&input).unwrap();
        let banned = db
            .people
            .iter()
            .filter(|p| {
                p.playing
                    .as_ref()
                    .is_some_and(|v| !v.health_records.bans.is_empty())
            })
            .map(|p| p.uid)
            .collect::<BTreeSet<_>>();
        assert!(!banned.is_empty());
        let injured = db
            .people
            .iter()
            .find(|p| {
                p.playing.as_ref().is_some_and(|v| {
                    !v.health_records.injuries.is_empty() && !v.health_records.bans.is_empty()
                })
            })
            .or_else(|| {
                db.people.iter().find(|p| {
                    p.playing
                        .as_ref()
                        .is_some_and(|v| !v.health_records.injuries.is_empty())
                })
            })
            .unwrap()
            .uid;
        let mut selections = banned
            .iter()
            .map(|uid| (*uid, RemovalKind::Suspensions))
            .collect::<Vec<_>>();
        selections.push((injured, RemovalKind::Injuries));
        let (out, report) = health::remove_records(&input, &selections).unwrap();
        assert_eq!(report.removals.len(), banned.len() + 1);
        let (after, _) = database::load(&out).unwrap();
        let index = after
            .people
            .iter()
            .map(|p| (p.uid, p))
            .collect::<BTreeMap<_, _>>();
        let records = |spans: &[database::RecordSpan]| {
            spans.iter().map(|r| r.bytes.clone()).collect::<Vec<_>>()
        };
        for person in &db.people {
            let Some(old) = &person.playing else { continue };
            let new = &index[&person.uid].playing.as_ref().unwrap().health_records;
            let old = &old.health_records;
            assert_eq!(records(&old.bookings), records(&new.bookings));
            assert_eq!(
                old.injury_status.as_ref().map(|r| &r.bytes),
                new.injury_status.as_ref().map(|r| &r.bytes)
            );
            if banned.contains(&person.uid) {
                assert!(new.bans.is_empty());
            } else {
                assert_eq!(records(&old.bans), records(&new.bans));
            }
            if person.uid == injured {
                assert!(new.injuries.is_empty());
            } else {
                assert_eq!(records(&old.injuries), records(&new.injuries));
            }
        }
    }
}
