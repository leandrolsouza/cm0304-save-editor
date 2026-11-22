use cm0304_core::{container::read_container, database, editor::Edit, market_lab, transaction};
use std::{fs, path::Path};

fn payload<'a>(bytes: &'a [u8], name: &str) -> &'a [u8] {
    let block = read_container(bytes)
        .unwrap()
        .blocks
        .into_iter()
        .find(|block| block.name == name && block.extension == ".dat")
        .unwrap();
    &bytes[block.payload_offset..block.payload_offset + block.size]
}

#[test]
fn braga_market_journal_keeps_each_operation_and_preserves_transfers() {
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../game/user data/games/save_inicial.cm4");
    let original = fs::read(&source).unwrap();
    let temp = tempfile::tempdir().unwrap();
    let restricted = temp.path().join("restricted.cm4");
    let restored = temp.path().join("restored.cm4");
    let available = temp.path().join("available.cm4");

    market_lab::synthesize(&source, &restricted, 1855, 68).unwrap();
    market_lab::synthesize(&restricted, &restored, 1855, 4).unwrap();
    market_lab::synthesize(&restored, &available, 1855, 6).unwrap();

    let restricted = fs::read(restricted).unwrap();
    let restored = fs::read(restored).unwrap();
    let available = fs::read(available).unwrap();
    assert!(market_lab::supports_journal(&original));
    assert!(market_lab::supports_journal(&available));
    let mut incompatible = available.clone();
    let block = read_container(&incompatible)
        .unwrap()
        .blocks
        .into_iter()
        .find(|block| block.name == "edit_session_man")
        .unwrap();
    incompatible[block.payload_offset + 12] = 9;
    assert!(!market_lab::supports_journal(&incompatible));
    let invalid_source = temp.path().join("unknown-journal.cm4");
    let invalid_copy = temp.path().join("unknown-journal-copy.cm4");
    fs::write(&invalid_source, incompatible).unwrap();
    let error = market_lab::synthesize(&invalid_source, &invalid_copy, 1855, 4)
        .err()
        .unwrap();
    assert!(error.contains("Diário contém operações"));
    assert!(!invalid_copy.exists());
    assert_eq!(fs::read(&source).unwrap(), original);
    for (copy, expected_status) in [(&restricted, 68), (&restored, 4), (&available, 6)] {
        let (db, _) = database::load(copy).unwrap();
        let quim = db.people.iter().find(|person| person.uid == 1855).unwrap();
        assert_eq!(quim.contracts[0].transfer_status, expected_status);
        assert_eq!(
            payload(copy, "transfer_man"),
            payload(&original, "transfer_man"),
        );
    }

    let restricted_journal = payload(&restricted, "edit_session_man");
    let restored_journal = payload(&restored, "edit_session_man");
    let undo = restored_journal.strip_prefix(restricted_journal).unwrap();
    assert_eq!(undo.len(), 65);
    assert_eq!(undo[2..6], 32787_i32.to_le_bytes());
    assert_eq!(undo[7..11], 321_i32.to_le_bytes());
    assert_eq!(undo[13..17], 921_i32.to_le_bytes());
    assert_eq!((undo[34], undo[50]), (68, 4));
    let availability = payload(&available, "edit_session_man")
        .strip_prefix(restored_journal)
        .unwrap();
    assert_eq!(availability.len(), 65);
    assert_eq!((availability[34], availability[50]), (4, 6));

    let invalid = temp.path().join("invalid.cm4");
    assert!(market_lab::synthesize(&source, &invalid, 1855, 69).is_err());
    assert!(!invalid.exists());
}

#[test]
fn market_edit_and_wage_are_reviewed_in_one_transaction() {
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../game/user data/games/save_inicial.cm4");
    let original = fs::read(source).unwrap();
    let mut plan: transaction::Transaction = serde_json::from_str("{}").unwrap();
    assert!(plan.is_empty());
    plan.market_statuses.insert(1855, 6);
    let (mut source_db, _) = database::load(&original).unwrap();
    assert_eq!(
        market_lab::editable_statuses(&source_db, &original).get(&736046),
        Some(&5)
    );
    source_db
        .people
        .iter_mut()
        .find(|person| person.uid != 736046 && person.playing.is_none())
        .unwrap()
        .uid = 736046;
    let eligibility = market_lab::editable_statuses(&source_db, &original);
    assert!(!eligibility.contains_key(&736046));
    assert_eq!(eligibility.get(&1855), Some(&4));
    plan.market_statuses.insert(736046, 7);
    plan.edits.push(Edit {
        uid: 1855,
        attribute: "contract.0.weeklyWage".into(),
        value: 1900,
        policy: None,
    });

    let (output, report) = transaction::apply(&original, &plan).unwrap();
    let change = report
        .changes
        .iter()
        .find(|change| change.uid == 1855 && change.attribute == "contract.0.transferStatus")
        .unwrap();
    assert_eq!((change.before, change.after), (4, 6));
    assert_eq!(report.changes.len(), 3);
    assert_eq!(report.size_after - report.size_before, 130);
    assert_eq!(report.market_journal_entries_added, 2);
    assert_eq!(report.market_journal_bytes_added, 130);
    assert_eq!(
        payload(&output, "transfer_man"),
        payload(&original, "transfer_man")
    );
    let (db, _) = database::load(&output).unwrap();
    let quim = db.people.iter().find(|person| person.uid == 1855).unwrap();
    assert_eq!(quim.contracts[0].weekly_wage, 1900);
    assert_eq!(quim.contracts[0].transfer_status, 6);
    let quinio = db
        .people
        .iter()
        .find(|person| person.uid == 736046)
        .unwrap();
    assert_ne!(quinio.team, quim.team);
    assert_eq!(quinio.contracts[0].transfer_status, 7);
    let journal = payload(&output, "edit_session_man");
    let reserve_entry = &journal[journal.len() - 65..];
    assert_eq!(reserve_entry[13..17], quim.team.to_le_bytes());
}
