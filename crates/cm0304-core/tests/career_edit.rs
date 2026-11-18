use cm0304_core::{
    database,
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
        e(5101855, "contract.1.weeklyWage", 500),
    ];
    let (output, report) = editor::apply(&input, &plan).unwrap();
    let (db, base) = database::load(&output).unwrap();
    let players = db.players(base).unwrap();
    for edit in &plan {
        assert_eq!(
            players.iter().find(|p| p.uid == edit.uid).unwrap().fields[&edit.attribute].value,
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
