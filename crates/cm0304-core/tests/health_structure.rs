use cm0304_core::{container, database};
use std::{fs, path::PathBuf};

#[test]
fn removing_a_game_generated_suspension_preserves_the_players_bookings() {
    use cm0304_core::transaction::{self, Transaction};

    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/v05_ban_variants7d.cm4"),
    )
    .unwrap();
    let uid = 744546;
    let (output, report) = transaction::apply(
        &input,
        &Transaction {
            clear_suspensions: [uid].into(),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(report.removals.len(), 1);
    for (save, expected_bans) in [(&input, 1), (&output, 0)] {
        let (db, _) = database::load(save).unwrap();
        let player = db.people.iter().find(|player| player.uid == uid).unwrap();
        let health = &player.playing.as_ref().unwrap().health_records;
        assert_eq!(health.bans.len(), expected_bans);
        assert_eq!(health.bookings.len(), 1);
        assert_eq!(health.bookings[0].bytes.as_slice(), &[0, 4, 1, 1]);
    }
}

#[test]
fn health_batch_previews_only_new_removals_on_a_real_save() {
    use cm0304_core::{batch, transaction};
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap();
    let (db, base) = database::load(&input).unwrap();
    let mut plan = transaction::Transaction {
        heal_injuries: [356].into(),
        ..Default::default()
    };
    let counts = [356, 5640794, 2001642].map(|uid| {
        let person = db.people.iter().find(|p| p.uid == uid).unwrap();
        let records = &person.playing.as_ref().unwrap().health_records;
        batch::HealthCounts {
            uid,
            name: "Jogador",
            injuries: records.injuries.len(),
            suspensions: records.bans.len(),
        }
    });
    let targets = batch::project_health_removals(&mut plan, &counts, true, true);
    assert_eq!(
        targets
            .iter()
            .filter(|t| t.before != t.after)
            .map(|t| (t.uid, t.attribute.as_str()))
            .collect::<Vec<_>>(),
        [(5640794, "clearSuspensions")]
    );
    assert_eq!(plan.heal_injuries, [356].into());
    assert_eq!(plan.clear_suspensions, [5640794].into());
    let repeated = batch::project_health_removals(&mut plan, &counts, true, true);
    assert!(repeated.iter().all(|t| t.before == t.after));
    let players = db.players(base).unwrap();
    let (recovery, targets) = batch::project(
        &players,
        &batch::Request {
            uids: vec![356],
            attribute: String::new(),
            operation: batch::Operation::Recover,
        },
    )
    .unwrap();
    assert!(
        targets
            .iter()
            .any(|t| t.attribute == "state.condition" && t.before < t.after)
    );
    plan.merge_fields(&players, recovery).unwrap();
    let (output, report) = transaction::apply(&input, &plan).unwrap();
    assert_eq!(report.removals.len(), 2);
    let (edited, edited_base) = database::load(&output).unwrap();
    let recovered = edited
        .players(edited_base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 356)
        .unwrap();
    assert_eq!(recovered.fields["state.condition"].value, 10_000);
    let gyan = edited.people.iter().find(|p| p.uid == 5640794).unwrap();
    let records = &gyan.playing.as_ref().unwrap().health_records;
    assert!(records.bans.is_empty());
    assert_eq!(records.bookings.len(), 1);
}

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
fn healing_zetterberg_preserves_his_staff_profile_and_relocates_its_offsets() {
    const UID: i32 = 600;
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_modificado.cm4"),
    )
    .unwrap();
    let (output, report) = cm0304_core::health::remove_injuries(&input, &[UID]).unwrap();
    assert_eq!(report.removals.len(), 1);
    let removed = report.removals[0].removed_bytes;
    assert_eq!(removed, 38);

    let read_profile = |bytes: &[u8]| {
        let (db, base) = database::load(bytes).unwrap();
        let person = db.people.iter().find(|person| person.uid == UID).unwrap();
        let injuries = cm0304_core::health::injuries(&db, person);
        let member = cm0304_core::staff::profiles(&db, bytes, base)
            .unwrap()
            .into_iter()
            .find(|member| member.uid == UID)
            .unwrap();
        assert!(member.is_player);
        (injuries, member)
    };
    let (before_injuries, mut before_staff) = read_profile(&input);
    let (after_injuries, after_staff) = read_profile(&output);
    assert_eq!(before_injuries.len(), 1);
    assert_eq!(before_injuries[0].estimated_days_remaining, Some(6));
    assert!(after_injuries.is_empty());
    before_staff.info_offset -= removed;
    before_staff.attributes_offset = before_staff
        .attributes_offset
        .map(|offset| offset - removed);
    assert_eq!(
        serde_json::to_value(before_staff).unwrap(),
        serde_json::to_value(after_staff).unwrap()
    );
}

#[test]
fn injury_removal_handles_multiple_players_noops_and_invalid_selections() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_modificado.cm4"),
    )
    .unwrap();
    let (db, _) = database::load(&input).unwrap();
    let lengths = [38, 40, 42];
    let mut selected = lengths.map(|length| {
        db.people
            .iter()
            .find(|person| {
                person.playing.as_ref().is_some_and(|playing| {
                    let records = &playing.health_records.injuries;
                    records.len() == 1 && records[0].bytes.len() == length
                })
            })
            .unwrap_or_else(|| panic!("Missing {length}-byte injury fixture"))
            .uid
    });
    selected.reverse(); // Input order must not become splice order.
    let (out, report) = cm0304_core::health::remove_injuries(&input, &selected).unwrap();
    let (after, _) = database::load(&out).unwrap();
    assert_eq!(report.removals.len(), 3);
    let mut removed_lengths = report
        .removals
        .iter()
        .map(|removal| removal.removed_bytes)
        .collect::<Vec<_>>();
    removed_lengths.sort_unstable();
    assert_eq!(removed_lengths, lengths);
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
        if sample == "save_modificado.cm4" {
            for (uid, length) in [(11944, 19), (1873, 58), (5601832, 19)] {
                let removal = report
                    .removals
                    .iter()
                    .find(|removal| removal.uid == uid && removal.kind == RemovalKind::Suspensions)
                    .unwrap();
                assert_eq!((removal.count, removal.removed_bytes), (1, length));
            }
        }
        let (second_output, second_report) = health::remove_records(&out, &selections).unwrap();
        assert!(second_report.removals.is_empty());
        assert_eq!(second_output, out);
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
