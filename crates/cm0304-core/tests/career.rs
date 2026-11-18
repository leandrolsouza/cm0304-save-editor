use cm0304_core::database;
use std::{fs, path::PathBuf};

#[test]
fn reads_existing_contracts_state_and_loan_entities_without_changing_layout() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../game/user data/games/save_inicial.cm4");
    let bytes = fs::read(path).unwrap();
    let (db, base) = database::load(&bytes).unwrap();
    assert_eq!(db.players(base).unwrap().len(), 22062);
    let player = |uid| db.people.iter().find(|p| p.uid == uid).unwrap();
    let clodoaldo = player(2001642);
    let info = clodoaldo.playing.as_ref().unwrap();
    assert_eq!(info.state.condition, 10000);
    assert_eq!(info.state.fitness, 10000);
    assert_eq!(info.state.jadedness, 0);
    assert_eq!(info.state.morale, 10); // "OK" in game
    let contract = &clodoaldo.contracts[0];
    assert_eq!(contract.kind, 0);
    assert_eq!(contract.weekly_wage, 1490);
    assert_eq!(
        (contract.expires.year, contract.expires.month_day()),
        (2005, Some((1, 1)))
    );
    assert_eq!(
        (contract.started.year, contract.started.month_day()),
        (1999, Some((7, 1)))
    );
    assert_eq!(
        db.clubs[contract.entity_index as usize].short_name,
        "Fortaleza"
    );
    assert_eq!(contract.squad_status, 4); // backup player in game
    let absolute = base + contract.common_offset;
    assert_eq!(
        i32::from_le_bytes(bytes[absolute + 5..absolute + 9].try_into().unwrap()),
        1490
    );
    let quim = player(1855);
    assert_eq!(quim.playing.as_ref().unwrap().state.condition, 9351);
    assert_eq!(quim.contracts[0].squad_number, 1);
    let loan = player(5101855); // Liam Chilvers, owner and loan club differ
    assert_eq!(loan.contracts.len(), 2);
    assert_eq!(loan.contracts[0].kind, 0);
    assert_eq!(loan.contracts[1].kind, 1);
    assert_eq!(
        db.clubs[loan.contracts[0].entity_index as usize].name,
        "Arsenal"
    );
    assert_eq!(
        db.clubs[loan.contracts[1].entity_index as usize].name,
        "Northampton Town"
    );
}
