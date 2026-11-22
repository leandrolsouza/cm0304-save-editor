use cm0304_core::{batch, database, roster_comparison, transaction};
use serde_json::to_value;
use std::{fs, path::PathBuf, sync::atomic::AtomicBool};

#[test]
fn an_unrepresentable_attribute_rejects_the_entire_batch() {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../game/user data/games/save_inicial.cm4");
    let input = fs::read(&source).unwrap();
    let (db, base) = database::load(&input).unwrap();
    let players = db.players(base).unwrap();
    let request = batch::Request {
        uids: vec![1855], // Quim's goalkeeper conversion can represent Passe 1.
        attribute: "passing".into(),
        operation: batch::Operation::Set(1),
    };
    let (edits, _) = batch::project(&players, &request).unwrap();
    let (output, _) = transaction::apply(
        &input,
        &transaction::Transaction {
            edits,
            ..Default::default()
        },
    )
    .unwrap();
    let (changed_db, changed_base) = database::load(&output).unwrap();
    let changed = changed_db
        .players(changed_base)
        .unwrap()
        .into_iter()
        .find(|player| player.uid == 1855)
        .unwrap();
    assert_eq!(changed.fields["passing"].value, 1);
    assert_eq!(changed.current_ability, 140);

    let mixed = batch::Request {
        uids: vec![1855, 2001642],
        ..request
    };
    let (edits, _) = batch::project(&players, &mixed).unwrap();
    let temp = tempfile::tempdir().unwrap();
    let destination = temp.path().join("lote.cm4");
    let error = transaction::save_snapshot(
        &source,
        &destination,
        &transaction::Transaction {
            edits,
            ..Default::default()
        },
        &input,
        &AtomicBool::new(false),
    )
    .unwrap_err();
    assert!(error.contains("Clodoaldo (UID 2001642)"));
    assert!(!destination.exists());
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
    assert_eq!(fs::read(&source).unwrap(), input);
}

#[test]
fn batch_personality_round_trips_alongside_player_state() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap();
    let (db, base) = database::load(&input).unwrap();
    let mut players = db.players(base).unwrap();
    let uid = 2001642;
    let request: batch::Request = serde_json::from_value(serde_json::json!({
        "uids": [uid],
        "operation": {"kind": "adjust", "value": [
            {"attribute": "personality.professionalism", "operation": {"kind": "set", "value": 19}},
            {"attribute": "state.morale", "operation": {"kind": "set", "value": 20}}
        ]}
    }))
    .unwrap();
    let (edits, targets) = batch::project(&players, &request).unwrap();
    assert_eq!(targets.len(), 2);
    assert!(targets.iter().all(|target| !target.limited));
    players
        .iter_mut()
        .find(|p| p.uid == 356)
        .unwrap()
        .fields
        .remove("personality.professionalism");
    let mixed = batch::Request {
        uids: vec![uid, 356],
        ..request
    };
    let (blocked_edits, preview) = batch::project(&players, &mixed).unwrap();
    assert!(blocked_edits.is_empty());
    assert!(preview.iter().any(|row| row.uid == 356 && row.unavailable));
    assert!(
        preview
            .iter()
            .any(|row| row.uid == uid && row.before != row.after)
    );
    let (written, _) = transaction::apply(
        &input,
        &transaction::Transaction {
            edits,
            ..Default::default()
        },
    )
    .unwrap();
    let (updated, base) = database::load(&written).unwrap();
    let player = updated
        .players(base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == uid)
        .unwrap();
    assert_eq!(player.fields["personality.professionalism"].value, 19);
    assert_eq!(player.fields["state.morale"].value, 20);
    assert_eq!(
        player.current_ability,
        players
            .iter()
            .find(|p| p.uid == uid)
            .unwrap()
            .current_ability
    );
}

#[test]
fn recovery_batch_restores_fitness_without_worsening_rest_or_morale() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap();
    let (db, base) = database::load(&input).unwrap();
    let mut players = db.players(base).unwrap();
    let prepared = players
        .iter_mut()
        .find(|player| player.uid == 2001642)
        .unwrap();
    prepared.fields.get_mut("state.jadedness").unwrap().value = -28;
    prepared.fields.get_mut("state.morale").unwrap().value = 20;
    let tired = players.iter_mut().find(|player| player.uid == 356).unwrap();
    tired.fields.get_mut("state.jadedness").unwrap().value = 21;
    let request = batch::Request {
        uids: vec![356, 2001642],
        attribute: String::new(),
        operation: batch::Operation::Recover,
    };
    let (edits, targets) = batch::project(&players, &request).unwrap();
    let for_player = |uid| {
        targets
            .iter()
            .filter(|target| target.uid == uid)
            .map(|target| (target.attribute.as_str(), target.after))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        for_player(356),
        [
            ("state.condition", 10000),
            ("state.fitness", 10000),
            ("state.jadedness", 0),
            ("state.morale", 18),
        ]
    );
    assert_eq!(
        for_player(2001642),
        [
            ("state.condition", 10000),
            ("state.fitness", 10000),
            ("state.jadedness", -28),
            ("state.morale", 20),
        ]
    );
    assert!(edits.iter().all(|edit| edit.uid == 356));
}

#[test]
fn replay_keeps_later_targets_without_reinstating_earlier_targets() {
    let original = transaction::Transaction::default();
    let field = |uid, attribute: &str, value| cm0304_core::editor::Edit {
        uid,
        attribute: attribute.into(),
        value,
        policy: None,
    };
    let mut earlier = original.clone();
    earlier.edits = vec![field(1, "passing", 15), field(1, "technique", 16)];
    earlier.heal_injuries.insert(1);
    let mut later = earlier.clone();
    later.edits[0].value = 19;
    later.edits.push(field(2, "state.morale", 20));
    later.club_balances.insert(42, 1_000);
    let mut replayed = original;
    replayed.replay_delta(&earlier, &later, |_| None, |_| None);
    assert_eq!(
        replayed.edits,
        [field(1, "passing", 19), field(2, "state.morale", 20)]
    );
    assert!(replayed.heal_injuries.is_empty());
    assert_eq!(replayed.club_balances[&42], 1_000);
}

#[test]
fn replay_keeps_staff_fields_independent_and_carries_policy_with_new_ca() {
    use cm0304_core::{editor::AbilityPolicy, staff_header::Edit};
    let mut earlier = transaction::Transaction::default();
    earlier.staff_header_edits.push(Edit {
        uid: 42,
        current_ability: Some(100),
        policy: Some(AbilityPolicy::PreserveValues),
        ..Default::default()
    });
    let mut later = earlier.clone();
    later.staff_header_edits[0].potential_ability = Some(150);
    let mut replayed = transaction::Transaction::default();
    replayed.replay_delta(&earlier, &later, |_| None, |_| None);
    let header = &replayed.staff_header_edits[0];
    assert_eq!(header.current_ability, None);
    assert_eq!(header.policy, None);
    assert_eq!(header.potential_ability, Some(150));
    let mut latest = later.clone();
    latest.staff_header_edits[0].current_ability = Some(125);
    replayed.replay_delta(&later, &latest, |_| None, |_| None);
    assert_eq!(replayed.staff_header_edits[0], latest.staff_header_edits[0]);
}

#[test]
fn replay_keeps_later_budget_and_reputation_components_only() {
    use cm0304_core::{staff_header::Edit, transaction::TransferBudgets};
    let budget_origin = |_| {
        Some(TransferBudgets {
            season: 1_000,
            remaining: 700,
        })
    };
    let reputation_origin = |_| Some([100, 200, 300]);
    let mut earlier = transaction::Transaction::default();
    earlier.club_transfer_budgets.insert(
        42,
        TransferBudgets {
            season: 2_000,
            remaining: 700,
        },
    );
    earlier.staff_header_edits.push(Edit {
        uid: 15,
        reputation: Some([900, 200, 300]),
        ..Default::default()
    });
    let mut later = earlier.clone();
    later.club_transfer_budgets.get_mut(&42).unwrap().remaining = 800;
    later.staff_header_edits[0].reputation = Some([900, 200, 400]);
    let mut replayed = transaction::Transaction::default();
    replayed.replay_delta(&earlier, &later, budget_origin, reputation_origin);
    assert_eq!(
        replayed.club_transfer_budgets[&42],
        TransferBudgets {
            season: 1_000,
            remaining: 800
        }
    );
    assert_eq!(
        replayed.staff_header_edits[0].reputation,
        Some([100, 200, 400])
    );
    replayed.replay_delta(
        &later,
        &transaction::Transaction::default(),
        budget_origin,
        reputation_origin,
    );
    assert!(replayed.is_empty());
}

#[test]
fn batch_edits_preserve_the_unselected_owner_and_report_contract_differences() {
    let source = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap();
    let players = {
        let (db, base) = database::load(&source).unwrap();
        db.players(base).unwrap()
    };
    let loan = || batch::ContractSelection {
        uid: 303210,
        index: 1,
    };
    let (mut edits, wage_targets) = batch::project_wages(
        &players,
        &batch::WageRequest {
            contracts: vec![loan()],
            operation: batch::Operation::Percent(20),
            ceiling: None,
        },
    )
    .unwrap();
    assert_eq!((wage_targets[0].before, wage_targets[0].after), (273, 328));
    let (expiry, _) = batch::project_expiries(
        &players,
        &batch::ExpiryRequest {
            contracts: vec![loan()],
            year: 2004,
            day: 59,
        },
    )
    .unwrap();
    edits.extend(expiry);
    let (statuses, targets) = batch::project_squad_status(
        &players,
        &batch::SquadStatusRequest {
            contracts: vec![loan()],
            status: 1,
        },
    )
    .unwrap();
    assert_eq!(targets[0].attribute, "contract.1.squadStatus");
    assert_eq!(targets[0].after, 1);
    edits.extend(statuses);
    drop(players);
    let plan = transaction::Transaction {
        edits,
        ..Default::default()
    };
    let (output, _) = transaction::apply(&source, &plan).unwrap();
    let checks = std::cell::Cell::new(0);
    let cancelled = roster_comparison::compare_with_cancel(&source, &output, 104750, || {
        let seen = checks.get();
        checks.set(seen + 1);
        seen >= 3
    });
    assert_eq!(cancelled.err().as_deref(), Some("Comparação cancelada"));
    let comparison =
        to_value(roster_comparison::compare(&source, &output, 104750).unwrap()).unwrap();
    assert_eq!(comparison["changed"].as_array().unwrap().len(), 1);
    let changed = &comparison["changed"][0];
    assert_eq!(changed["uid"], 303210);
    assert_eq!(changed["contractsBefore"][0], changed["contractsAfter"][0]);
    assert!(changed["contractsBefore"][0]["entityUid"].is_number());
    assert_eq!(changed["contractsBefore"][1]["entityUid"], 104750);
    assert_eq!(changed["contractsAfter"][1]["entityUid"], 104750);
    assert_ne!(
        changed["contractsBefore"][0]["entityUid"],
        changed["contractsBefore"][1]["entityUid"]
    );
    assert_eq!(changed["contractsBefore"][1]["weeklyWageGbp"], 273);
    assert_eq!(changed["contractsAfter"][1]["weeklyWageGbp"], 328);
    assert_eq!(changed["contractsAfter"][1]["squadStatus"], 1);
    assert_eq!(
        changed["contractsAfter"][1]["expires"],
        serde_json::json!([2004, 59])
    );
}
#[test]
fn roster_comparison_identifies_an_invalid_input() {
    use cm0304_core::roster_comparison::compare;

    let error = compare(b"invalid", b"invalid", 0)
        .err()
        .expect("invalid save");
    assert!(error.starts_with("Save de origem:"));
    assert!(error.contains(".cm4 sem compactação do CM0304 4.1.5"));
}
