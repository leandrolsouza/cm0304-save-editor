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
fn dual_role_conversion_matches_original_and_edited_native_profiles() {
    let observations: serde_json::Value = serde_json::from_str(include_str!(
        "../../../reports/v06-dual-staff-observed.json"
    ))
    .unwrap();
    assert_eq!(observations.as_array().unwrap().len(), 16);
    for row in observations.as_array().unwrap() {
        let raw = row["raw"].as_i64().unwrap() as i8;
        assert_eq!(
            cm0304_core::attributes::convert(raw, 147, false).unwrap(),
            row["observed"].as_i64().unwrap() as i8
        );
    }
    let input = sample("save_inicial.cm4");
    let (db, base) = database::load(&input).unwrap();
    let profiles = staff::profiles(&db, &input, base).unwrap();
    let p = profiles.iter().find(|p| p.uid == 600).unwrap();
    assert_eq!(p.current_ability, 150);
    assert_eq!(p.player_current_ability, Some(147));
    assert_eq!(p.attributes.as_ref().unwrap()["manManagement"], 12);
    assert_eq!(p.attributes.as_ref().unwrap()["tacticalKnowledge"], 17);
}

#[test]
fn staff_ability_projection_requires_a_policy_and_respects_explicit_targets() {
    use cm0304_core::{editor::AbilityPolicy, staff_editor::project_ability};
    use std::collections::BTreeMap;
    let input = sample("save_inicial.cm4");
    let (db, base) = database::load(&input).unwrap();
    let profiles = staff::profiles(&db, &input, base).unwrap();
    for uid in [4950, 600] {
        let p = profiles.iter().find(|p| p.uid == uid).unwrap();
        let original = serde_json::to_value(p).unwrap();
        let preserved =
            project_ability(p, 100, AbilityPolicy::PreserveValues, &BTreeMap::new()).unwrap();
        assert_eq!(preserved.attributes_after, *p.attributes.as_ref().unwrap());
        if !p.is_player {
            assert_ne!(
                preserved.raw_attributes_after,
                *p.raw_attributes.as_ref().unwrap()
            );
        }
        let relative =
            project_ability(p, 100, AbilityPolicy::PreserveRelative, &BTreeMap::new()).unwrap();
        assert_eq!(
            relative.raw_attributes_after,
            *p.raw_attributes.as_ref().unwrap()
        );
        if p.is_player {
            assert_eq!(relative.attributes_after, relative.attributes_before);
        } else {
            assert_ne!(relative.attributes_after, relative.attributes_before);
        }
        for policy in [
            AbilityPolicy::PreserveValues,
            AbilityPolicy::PreserveRelative,
        ] {
            let overrides = BTreeMap::from([
                ("coachingOutfieldPlayers".into(), 20),
                ("discipline".into(), 19),
            ]);
            let result = project_ability(p, 100, policy, &overrides).unwrap();
            assert_eq!(result.attributes_after["coachingOutfieldPlayers"], 20);
            assert_eq!(result.attributes_after["discipline"], 19);
            assert_eq!(result.current_ability_before, p.current_ability);
            assert_eq!(result.current_ability_after, 100);
        }
        assert_eq!(serde_json::to_value(p).unwrap(), original);
        assert!(project_ability(p, -1, AbilityPolicy::PreserveValues, &BTreeMap::new()).is_err());
        assert!(project_ability(p, 201, AbilityPolicy::PreserveValues, &BTreeMap::new()).is_err());
        for (key, value) in [("unknown", 20), ("discipline", 0), ("discipline", 21)] {
            assert!(
                project_ability(
                    p,
                    100,
                    AbilityPolicy::PreserveValues,
                    &BTreeMap::from([(key.into(), value)])
                )
                .is_err()
            );
        }
        assert!(
            project_ability(
                p,
                200,
                AbilityPolicy::PreserveRelative,
                &BTreeMap::from([("coachingOutfieldPlayers".into(), 1)])
            )
            .is_err()
        );
    }
    let mut missing = profiles[0].clone();
    missing.raw_attributes = None;
    assert!(
        project_ability(
            &missing,
            100,
            AbilityPolicy::PreserveValues,
            &BTreeMap::new()
        )
        .is_err()
    );
}

#[test]
fn staff_plan_replaces_targets_reverts_and_rejects_invalid_submissions_atomically() {
    use cm0304_core::staff_editor::{Edit, merge_plan};
    let input = sample("save_inicial.cm4");
    let (db, base) = database::load(&input).unwrap();
    let profiles = staff::profiles(&db, &input, base).unwrap();
    let edit = |uid, attribute: &str, value| Edit {
        uid,
        attribute: attribute.into(),
        value,
    };
    let first = merge_plan(
        &profiles,
        &[],
        vec![edit(4950, "coachingOutfieldPlayers", 20)],
    )
    .unwrap();
    assert_eq!(first.len(), 1);
    assert_eq!(merge_plan(&profiles, &first, first.clone()).unwrap(), first);
    let replaced = merge_plan(
        &profiles,
        &first,
        vec![
            edit(4950, "coachingOutfieldPlayers", 19),
            edit(600, "judgingPlayerAbility", 20),
        ],
    )
    .unwrap();
    assert_eq!(replaced.len(), 2);
    assert_eq!(replaced.iter().find(|e| e.uid == 4950).unwrap().value, 19);
    let reverted = merge_plan(
        &profiles,
        &replaced,
        vec![edit(4950, "coachingOutfieldPlayers", 12)],
    )
    .unwrap();
    assert_eq!(reverted, vec![edit(600, "judgingPlayerAbility", 20)]);
    assert!(
        merge_plan(
            &profiles,
            &reverted,
            vec![edit(600, "judgingPlayerAbility", 15)]
        )
        .unwrap()
        .is_empty()
    );
    let snapshot = first.clone();
    for invalid in [
        edit(4950, "discipline", 0),
        edit(4950, "discipline", 21),
        edit(i32::MAX, "discipline", 20),
        edit(4950, "unknown", 20),
    ] {
        assert!(
            merge_plan(
                &profiles,
                &first,
                vec![edit(4950, "discipline", 20), invalid]
            )
            .is_err()
        );
        assert_eq!(first, snapshot);
    }
    assert!(merge_plan(&profiles, &first, vec![first[0].clone(), first[0].clone()]).is_err());
    assert!(merge_plan(&profiles, &[first[0].clone(), first[0].clone()], vec![]).is_err());
    let mut altered = profiles.clone();
    let rui = altered.iter_mut().find(|p| p.uid == 4950).unwrap();
    rui.current_ability = 200;
    assert!(
        merge_plan(
            &altered,
            &first,
            vec![edit(4950, "coachingOutfieldPlayers", 1)]
        )
        .is_err()
    );
    let rui_offset = profiles
        .iter()
        .find(|p| p.uid == 4950)
        .unwrap()
        .attributes_offset;
    altered
        .iter_mut()
        .find(|p| p.uid == 600)
        .unwrap()
        .attributes_offset = rui_offset;
    assert!(
        merge_plan(&altered, &first, vec![edit(4950, "discipline", 20)])
            .unwrap_err()
            .contains("compartilhado")
    );
    let rui = altered.iter_mut().find(|p| p.uid == 4950).unwrap();
    rui.attributes = None;
    assert!(merge_plan(&altered, &first, vec![edit(4950, "discipline", 20)]).is_err());
}

#[test]
fn staff_attribute_transaction_preserves_players_and_only_changes_planned_bytes() {
    use cm0304_core::staff_editor::{self, Edit};
    for name in ["save_inicial.cm4", "save_modificado.cm4"] {
        let input = sample(name);
        let (db, base) = database::load(&input).unwrap();
        let before = staff::profiles(&db, &input, base).unwrap();
        let edits = vec![
            Edit {
                uid: 4950,
                attribute: "coachingOutfieldPlayers".into(),
                value: 20,
            },
            Edit {
                uid: 4950,
                attribute: "discipline".into(),
                value: 20,
            },
            Edit {
                uid: 600,
                attribute: "judgingPlayerAbility".into(),
                value: 20,
            },
        ];
        let (output, changes) = staff_editor::apply(&input, &edits).unwrap();
        assert_eq!(changes.len(), 3);
        assert_eq!(input.len(), output.len());
        let changed = input
            .iter()
            .zip(&output)
            .enumerate()
            .filter_map(|(i, (a, b))| (a != b).then_some(i))
            .collect::<BTreeSet<_>>();
        assert_eq!(changed, changes.iter().map(|c| c.offset).collect());
        let (after_db, after_base) = database::load(&output).unwrap();
        assert_eq!(
            serde_json::to_value(db.players(base).unwrap()).unwrap(),
            serde_json::to_value(after_db.players(after_base).unwrap()).unwrap()
        );
        let after = staff::profiles(&after_db, &output, after_base).unwrap();
        for (a, b) in before.iter().zip(&after) {
            assert_eq!(a.uid, b.uid);
            assert_eq!(a.current_ability, b.current_ability);
            assert_eq!(a.potential_ability, b.potential_ability);
            assert_eq!(a.reputation, b.reputation);
            assert_eq!(a.personality, b.personality);
            assert_eq!(a.team, b.team);
            for key in staff::FIELDS {
                let expected = edits
                    .iter()
                    .find(|e| e.uid == a.uid && e.attribute == key)
                    .map_or(a.attributes.as_ref().unwrap()[key], |e| e.value);
                assert_eq!(b.attributes.as_ref().unwrap()[key], expected);
            }
        }
        assert!(staff_editor::apply(&input, &[]).is_err());
        assert!(staff_editor::apply(&input, &[edits[0].clone(), edits[0].clone()]).is_err());
        for (uid, attribute, value) in [
            (i32::MAX, "discipline", 20),
            (4950, "unknown", 20),
            (4950, "discipline", 0),
            (4950, "discipline", 21),
        ] {
            assert!(
                staff_editor::apply(
                    &input,
                    &[Edit {
                        uid,
                        attribute: attribute.into(),
                        value
                    }]
                )
                .is_err()
            );
        }
        // Force two staff members to reference the same professional record.
        let rui = before.iter().find(|p| p.uid == 4950).unwrap();
        let dual = before.iter().find(|p| p.uid == 600).unwrap();
        let mut shared = input.clone();
        shared[dual.info_offset + 11..dual.info_offset + 15]
            .copy_from_slice(&rui.attribute_index.to_le_bytes());
        let error = staff_editor::apply(&shared, &edits[..1]).unwrap_err();
        assert!(error.contains("compartilhado"));
    }
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

#[test]
fn staff_conversion_matches_twenty_four_native_observations_and_exact_inverse() {
    let observations: serde_json::Value = serde_json::from_str(include_str!(
        "../../../reports/v06-staff-conversion-observed.json"
    ))
    .unwrap();
    assert_eq!(observations.as_array().unwrap().len(), 24);
    for observation in observations.as_array().unwrap() {
        let key = observation["attribute"].as_str().unwrap();
        let raw = observation["raw"].as_i64().unwrap() as i8;
        let ca = observation["ca"].as_i64().unwrap() as i16;
        let expected = observation["observed"].as_i64().unwrap() as i8;
        let mut attributes: cm0304_core::attributes::Attributes = staff::FIELDS
            .iter()
            .map(|key| (key.to_string(), 0))
            .collect();
        attributes.insert(key.to_string(), raw);
        assert_eq!(
            staff::decoded(&attributes, ca).unwrap()[key],
            expected,
            "{observation}"
        );
        assert_eq!(
            staff::encode_attribute(key, expected, ca, raw).unwrap(),
            raw
        );
    }
    for ca in 0..=200 {
        for target in 1..=20 {
            match staff::encode_attribute("manManagement", target, ca, -24) {
                Ok(raw) => assert_eq!(
                    cm0304_core::attributes::convert(raw, ca, false).unwrap(),
                    target
                ),
                Err(_) => assert!(!(-128i16..=127).any(|raw| {
                    cm0304_core::attributes::convert(raw as i8, ca, false).unwrap() == target
                })),
            }
        }
    }
    assert!(staff::encode_attribute("unknown", 10, 70, 0).is_err());
    assert!(staff::encode_attribute("attacking", 0, 70, 0).is_err());
    assert_eq!(
        staff::encode_attribute("attacking", 20, 70, 12).unwrap(),
        20
    );
}
