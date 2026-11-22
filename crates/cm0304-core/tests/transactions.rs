use cm0304_core::{
    database,
    editor::Edit,
    transaction::{self, Transaction},
};
use std::{collections::BTreeSet, fs, path::PathBuf, sync::atomic::AtomicBool};

#[test]
fn rejects_unknown_operation_in_a_plan_with_valid_edits() {
    let valid = r#"{"edits":[{"uid":2001642,"attribute":"passing","value":20}]}"#;
    let plan: Transaction = serde_json::from_str(valid).unwrap();
    assert_eq!(plan.edits.len(), 1);
    assert!(plan.heal_injuries.is_empty());

    let invalid =
        r#"{"edits":[{"uid":2001642,"attribute":"passing","value":20}],"clearInjuries":[356]}"#;
    let error = serde_json::from_str::<Transaction>(invalid).unwrap_err();
    assert!(error.to_string().contains("unknown field `clearInjuries`"));
}

#[test]
fn mixed_edit_reproduces_verified_copies_for_four_career_checkpoints() {
    let saves = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../game/user data/games");
    let variants = [
        (
            "save_inicial.cm4",
            1590,
            4950,
            13,
            1488,
            625326,
            "49f7f5be4388bf52a2059919cd3fbdc91426535da288095be2dda5a1b5de0b2c",
        ),
        (
            "save_modificado.cm4",
            1590,
            4950,
            13,
            1488,
            1564831,
            "96dfb1b936adcc9ee3fb8a49b07ffbfdb6f24682d55592a1305085ad6aa320d3",
        ),
        (
            "matrizbinicial.cm4",
            1751,
            315966,
            12,
            104750,
            2969489,
            "5c9a366452631877f421900a2ff3c2dc18b9773b96d7c8e460b2201774027d12",
        ),
        (
            "matrizbavancado.cm4",
            1751,
            315966,
            12,
            104750,
            2816969,
            "fc84040974e05b55a64664e33fbbfb9b106448d039c090a6d913402487c37738",
        ),
    ];
    for (name, wage, staff_uid, training, club_uid, balance, expected_hash) in variants {
        let source = saves.join(name);
        let before = fs::read(&source).unwrap();
        let temp = tempfile::tempdir().unwrap();
        let copy = temp.path().join("copy.cm4");
        let plan = Transaction {
            edits: [("passing", 20), ("contract.0.weeklyWage", wage)]
                .into_iter()
                .map(|(attribute, value)| Edit {
                    uid: 2001642,
                    attribute: attribute.into(),
                    value,
                    policy: None,
                })
                .collect(),
            staff_edits: vec![cm0304_core::staff_editor::Edit {
                uid: staff_uid,
                attribute: "coachingOutfieldPlayers".into(),
                value: training,
            }],
            club_balances: [(club_uid, balance)].into(),
            ..Default::default()
        };
        let report =
            transaction::save_snapshot(&source, &copy, &plan, &before, &AtomicBool::new(false))
                .unwrap();
        assert_eq!(report.copy_sha256, expected_hash, "{name}");
        assert_eq!(
            cm0304_core::container::hash(&fs::read(copy).unwrap()),
            expected_hash,
            "{name}"
        );
        assert_eq!(fs::read(source).unwrap(), before, "{name}");
    }
}

fn sample() -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap()
}
fn plan() -> Transaction {
    Transaction {
        club_balances: [(1488, -250_000)].into(),
        club_wage_budgets: [(1488, 80_000)].into(),
        club_stadium_capacities: [(
            1488,
            transaction::StadiumCapacities {
                capacity: 31_234,
                seated_capacity: 24_567,
            },
        )]
        .into(),
        club_training: [(1488, 20)].into(),
        club_reputation: [(1488, 10_000)].into(),
        club_youth_setup: [(1488, 19)].into(),
        club_transfer_budgets: [(
            1488,
            transaction::TransferBudgets {
                season: 900_000,
                remaining: 700_000,
            },
        )]
        .into(),
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
            Edit {
                uid: 5101855,
                attribute: "contract.1.weeklyWage".into(),
                value: 571,
                policy: None,
            },
            Edit {
                uid: 5101855,
                attribute: "contract.1.expires.year".into(),
                value: 2004,
                policy: None,
            },
            Edit {
                uid: 5101855,
                attribute: "contract.1.expires.day".into(),
                value: 59,
                policy: None,
            },
        ],
        market_statuses: [(1855, 6)].into(),
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
    assert_eq!(report.changes.len(), 16);
    assert_eq!(report.staff_changes.len(), 1);
    assert_eq!(report.removals.len(), 2);
    assert_eq!(report.removed_bytes, 96);
    assert_eq!(report.size_before - report.size_after, 96 - 65);
    assert_eq!(report.source_sha256, cm0304_core::container::hash(&bytes));
    assert_eq!(report.copy_sha256, cm0304_core::container::hash(&output));
    let (db, base) = database::load(&output).unwrap();
    let quim = db.people.iter().find(|person| person.uid == 1855).unwrap();
    assert_eq!(quim.contracts[0].transfer_status, 6);
    let braga_index = db.clubs.iter().position(|club| club.uid == 1488).unwrap();
    let mut teams = db.teams.iter().filter(|team| {
        team.container_type == 0 && team.container_index == braga_index as i32 && team.squad == 0
    });
    let main = teams.next().unwrap();
    assert!(teams.next().is_none());
    assert_eq!(main.reputation, 10_000);
    let report_entry = report
        .changes
        .iter()
        .find(|change| change.attribute == "club.reputation")
        .unwrap();
    assert_eq!(
        (report_entry.uid, report_entry.after, report_entry.width),
        (1488, 10_000, 2)
    );
    assert_eq!(
        &output[base + main.reputation_offset..base + main.reputation_offset + 2],
        &10_000i16.to_le_bytes()
    );
    let balance = |uid| {
        db.clubs
            .iter()
            .find(|c| c.uid == uid)
            .unwrap()
            .details
            .as_ref()
            .unwrap()
            .balance
    };
    assert_eq!(balance(1488), Some(-250_000));
    let details = db
        .clubs
        .iter()
        .find(|c| c.uid == 1488)
        .unwrap()
        .details
        .as_ref()
        .unwrap();
    assert_eq!(details.training_raw, 20);
    assert_eq!(details.youth_setup_raw, 19);
    assert_eq!(details.transfer_budget_season, Some(900_000));
    assert_eq!(details.transfer_budget_remaining, Some(700_000));
    assert_eq!(details.wage_budget_weekly, Some(80_000));
    let stadiums = cm0304_core::clubs::stadiums_by_club(&db);
    let stadium = &stadiums[&1488].stadium;
    assert_eq!(
        (stadium.capacity, stadium.seated_capacity),
        (31_234, 24_567)
    );
    assert!(
        report
            .changes
            .iter()
            .any(|c| c.attribute == "club.transferBudgetSeason" && c.after == 900_000)
    );
    assert!(
        report
            .changes
            .iter()
            .any(|c| c.attribute == "club.transferBudgetRemaining" && c.after == 700_000)
    );
    assert_eq!(balance(104750), Some(1_250_000));
    let change = report
        .changes
        .iter()
        .find(|c| c.attribute == "club.balance")
        .unwrap();
    assert_eq!(
        (change.uid, change.before, change.after),
        (1488, 525_326, -250_000)
    );
    let (_, original_base) = database::load(&bytes).unwrap();
    assert_eq!(
        &bytes[change.offset..change.offset + 4],
        &525_326i32.to_le_bytes()
    );
    let relocated = base + (change.offset - original_base);
    assert_eq!(
        &output[relocated..relocated + 4],
        &(-250_000i32).to_le_bytes()
    );
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
    let contracts = &db
        .people
        .iter()
        .find(|p| p.uid == 5101855)
        .unwrap()
        .contracts;
    let (original_db, _) = database::load(&bytes).unwrap();
    let original_contracts = &original_db
        .people
        .iter()
        .find(|p| p.uid == 5101855)
        .unwrap()
        .contracts;
    let owner = &contracts[0];
    let loan = &contracts[1];
    assert_eq!(contracts.len(), original_contracts.len());
    assert_eq!(owner.entity_index, original_contracts[0].entity_index);
    assert_eq!(owner.weekly_wage, original_contracts[0].weekly_wage);
    assert_eq!(owner.expires.year, original_contracts[0].expires.year);
    assert_eq!(owner.expires.day, original_contracts[0].expires.day);
    assert_eq!(loan.entity_index, original_contracts[1].entity_index);
    assert_eq!(loan.weekly_wage, 571);
    assert_eq!((loan.expires.year, loan.expires.day), (2004, 59));
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
    let unsupported = Transaction {
        club_balances: [(104750, 1)].into(),
        ..Default::default()
    };
    assert!(
        transaction::apply(&bytes, &unsupported)
            .unwrap_err()
            .contains("registro financeiro completo")
    );
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
