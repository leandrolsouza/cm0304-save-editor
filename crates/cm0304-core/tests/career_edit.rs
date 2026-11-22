use cm0304_core::{
    career, database,
    editor::{self, Edit},
};
use std::{fs, path::PathBuf};
fn sample() -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap()
}
fn e(uid: i32, key: &str, value: i32) -> Edit {
    Edit {
        uid,
        attribute: key.into(),
        value,
        policy: None,
    }
}

#[test]
fn synthetic_national_contract_keeps_its_entity_during_a_salary_edit() {
    let mut input = sample();
    let uid = 2001642;
    let (db, base) = database::load(&input).unwrap();
    let person = db.people.iter().find(|person| person.uid == uid).unwrap();
    let offset = base + person.contracts[0].common_offset;
    let nation_index = db
        .nations
        .iter()
        .position(|nation| nation.name == "Brasil")
        .unwrap();
    let nation_index = i32::try_from(nation_index).unwrap();

    // No local career contains this contract type. Change only the fixture's entity.
    input[offset] = 1;
    input[offset + 1..offset + 5].copy_from_slice(&nation_index.to_le_bytes());
    let (output, _) = editor::apply(&input, &[e(uid, "contract.0.weeklyWage", 2000)]).unwrap();
    let (db, _) = database::load(&output).unwrap();
    let person = db.people.iter().find(|person| person.uid == uid).unwrap();
    let contract = &person.contracts[0];
    assert_eq!(
        (contract.entity_type, contract.entity_index),
        (1, nation_index)
    );
    assert_eq!(contract.weekly_wage, 2000);
    assert_eq!(
        career::contracts(&db, person)[0].entity.as_deref(),
        Some("Brasil")
    );
    assert_eq!(&input[..offset + 5], &output[..offset + 5]);
    assert_eq!(&input[offset + 9..], &output[offset + 9..]);
}

#[test]
fn career_transaction_preserves_entities_and_other_contracts() {
    let input = sample();
    let plan = vec![
        e(2001642, "state.condition", 7000),
        e(2001642, "state.fitness", 8000),
        e(2001642, "state.jadedness", -1000),
        e(2001642, "state.morale", 20),
        e(2001642, "state.estimatedValue", 2_000_000),
        e(2001642, "state.saleValue", 3_000_000),
        e(2001642, "contract.0.weeklyWage", 2500),
        e(2001642, "contract.0.expires.year", 2004),
        e(2001642, "contract.0.expires.day", 0),
        e(2001642, "contract.0.squadStatus", 1),
        e(2001642, "contract.0.squadNumber", 99),
        e(2001642, "contract.0.goalBonus", 600),
        e(2001642, "contract.0.appearanceBonus", 900),
        e(2001642, "contract.0.assistBonus", 300),
        e(18, "contract.0.releaseClause", 800_000),
        e(5101855, "contract.1.weeklyWage", 500),
    ];
    let (output, report) = editor::apply(&input, &plan).unwrap();
    let (db, base) = database::load(&output).unwrap();
    let players = db.players(base).unwrap();
    for edit in &plan {
        assert_eq!(
            players.iter().find(|p| p.uid == edit.uid).unwrap().fields[edit.attribute.as_str()]
                .value,
            edit.value
        );
    }
    let before = database::load(&input).unwrap().0;
    let loan = db.people.iter().find(|p| p.uid == 5101855).unwrap();
    let old = before.people.iter().find(|p| p.uid == 5101855).unwrap();
    assert_eq!(loan.team, old.team);
    assert_eq!(loan.contracts.len(), old.contracts.len());
    for (a, b) in loan.contracts.iter().zip(&old.contracts) {
        assert_eq!(
            (a.kind, a.entity_type, a.entity_index),
            (b.kind, b.entity_type, b.entity_index)
        );
    }
    assert_eq!(loan.contracts[0].weekly_wage, old.contracts[0].weekly_wage);
    assert!(
        report
            .changes
            .iter()
            .any(|c| c.after == -1000 && c.raw_after == -1000)
    );
    assert!(report.changes.iter().any(|c| c.width == 4));
    assert_eq!(input.len(), output.len());
    let allowed = report
        .changes
        .iter()
        .flat_map(|c| c.offset..c.offset + c.width as usize)
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        input
            .iter()
            .zip(&output)
            .enumerate()
            .all(|(i, (a, b))| a == b || allowed.contains(&i))
    );
}
#[test]
fn career_rejects_invalid_dates_ranges_and_loan_relationships() {
    let input = sample();
    for plan in [
        vec![e(2001642, "state.condition", 10001)],
        vec![e(2001642, "state.morale", 21)],
        vec![e(2001642, "contract.99.weeklyWage", 100)],
        vec![e(2001642, "contract.0.weeklyWage", -1)],
        vec![e(2001642, "contract.0.releaseClause", 100_000)],
        vec![e(18, "contract.0.releaseClause", 0)],
        vec![e(5101855, "contract.1.appearanceBonus", 900)],
        vec![
            e(2001642, "contract.0.expires.year", 2003),
            e(2001642, "contract.0.expires.day", 365),
        ],
        vec![e(2001642, "contract.0.expires.year", 2002)],
        vec![e(5101855, "contract.1.expires.year", 2200)],
        vec![
            e(5101855, "contract.1.expires.year", 1900),
            e(5101855, "contract.1.expires.day", 1),
        ],
    ] {
        assert!(editor::apply(&input, &plan).is_err());
    }
    let (out, _) = editor::apply(
        &input,
        &[
            e(2001642, "contract.0.expires.year", 1900),
            e(2001642, "contract.0.expires.day", 1),
        ],
    )
    .unwrap();
    let (db, _) = database::load(&out).unwrap();
    assert!(cm0304_core::career::is_monthly(
        &db.people
            .iter()
            .find(|p| p.uid == 2001642)
            .unwrap()
            .contracts[0]
            .expires
    ));
}
#[test]
fn squad_numbers_consider_the_whole_transaction() {
    let input = sample();
    let (db, _) = database::load(&input).unwrap();
    let a = db.people.iter().find(|p| p.uid == 1855).unwrap();
    let ac = &a.contracts[0];
    let b = db
        .people
        .iter()
        .find(|p| {
            p.uid != a.uid
                && p.playing.is_some()
                && p.team == a.team
                && p.contracts.first().is_some_and(|c| {
                    c.entity_type == ac.entity_type
                        && c.entity_index == ac.entity_index
                        && c.squad_number > 0
                        && c.squad_number != ac.squad_number
                })
        })
        .unwrap();
    let bc = &b.contracts[0];
    assert!(
        editor::apply(
            &input,
            &[e(a.uid, "contract.0.squadNumber", bc.squad_number as i32)]
        )
        .is_err()
    );
    let (out, _) = editor::apply(
        &input,
        &[
            e(a.uid, "contract.0.squadNumber", bc.squad_number as i32),
            e(b.uid, "contract.0.squadNumber", ac.squad_number as i32),
        ],
    )
    .unwrap();
    let (after, _) = database::load(&out).unwrap();
    assert_eq!(
        after
            .people
            .iter()
            .find(|p| p.uid == a.uid)
            .unwrap()
            .contracts[0]
            .squad_number,
        bc.squad_number
    );
}

#[test]
fn porto_shirt_swap_preserves_european_squad_entries() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../game/user data/games/12.cm4"),
    )
    .unwrap();
    let (output, _) = editor::apply(
        &input,
        &[
            e(1106, "contract.0.squadNumber", 30),
            e(1597, "contract.0.squadNumber", 99),
        ],
    )
    .unwrap();
    let (original, _) = database::load(&input).unwrap();
    let (edited, _) = database::load(&output).unwrap();
    let european_entry = [1, 25, 0, 0, 213, 2, 0, 0];
    for (uid, number, registered) in [(1106, 30, true), (1597, 99, false)] {
        let before = original
            .people
            .iter()
            .find(|person| person.uid == uid)
            .unwrap();
        let after = edited
            .people
            .iter()
            .find(|person| person.uid == uid)
            .unwrap();
        assert_eq!(after.contracts[0].squad_number, number);
        assert_eq!(after.squad_entries_raw, before.squad_entries_raw);
        assert_eq!(
            after.squad_entries_raw.contains(&european_entry),
            registered
        );
    }
    assert_eq!(
        cm0304_core::container::hash(&output),
        "07df9c6d5cb4190463818963fa3786536ac39bdee786b18905f803c4f05f9f33"
    );
}

#[test]
fn braga_shirt_swap_preserves_the_loan_owners_number() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_modificado.cm4"),
    )
    .unwrap();
    let quim = e(1855, "contract.0.squadNumber", 41);
    let fredrik = e(1873, "contract.1.squadNumber", 1);
    assert!(editor::apply(&input, std::slice::from_ref(&quim)).is_err());
    assert!(editor::apply(&input, std::slice::from_ref(&fredrik)).is_err());

    let (before, _) = database::load(&input).unwrap();
    let loan_player = before.people.iter().find(|p| p.uid == 1873).unwrap();
    let owner = &loan_player.contracts[0];
    let occupied_at_porto = before
        .people
        .iter()
        .filter(|p| p.uid != loan_player.uid && p.team != loan_player.team)
        .flat_map(|p| &p.contracts)
        .find(|c| {
            c.entity_type == owner.entity_type
                && c.entity_index == owner.entity_index
                && c.squad_number > 0
                && c.squad_number != owner.squad_number
        })
        .unwrap();
    let conflict = e(
        1873,
        "contract.0.squadNumber",
        occupied_at_porto.squad_number.into(),
    );
    assert!(
        editor::apply(&input, &[conflict])
            .unwrap_err()
            .contains("já usada")
    );

    let (output, _) = editor::apply(&input, &[quim, fredrik]).unwrap();
    let (db, _) = database::load(&output).unwrap();
    let quim = db.people.iter().find(|p| p.uid == 1855).unwrap();
    let fredrik = db.people.iter().find(|p| p.uid == 1873).unwrap();
    assert_eq!(quim.contracts[0].squad_number, 41);
    assert_eq!(fredrik.contracts[1].squad_number, 1);
    assert_eq!(fredrik.contracts[0].squad_number, 24);
    assert_eq!(input.len(), output.len());
    assert_eq!(input.iter().zip(&output).filter(|(a, b)| a != b).count(), 2);
}
