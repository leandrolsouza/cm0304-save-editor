use cm0304_core::database;
use std::{fs, path::PathBuf};

#[test]
fn health_record_spans_match_source_bytes_in_both_samples() {
    for sample in ["save_inicial.cm4", "save_modificado.cm4"] {
        let bytes = fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../game/user data/games")
                .join(sample),
        )
        .unwrap();
        let (db, base) = database::load(&bytes).unwrap();
        let mut counts = [0usize; 4];
        for person in &db.people {
            let Some(info) = &person.playing else {
                continue;
            };
            let health = &info.health_records;
            if let Some(offset) = health.injury_count_offset {
                assert_eq!(bytes[base + offset] as usize, health.injuries.len());
                let ban_offset = health.ban_count_offset.unwrap();
                assert_eq!(bytes[base + ban_offset] as usize, health.bans.len());
                assert_eq!(
                    bytes[base + health.injury_status_flag_offset.unwrap()] != 0,
                    health.injury_status.is_some()
                );
                let injury_end = health
                    .injuries
                    .last()
                    .map(|record| record.offset + record.bytes.len())
                    .unwrap_or(offset + 1);
                assert_eq!(injury_end, ban_offset);
            }
            for (kind, records) in [
                health.injuries.as_slice(),
                health.bans.as_slice(),
                health.injury_status.as_slice(),
                health.bookings.as_slice(),
            ]
            .into_iter()
            .enumerate()
            {
                counts[kind] += records.len();
                for record in records {
                    assert!(record.offset > info.offset);
                    let start = base + record.offset;
                    assert_eq!(record.bytes, bytes[start..start + record.bytes.len()]);
                    match kind {
                        0 => {
                            assert!(matches!(record.bytes.len(), 38 | 40 | 42 | 44));
                            let injury_index =
                                i32::from_le_bytes(record.bytes[..4].try_into().unwrap());
                            assert!(injury_index >= 0);
                            assert!(db.injury_types.get(injury_index as usize).is_some());
                        }
                        1 => assert!(matches!(record.bytes.len(), 19 | 58)),
                        2 => assert_eq!(record.bytes.len(), 26),
                        3 => assert_eq!(
                            record.bytes.len(),
                            1 + [3, 5, 5, 8, 4][record.bytes[0] as usize]
                        ),
                        _ => unreachable!(),
                    }
                }
            }
        }
        assert!(
            counts.iter().all(|count| *count > 0),
            "{sample}: {counts:?}"
        );
    }
}

#[test]
fn reads_existing_contracts_state_and_loan_entities_without_changing_layout() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../game/user data/games/save_inicial.cm4");
    let bytes = fs::read(path).unwrap();
    let (db, base) = database::load(&bytes).unwrap();
    assert_eq!(db.injury_types.len(), 83);
    let ankle = &db.injury_types[11];
    assert_eq!(ankle.name, "entorse dos ligamentos do tornozelo");
    assert_eq!((ankle.id, ankle.uid), (11, 12));
    assert_eq!((ankle.minimum_days_out, ankle.extra_days_out), (15, 30));
    let hoekstra =
        cm0304_core::health::injuries(&db, db.people.iter().find(|p| p.uid == 356).unwrap());
    assert_eq!(hoekstra.len(), 1);
    assert_eq!(hoekstra[0].name, "entorse dos ligamentos do tornozelo");
    let recorded = hoekstra[0].recorded_on.as_ref().unwrap();
    assert_eq!((recorded.year, recorded.month_day()), (2003, Some((7, 13))));
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
