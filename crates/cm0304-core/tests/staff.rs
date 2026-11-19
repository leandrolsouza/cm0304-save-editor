use cm0304_core::{container, database, staff};
use std::{collections::BTreeSet, fs, path::PathBuf};

fn sample(name: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games")
            .join(name),
    )
    .unwrap()
}

#[test]
fn staff_records_resolve_in_both_samples_without_changing_historical_players() {
    for name in ["save_inicial.cm4", "save_modificado.cm4"] {
        let input = sample(name);
        let (db, base) = database::load(&input).unwrap();
        let boundary = db.parsed_until;
        let profiles = staff::profiles(&db, &input, base).unwrap();
        assert_eq!(profiles.len(), 6221);
        assert_eq!(profiles.iter().filter(|p| p.is_player).count(), 303);
        assert_eq!(profiles.iter().filter(|p| p.is_human).count(), 1);
        assert_eq!(
            profiles
                .iter()
                .map(|p| p.uid)
                .collect::<BTreeSet<_>>()
                .len(),
            profiles.len()
        );
        for profile in &profiles {
            let attributes = profile.raw_attributes.as_ref().unwrap();
            let offset = profile.attributes_offset.unwrap();
            assert_eq!(attributes.len(), 28);
            for (i, key) in staff::FIELDS.iter().enumerate() {
                assert_eq!(attributes[*key], input[offset + i] as i8);
            }
            assert_eq!(
                profile.current_ability,
                i16::from_le_bytes(
                    input[profile.info_offset + 6..profile.info_offset + 8]
                        .try_into()
                        .unwrap()
                )
            );
        }
        let assistant = profiles.iter().find(|p| p.uid == 4950).unwrap();
        assert_eq!(assistant.name, "Rui Águas");
        assert_eq!(assistant.team.as_deref(), Some("Sporting Clube de Braga"));
        assert_eq!(
            (assistant.current_ability, assistant.potential_ability),
            (70, 140)
        );
        assert_eq!(
            (assistant.club_job_raw, assistant.club_job),
            (20, "Treinador adjunto")
        );
        let dual = profiles.iter().find(|p| p.uid == 600).unwrap();
        assert_eq!(dual.name, "Pär Zetterberg");
        assert!(dual.is_player);
        assert_eq!(dual.current_ability, 150);
        assert_eq!(dual.national_team.as_deref(), Some("Suécia"));
        assert_eq!(dual.raw_attributes.as_ref().unwrap()["manManagement"], -24);
        assert_eq!(db.players(base).unwrap().len(), 22062);
        assert_eq!(db.sections.len(), 23);
        assert_eq!(db.parsed_until, boundary);
    }
}

#[test]
fn missing_staff_table_and_invalid_attribute_references_are_rejected() {
    let input = sample("save_inicial.cm4");
    let (db, base) = database::load(&input).unwrap();
    let block = container::read_container(&input)
        .unwrap()
        .blocks
        .into_iter()
        .find(|b| b.payload_offset == base)
        .unwrap();
    let payload = &input[base..base + block.size];
    let truncated =
        container::replace_block(&input, "game_db", ".dat", &payload[..db.parsed_until]).unwrap();
    let (short_db, short_base) = database::load(&truncated).unwrap();
    assert!(staff::profiles(&short_db, &truncated, short_base).is_err());
    let person = db.people.iter().find(|p| p.uid == 4950).unwrap();
    let mut broken = payload.to_vec();
    broken[person.common_offset - 4..person.common_offset].copy_from_slice(&i32::MAX.to_le_bytes());
    let broken = container::replace_block(&input, "game_db", ".dat", &broken).unwrap();
    let (broken_db, broken_base) = database::load(&broken).unwrap();
    let error = staff::profiles(&broken_db, &broken, broken_base).unwrap_err();
    assert!(error.contains("4950"));
    assert!(error.contains("referência inválida"));
}

#[test]
fn dual_role_staff_reflects_shared_birth_edits_without_changing_staff_ability() {
    let input = sample("save_inicial.cm4");
    let (db, base) = database::load(&input).unwrap();
    let original = staff::profiles(&db, &input, base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 600)
        .unwrap();
    let plan = cm0304_core::transaction::Transaction {
        edits: vec![cm0304_core::editor::Edit {
            uid: 600,
            attribute: "birth.year".into(),
            value: 1971,
            policy: None,
        }],
        ..Default::default()
    };
    let (output, _) = cm0304_core::transaction::apply(&input, &plan).unwrap();
    let (after, after_base) = database::load(&output).unwrap();
    let projected = staff::profiles(&after, &output, after_base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 600)
        .unwrap();
    assert_eq!(projected.date_of_birth.as_ref().unwrap().year, 1971);
    assert_eq!(projected.current_ability, original.current_ability);
    assert_eq!(projected.potential_ability, original.potential_ability);
    assert_eq!(projected.reputation, original.reputation);
    assert_eq!(projected.raw_attributes, original.raw_attributes);
    assert_eq!(projected.team, original.team);
    assert_eq!(projected.national_team, original.national_team);
}
