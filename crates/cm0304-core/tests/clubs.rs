use cm0304_core::{clubs, database};
use std::{fs, path::PathBuf};
fn sample(name: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games")
            .join(name),
    )
    .unwrap()
}

#[test]
fn club_finance_variants_remain_aligned_and_isolate_balance_from_players() {
    for name in ["save_inicial.cm4", "save_modificado.cm4"] {
        let input = sample(name);
        let (db, base) = database::load(&input).unwrap();
        assert_eq!(db.clubs.len(), 3554);
        assert_eq!(db.sections.len(), 23);
        let players = serde_json::to_vec(&db.players(base).unwrap()).unwrap();
        let mut kinds = std::collections::BTreeSet::new();
        for club in &db.clubs {
            let d = club.details.as_ref().unwrap();
            kinds.insert(d.finance_kind);
            assert!(d.record_offset < d.infrastructure_offset);
            assert!(d.infrastructure_offset + 17 <= d.finance_offset);
            assert_eq!(
                d.finance_raw,
                input[base + d.finance_offset..base + d.finance_offset + d.finance_raw.len()]
            );
            match d.finance_kind {
                0 | 1 => {
                    let offset = d.balance_offset.unwrap();
                    assert_eq!(
                        d.finance_raw.len(),
                        if d.finance_kind == 0 { 4 } else { 438 }
                    );
                    // One representative of each variant is enough for the costly reparse.
                    if db
                        .clubs
                        .iter()
                        .find(|c| c.details.as_ref().unwrap().finance_kind == d.finance_kind)
                        .unwrap()
                        .uid
                        == club.uid
                    {
                        let mut edited = input.clone();
                        edited[base + offset..base + offset + 4]
                            .copy_from_slice(&(-1234567i32).to_le_bytes());
                        let (after, after_base) = database::load(&edited).unwrap();
                        assert_eq!(after_base, base);
                        assert_eq!(after.sections.len(), 23);
                        assert_eq!(
                            serde_json::to_vec(&after.players(base).unwrap()).unwrap(),
                            players
                        );
                        for (before, after) in db.clubs.iter().zip(&after.clubs) {
                            assert_eq!(before.uid, after.uid);
                            let expected = if before.uid == club.uid {
                                Some(-1234567)
                            } else {
                                before.details.as_ref().unwrap().balance
                            };
                            assert_eq!(after.details.as_ref().unwrap().balance, expected);
                        }
                    }
                }
                2 => {
                    assert!(d.finance_raw.is_empty());
                    assert!(d.balance.is_none() && d.balance_offset.is_none());
                }
                other => panic!("unsupported finance kind {other}"),
            }
        }
        assert_eq!(kinds, [0, 1, 2].into());
        println!("{name}: finance variants {kinds:?}");
    }
}

#[test]
fn club_uids_and_memberships_resolve_both_samples_without_name_joins() {
    for name in ["save_inicial.cm4", "save_modificado.cm4"] {
        let input = sample(name);
        let (mut db, base) = database::load(&input).unwrap();
        let profiles = clubs::profiles(&db).unwrap();
        let fortaleza = profiles.iter().find(|c| c.uid == 104750).unwrap();
        assert_eq!(fortaleza.name, "Fortaleza EC");
        assert_eq!(fortaleza.player_uids.len(), 9);
        assert!(fortaleza.player_uids.contains(&2001642));
        let braga = profiles.iter().find(|c| c.uid == 1488).unwrap();
        assert!(braga.player_uids.contains(&1855));
        assert!(braga.staff_uids.contains(&4950));
        assert!(braga.staff_uids.contains(&4100749));
        let northampton = profiles.iter().find(|c| c.uid == 689).unwrap();
        assert!(northampton.player_uids.contains(&5101855));
        let arsenal = profiles.iter().find(|c| c.short_name == "Arsenal").unwrap();
        assert!(!arsenal.player_uids.contains(&5101855));
        let anderlecht = profiles
            .iter()
            .find(|c| c.short_name == "Anderlecht")
            .unwrap();
        assert!(anderlecht.player_uids.contains(&600) && anderlecht.staff_uids.contains(&600));
        assert_eq!(
            profiles.iter().map(|c| c.player_uids.len()).sum::<usize>(),
            db.players(base)
                .unwrap()
                .iter()
                .filter(|p| p.club.is_some())
                .count()
        );
        assert_eq!(db.sections.len(), 23);
        let first_teams = db
            .teams
            .iter()
            .filter(|t| {
                t.container_type == 0 && t.container_index == fortaleza.index as i32 && t.squad == 0
            })
            .collect::<Vec<_>>();
        assert_eq!(first_teams.len(), 1);
        assert_eq!(first_teams[0].reputation, 4500);
        let offset = base + first_teams[0].reputation_offset;
        assert_eq!(&input[offset..offset + 2], &4500i16.to_le_bytes());
        let braga_uid = braga.uid;
        let braga_index = braga.index;
        let roster = braga.player_uids.clone();
        db.clubs[braga_index].name = "Fortaleza EC".into();
        let renamed = clubs::profiles(&db).unwrap();
        assert_eq!(
            renamed
                .iter()
                .find(|c| c.uid == braga_uid)
                .unwrap()
                .player_uids,
            roster
        );
        assert!(
            !renamed
                .iter()
                .find(|c| c.uid == 104750)
                .unwrap()
                .player_uids
                .contains(&1855)
        );
    }
}

#[test]
fn club_links_reject_bad_references_and_keep_national_teams_separate() {
    let input = sample("save_inicial.cm4");
    let (mut db, _) = database::load(&input).unwrap();
    let person = db.people.iter().position(|p| p.uid == 2001642).unwrap();
    let original = db.people[person].team;
    db.people[person].team = i32::MAX;
    assert!(clubs::profiles(&db).unwrap_err().contains("2001642"));
    db.people[person].team = original;
    let national = db.teams.len();
    db.teams.push(database::Team {
        squad: 0,
        reputation: 0,
        reputation_offset: 0,
        container_type: 1,
        container_index: 0,
    });
    db.people[person].team = national as i32;
    assert!(
        !clubs::profiles(&db)
            .unwrap()
            .iter()
            .any(|c| c.player_uids.contains(&2001642))
    );
    db.teams[national].container_index = i32::MAX;
    assert!(
        clubs::profiles(&db)
            .unwrap_err()
            .contains("seleção inválida")
    );
    db.people[person].team = original;
    db.clubs[0].uid = db.clubs[1].uid;
    assert!(
        clubs::profiles(&db)
            .unwrap_err()
            .contains("UID de clube repetido")
    );
}
