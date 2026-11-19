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
