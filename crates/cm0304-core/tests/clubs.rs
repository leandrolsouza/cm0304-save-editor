use cm0304_core::transaction::{self, StadiumCapacities, Transaction};
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
fn game_rewrite_preserves_the_free_transfer_history_row() {
    use cm0304_core::{container::read_container, transfer_lab::inspect_history};

    let squad_hash = |bytes: &[u8]| {
        read_container(bytes)
            .unwrap()
            .blocks
            .into_iter()
            .find(|block| block.name == "squad_man" && block.extension == ".dat")
            .unwrap()
            .sha256
    };
    let source_squad = squad_hash(&sample("pgtoeditfim.cm4"));
    for name in ["0angelimhist.cm4", "4.cm4"] {
        let bytes = sample(name);
        assert_eq!(squad_hash(&bytes), source_squad, "{name}");
        let rows = inspect_history(&bytes).unwrap();
        let mut matches = rows.iter().filter(|row| row.person_id == 8992);
        let row = matches.next().expect("Expected the transfer for Angelim");
        assert!(matches.next().is_none(), "{name}");
        assert_eq!(
            (row.source_team_index, row.destination_team_index),
            (1397, 1396)
        );
        assert_eq!((row.year, row.day, row.fee_raw), (2004, 164, 0));
        assert_eq!((row.status_raw, row.kind_raw), (1, 1));
        let (db, _) = database::load(&bytes).unwrap();
        let person = &db.people[8992];
        assert_eq!((person.uid, person.team), (311399, 1396));
    }
}

#[test]
fn transfer_lab_reproduces_the_game_tested_free_transfer_with_history() {
    use cm0304_core::transfer_lab::{inspect_history, save_with_options};

    let original = sample("pgtoeditfim.cm4");
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let copy = directory.path().join("copy.cm4");
    let without_history = directory.path().join("without-history.cm4");
    fs::write(&source, &original).unwrap();

    let report = save_with_options(&source, &copy, 311399, 104749, true).unwrap();
    let report = serde_json::to_value(report).unwrap();
    for (team, before, after, recorded) in [
        ("sourceTeam", 15_379, 14_753, 15_379),
        ("destinationTeam", 11_020, 11_646, 10_469),
    ] {
        assert_eq!(report[team]["clubFullPlayerWagesBefore"], before);
        assert_eq!(report[team]["clubFullPlayerWagesAfter"], after);
        assert_eq!(report[team]["clubRecordedWagesBefore"], recorded);
        assert_eq!(report[team]["clubRecordedWagesAfter"], recorded);
    }
    save_with_options(&source, &without_history, 311399, 104749, false).unwrap();

    assert_eq!(fs::read(&source).unwrap(), original);
    assert_eq!(fs::read(&copy).unwrap(), sample("0angelimhist.cm4"));
    assert_eq!(
        inspect_history(&fs::read(without_history).unwrap()).unwrap(),
        inspect_history(&original).unwrap()
    );
}

#[test]
fn transfer_with_history_refuses_a_pending_market_edit_for_the_player() {
    use cm0304_core::{container::read_container, market_lab, transfer_lab};

    let original = sample("pgtoeditfim.cm4");
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let pending = directory.path().join("pending.cm4");
    let copy = directory.path().join("copy.cm4");
    fs::write(&source, &original).unwrap();
    market_lab::synthesize(&source, &pending, 311399, 68).unwrap();
    let pending_bytes = fs::read(&pending).unwrap();

    let preview = transfer_lab::preview_with_options(&pending_bytes, 311399, 104749, true);
    assert!(preview.err().unwrap().contains("edição de mercado"));
    let result = transfer_lab::save_with_options(&pending, &copy, 311399, 104749, true);
    assert!(result.err().unwrap().contains("edição de mercado"));
    assert!(!copy.exists());
    assert_eq!(fs::read(&pending).unwrap(), pending_bytes);

    let unrelated = directory.path().join("unrelated.cm4");
    market_lab::synthesize(&source, &unrelated, 2001642, 68).unwrap();
    let unrelated_bytes = fs::read(&unrelated).unwrap();
    transfer_lab::preview_with_options(&unrelated_bytes, 311399, 104749, true).unwrap();
    transfer_lab::save_with_options(&unrelated, &copy, 311399, 104749, true).unwrap();
    let journal_hash = |bytes: &[u8]| {
        read_container(bytes)
            .unwrap()
            .blocks
            .into_iter()
            .find(|block| block.name == "edit_session_man" && block.extension == ".dat")
            .unwrap()
            .sha256
    };
    assert_eq!(
        journal_hash(&fs::read(copy).unwrap()),
        journal_hash(&unrelated_bytes)
    );
}

#[test]
fn transfer_with_history_refuses_an_unrecognized_operations_journal() {
    use cm0304_core::{container, transfer_lab};

    let original = sample("pgtoeditfim.cm4");
    let unknown =
        container::replace_block(&original, "edit_session_man", ".dat", &[0; 13]).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let copy = directory.path().join("copy.cm4");
    fs::write(&source, &unknown).unwrap();

    let result = transfer_lab::save_with_options(&source, &copy, 311399, 104749, true);
    assert!(result.err().unwrap().contains("Diário de operações"));
    assert!(!copy.exists());
    assert_eq!(fs::read(&source).unwrap(), unknown);
}

#[test]
fn buyer_club_contains_sell_on_clause_only_after_completed_transfer() {
    for (name, buyer, player, expected) in [
        ("3.cm4", 734132, 736046, false),
        ("h.cm4", 734132, 736046, true),
        ("save_modificado.cm4", 744319, 728613, true),
    ] {
        let (db, _) = database::load(&sample(name)).unwrap();
        let buyer = db.clubs.iter().find(|club| club.uid == buyer).unwrap();
        let clauses = &buyer.details.as_ref().unwrap().sell_on_clauses;
        let clause = clauses.iter().find(|clause| clause.player_uid == player);
        assert_eq!(clause.is_some(), expected, "{name}");
        if let Some(clause) = clause {
            assert_eq!(db.clubs[clause.beneficiary_club_index as usize].uid, 1488);
            assert_eq!(clause.percent, 20);
            assert_eq!(clause.trailing_byte, 0x2f);
        }
    }
}

#[test]
fn transfer_lab_refuses_to_leave_an_active_sell_on_clause_unprocessed() {
    let input = sample("h.cm4");
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let copy = directory.path().join("copy.cm4");
    fs::write(&source, &input).unwrap();

    let result = cm0304_core::transfer_lab::save(&source, &copy, 736046, 744319);
    assert!(result.err().unwrap().contains("cláusula de revenda"));
    assert!(!copy.exists());
    assert_eq!(fs::read(&source).unwrap(), input);
}

#[test]
fn accepted_offer_reader_checks_both_copies_of_terms() {
    use cm0304_core::{container, transfer_lab::inspect_accepted_offers};

    assert!(
        inspect_accepted_offers(&sample("v1.cm4"))
            .unwrap()
            .is_empty()
    );
    let accepted = sample("3.cm4");
    let offers = inspect_accepted_offers(&accepted).unwrap();
    let [rui, quinio] = offers.as_slice() else {
        panic!("Expected the two accepted offers in order");
    };
    assert_eq!((rui.person_id, rui.destination_team_index), (8927, 2726));
    assert_eq!(
        (
            rui.upfront_raw,
            rui.installments_raw,
            rui.installment_months,
            rui.sell_on_percent
        ),
        (3967, 11903, 24, 20)
    );
    assert_eq!(
        (quinio.person_id, quinio.destination_team_index),
        (10496, 2689)
    );
    assert_eq!(
        (
            quinio.upfront_raw,
            quinio.installments_raw,
            quinio.installment_months,
            quinio.sell_on_percent
        ),
        (1985, 5943, 18, 20)
    );

    let journal = container::read_container(&accepted)
        .unwrap()
        .blocks
        .into_iter()
        .find(|block| block.name == "edit_session_man")
        .unwrap();
    let mut corrupt = accepted;
    // Change only the repeated copy of the second offer's installment amount.
    corrupt[journal.payload_offset + 635] ^= 1;
    assert!(inspect_accepted_offers(&corrupt).is_err());
    assert!(inspect_accepted_offers(&sample("0quiniotauri.cm4")).is_err());
}

#[test]
fn transfer_lab_refuses_players_with_an_observed_accepted_offer() {
    use cm0304_core::transfer_lab::{preview_with_options, save_with_options};

    let input = sample("3.cm4");
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let copy = directory.path().join("copy.cm4");
    fs::write(&source, &input).unwrap();

    for (uid, destination) in [(728613, 744319), (736046, 734132)] {
        for with_history in [false, true] {
            assert!(
                preview_with_options(&input, uid, destination, with_history)
                    .err()
                    .unwrap()
                    .contains("proposta aceita")
            );
            assert!(
                save_with_options(&source, &copy, uid, destination, with_history)
                    .err()
                    .unwrap()
                    .contains("proposta aceita")
            );
        }
    }
    assert!(!copy.exists());
    assert_eq!(fs::read(&source).unwrap(), input);
}

#[test]
fn transfer_lab_preserves_players_with_a_club_squad_entry() {
    use cm0304_core::transfer_lab::{preview, save};

    let before = sample("1.cm4");
    let registered = sample("11.cm4");
    let match_day = sample("12.cm4");
    let player_uid = 4102044;
    let destination_uid = 1488;
    for (input, expected) in [
        (&before, None),
        (&registered, Some(26)),
        (&match_day, Some(25)),
    ] {
        let (db, _) = database::load(input).unwrap();
        let player = db.people.iter().find(|p| p.uid == player_uid).unwrap();
        let team = &db.teams[player.team as usize];
        let club = &db.clubs[team.container_index as usize];
        assert_eq!(club.uid, 1478);
        assert_eq!(team.container_index, 725);
        assert_ne!(player.team, team.container_index);
        assert_eq!(
            player.squad_entries_raw.as_slice(),
            expected
                .map(|value| [1, value, 0, 0, 213, 2, 0, 0])
                .as_slice()
        );
    }

    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let copy = directory.path().join("copy.cm4");
    fs::write(&source, &registered).unwrap();
    let preview_error = preview(&registered, player_uid, destination_uid)
        .err()
        .unwrap();
    let save_error = save(&source, &copy, player_uid, destination_uid)
        .err()
        .unwrap();
    assert!(preview_error.contains("registros de elenco associados ao clube atual"));
    assert_eq!(preview_error, save_error);
    assert!(!copy.exists());
    assert_eq!(fs::read(&source).unwrap(), registered);
    let match_day_error = preview(&match_day, player_uid, destination_uid)
        .err()
        .unwrap();
    assert_eq!(match_day_error, preview_error);
}

#[test]
fn transfer_installment_count_drops_between_monthly_checkpoints() {
    use cm0304_core::transfer_lab::inspect_references;
    use std::collections::BTreeMap;

    let before = inspect_references(&sample("matrizbmistor.cm4")).unwrap();
    let after = inspect_references(&sample("x.cm4")).unwrap();
    let key = |row: &cm0304_core::transfer_lab::TransferReference| {
        (
            row.person_id,
            row.source_club_index,
            row.destination_club_index,
            row.installment_amount_raw,
        )
    };
    let before: BTreeMap<_, _> = before
        .iter()
        .map(|row| (key(row), row.installments_remaining))
        .collect();
    let mut retained = 0;
    for row in &after {
        if let Some(&remaining) = before.get(&key(row)) {
            assert_eq!(row.installments_remaining.checked_add(1), Some(remaining));
            retained += 1;
        }
    }
    assert_eq!(retained, 122);
}

#[test]
fn transfer_references_read_variable_footers_and_reject_invalid_counts() {
    use cm0304_core::{container, transfer_lab::inspect_references};

    assert!(
        inspect_references(&sample("save_inicial.cm4"))
            .unwrap()
            .is_empty()
    );
    let portugal = sample("save_modificado.cm4");
    let rows = inspect_references(&portugal).unwrap();
    assert_eq!(rows.len(), 90);
    assert!(rows.iter().any(|row| {
        row.person_id == 8927
            && row.source_club_index == 734
            && row.destination_club_index == 2445
            && row.payload_offset == 166316
    }));

    let brazil = sample("angelimceara12d.cm4");
    let rows = inspect_references(&brazil).unwrap();
    assert_eq!(rows.len(), 716);
    assert!(rows.iter().all(|row| row.person_id != 8992));

    let transfer_block = container::read_container(&brazil)
        .unwrap()
        .blocks
        .into_iter()
        .find(|block| block.name == "transfer_man")
        .unwrap();
    let table_end = transfer_block.payload_offset + rows.last().unwrap().payload_offset + 17;
    assert_eq!(&brazil[table_end..table_end + 4], &55_u32.to_le_bytes());
    let mut corrupt = brazil;
    corrupt[table_end..table_end + 4].copy_from_slice(&54_u32.to_le_bytes());
    assert!(inspect_references(&corrupt).is_err());
}

#[test]
fn transfer_lab_moves_contract_and_roster_together_and_rejects_loans() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let first = directory.path().join("pampilhosa.cm4");
    let second = directory.path().join("tabuense.cm4");
    let input = sample("save_inicial.cm4");
    fs::write(&source, &input).unwrap();

    let save = cm0304_core::transfer_lab::save;
    assert!(save(&source, &first, 303210, 744319).is_err());
    assert!(!first.exists());
    let preview = cm0304_core::transfer_lab::preview(&input, 728613, 744319).unwrap();
    assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    let report = serde_json::to_value(save(&source, &first, 728613, 744319).unwrap()).unwrap();
    assert_eq!(serde_json::to_value(preview).unwrap(), report);
    assert_eq!(report["sourceTeam"]["teamIndex"], 3828);
    assert_eq!(report["destinationTeam"]["teamIndex"], 2726);
    assert_eq!(report["contractTerms"]["weeklyWageGbp"], 111);
    assert_eq!(report["contractTerms"]["transferStatus"], 5);
    assert_eq!(report["contractTerms"]["appearanceBonusGbp"], 50);
    for (side, before, after, delta) in [
        ("sourceTeam", 33, 32, -111),
        ("destinationTeam", 16, 17, 111),
    ] {
        let impact = &report[side];
        assert_eq!(impact["rosterBefore"], before);
        assert_eq!(impact["rosterAfter"], after);
        assert_eq!(
            impact["clubFullPlayerWagesAfter"].as_i64().unwrap()
                - impact["clubFullPlayerWagesBefore"].as_i64().unwrap(),
            delta
        );
    }
    let first_bytes = fs::read(&first).unwrap();
    assert_eq!(
        cm0304_core::container::hash(&first_bytes),
        "0e2f0a59ff490672773e3f9b71e0a03b945e729b00490e8c2c28ca1c18a23194"
    );
    assert!(save(&source, &first, 728613, 744319).is_err());

    save(&first, &second, 728613, 744320).unwrap();
    let output = fs::read(&second).unwrap();
    assert_eq!(output.len(), input.len());
    let (db, _) = database::load(&output).unwrap();
    let person = db.people.iter().find(|p| p.uid == 728613).unwrap();
    assert_eq!(person.team, 2727);
    let [contract] = person.contracts.as_slice() else {
        panic!("Expected one club contract")
    };
    assert_eq!(db.clubs[contract.entity_index as usize].uid, 744320);
    assert_eq!(contract.weekly_wage, 111);
    assert_eq!(db.teams[2727].roster_person_indices.last(), Some(&8927));
    assert!(!db.teams[2726].roster_person_indices.contains(&8927));
    assert_eq!(fs::read(&source).unwrap(), input);
    assert_eq!(fs::read(&first).unwrap(), first_bytes);
}

#[test]
fn transfer_lab_rejects_duplicate_shirt_numbers_at_destination() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.cm4");
    let copy = directory.path().join("copy.cm4");
    let mut input = sample("save_inicial.cm4");
    let (db, base) = database::load(&input).unwrap();
    let source_contract = &db
        .people
        .iter()
        .find(|p| p.uid == 728613)
        .unwrap()
        .contracts[0];
    let destination_index = db.clubs.iter().position(|c| c.uid == 744319).unwrap();
    let destination_contract = db
        .people
        .iter()
        .flat_map(|p| &p.contracts)
        .find(|c| c.entity_type == 0 && c.entity_index as usize == destination_index)
        .unwrap();
    for contract in [source_contract, destination_contract] {
        input[base + contract.common_offset + 30] = 99;
    }
    fs::write(&source, &input).unwrap();

    let result = cm0304_core::transfer_lab::save(&source, &copy, 728613, 744319);
    assert!(result.err().unwrap().contains("Camisa 99 já usada"));
    assert!(!copy.exists());
    assert_eq!(fs::read(&source).unwrap(), input);
}

#[test]
fn reads_both_team_rosters_around_a_real_transfer() {
    for (name, source_count, destination_count, expected_team) in [
        ("save_inicial.cm4", 33, 16, 3828),
        ("save_modificado.cm4", 29, 24, 2726),
    ] {
        let input = sample(name);
        let (db, base) = database::load(&input).unwrap();
        let person = db.people.iter().find(|p| p.uid == 728613).unwrap();
        assert_eq!(person.index, 8927);
        assert_eq!(person.team, expected_team);
        for (index, count) in [(3828, source_count), (2726, destination_count)] {
            let team = &db.teams[index];
            assert_eq!(team.roster_person_indices.len(), count);
            assert_eq!(
                team.roster_person_indices.contains(&8927),
                index as i32 == expected_team
            );
            let offset = base + team.roster_offset;
            assert_eq!(&input[offset..offset + 2], &(count as u16).to_le_bytes());
        }
    }
}

#[test]
fn resolves_total_and_seated_capacity_and_reports_shared_first_team_grounds() {
    let (db, _) = database::load(&sample("save_inicial.cm4")).unwrap();
    let stadiums = clubs::stadiums_by_club(&db);
    let braga = &stadiums[&1488];
    assert_eq!(braga.stadium.uid, 1461);
    assert_eq!(
        (braga.stadium.capacity, braga.stadium.seated_capacity),
        (28_000, 28_000)
    );
    assert_eq!(braga.first_team_club_uids, [1488]);

    for uid in [61, 69, 490410] {
        let shared = &stadiums[&uid];
        assert_eq!(shared.stadium.uid, 8502565);
        assert_eq!(shared.first_team_club_uids, [61, 69, 490410]);
    }
}

#[test]
fn stadium_edit_is_atomic_and_isolates_both_capacity_fields() {
    let input = sample("save_inicial.cm4");
    let (db, base) = database::load(&input).unwrap();
    let offset = base + clubs::stadiums_by_club(&db)[&1488].stadium.record_offset;
    let capacity = StadiumCapacities {
        capacity: 31_234,
        seated_capacity: 24_567,
    };
    let mut plan = Transaction::default();
    plan.club_stadium_capacities.insert(1488, capacity);
    let (output, report) = transaction::apply(&input, &plan).unwrap();
    assert_eq!(report.field_changed_bytes, 4);
    assert_eq!(report.changes.len(), 2);
    assert_eq!(&input[..offset + 1], &output[..offset + 1]);
    assert_eq!(&input[offset + 9..], &output[offset + 9..]);
    let (updated, _) = database::load(&output).unwrap();
    let stadium = &clubs::stadiums_by_club(&updated)[&1488].stadium;
    assert_eq!(stadium.capacity, capacity.capacity);
    assert_eq!(stadium.seated_capacity, capacity.seated_capacity);

    plan.club_stadium_capacities.insert(61, capacity);
    assert!(
        transaction::apply(&input, &plan)
            .unwrap_err()
            .contains("compartilhado")
    );
    plan.club_stadium_capacities.remove(&61);
    plan.club_stadium_capacities
        .get_mut(&1488)
        .unwrap()
        .capacity = 1;
    assert!(
        transaction::apply(&input, &plan)
            .unwrap_err()
            .contains("lugares sentados")
    );
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
            assert_eq!(d.payroll_expenses.is_some(), d.finance_kind == 1);
            assert_eq!(d.finance_summary.is_some(), d.finance_kind == 1);
            assert_eq!(d.current_wages_weekly.is_some(), d.finance_kind == 1);
            assert_eq!(d.wage_budget_weekly.is_some(), d.finance_kind == 1);
            if name == "save_modificado.cm4" && club.uid == 1488 {
                assert_eq!(d.current_wages_weekly, Some(37_877));
                assert_eq!(d.wage_budget_weekly, Some(60_764));
                let payroll = d.payroll_expenses.unwrap();
                assert_eq!(
                    (
                        payroll.wages.current_month,
                        payroll.wages.previous_month,
                        payroll.wages.season
                    ),
                    (77_324, 77_774, 155_098)
                );
                assert_eq!(
                    (
                        payroll.bonuses.current_month,
                        payroll.bonuses.previous_month,
                        payroll.bonuses.season
                    ),
                    (2_871, 0, 2_871)
                );
            }
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
        stadium_index: None,
        roster_offset: 0,
        roster_person_indices: Vec::new(),
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
