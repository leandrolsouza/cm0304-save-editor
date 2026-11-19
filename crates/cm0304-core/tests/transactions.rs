use cm0304_core::{
    database,
    editor::Edit,
    transaction::{self, Transaction},
};
use std::{collections::BTreeSet, fs, path::PathBuf, sync::atomic::AtomicBool};
fn sample() -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap()
}
fn plan() -> Transaction {
    Transaction {
        edits: vec![
            Edit {
                uid: 356,
                attribute: "state.morale".into(),
                value: 20,
                policy: None,
            },
            Edit {
                uid: 2001642,
                attribute: "passing".into(),
                value: 20,
                policy: None,
            },
        ],
        heal_injuries: BTreeSet::from([356]),
        clear_suspensions: BTreeSet::from([5640794]),
        staff_edits: vec![cm0304_core::staff_editor::Edit {
            uid: 4950,
            attribute: "coachingOutfieldPlayers".into(),
            value: 20,
        }],
        staff_header_edits: vec![cm0304_core::staff_header::Edit {
            uid: 4950,
            potential_ability: Some(180),
            ..Default::default()
        }],
    }
}
#[test]
fn mixed_transaction_relocates_fields_and_keeps_both_operations() {
    let bytes = sample();
    let (output, report) = transaction::apply(&bytes, &plan()).unwrap();
    assert_eq!(report.changes.len(), 3);
    assert_eq!(report.staff_changes.len(), 1);
    assert_eq!(report.removals.len(), 2);
    assert_eq!(report.removed_bytes, 96);
    assert_eq!(report.size_before - report.size_after, 96);
    assert_eq!(report.source_sha256, cm0304_core::container::hash(&bytes));
    assert_eq!(report.copy_sha256, cm0304_core::container::hash(&output));
    let (db, base) = database::load(&output).unwrap();
    let staff = cm0304_core::staff::profiles(&db, &output, base).unwrap();
    assert_eq!(
        staff
            .iter()
            .find(|p| p.uid == 4950)
            .unwrap()
            .potential_ability,
        180
    );
    assert_eq!(
        staff
            .iter()
            .find(|p| p.uid == 4950)
            .unwrap()
            .attributes
            .as_ref()
            .unwrap()["coachingOutfieldPlayers"],
        20
    );
    let players = db.players(base).unwrap();
    assert_eq!(
        players
            .iter()
            .find(|p| p.uid == 2001642)
            .unwrap()
            .attributes["passing"],
        20
    );
    let healed = db
        .people
        .iter()
        .find(|p| p.uid == 356)
        .unwrap()
        .playing
        .as_ref()
        .unwrap();
    assert_eq!(healed.state.morale, 20);
    assert!(healed.health_records.injuries.is_empty());
    assert!(
        db.people
            .iter()
            .find(|p| p.uid == 5640794)
            .unwrap()
            .playing
            .as_ref()
            .unwrap()
            .health_records
            .bans
            .is_empty()
    );
    let mut invalid = plan();
    invalid.heal_injuries.insert(i32::MAX);
    assert!(transaction::apply(&bytes, &invalid).is_err());
    assert!(transaction::apply(&bytes, &Transaction::default()).is_err());
}
#[test]
fn mixed_snapshot_save_is_exclusive_and_checks_source_before_persisting() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.cm4");
    let target = temp.path().join("mixed.cm4");
    let input = sample();
    fs::write(&source, &input).unwrap();
    assert!(
        transaction::save_snapshot(&source, &target, &plan(), &input, &AtomicBool::new(true))
            .is_err()
    );
    assert!(!target.exists());
    let report =
        transaction::save_snapshot(&source, &target, &plan(), &input, &AtomicBool::new(false))
            .unwrap();
    assert_eq!(fs::read(&source).unwrap(), input);
    assert_eq!(
        cm0304_core::container::hash(&fs::read(&target).unwrap()),
        report.copy_sha256
    );
    assert!(
        transaction::save_snapshot(&source, &target, &plan(), &input, &AtomicBool::new(false))
            .is_err()
    );
    fs::write(&source, b"source changed").unwrap();
    let second = temp.path().join("second.cm4");
    assert!(
        transaction::save_snapshot(&source, &second, &plan(), &input, &AtomicBool::new(false))
            .is_err()
    );
    assert!(!second.exists());
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 2);
}
