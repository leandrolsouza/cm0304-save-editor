use cm0304_core::{
    database, editor::AbilityPolicy, staff, staff_editor, staff_header, transaction,
};
use std::{fs, path::PathBuf};

fn sample() -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap()
}

#[test]
fn header_edits_preserve_player_role_and_show_ca_effects_for_both_policies() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let before = staff::profiles(&db, &input, base).unwrap();
    for policy in [
        AbilityPolicy::PreserveValues,
        AbilityPolicy::PreserveRelative,
    ] {
        let plan = transaction::Transaction {
            staff_header_edits: vec![staff_header::Edit {
                uid: 600,
                current_ability: Some(100),
                potential_ability: Some(180),
                reputation: Some([8000, 8500, 9000]),
                policy: Some(policy),
            }],
            staff_edits: vec![staff_editor::Edit {
                uid: 600,
                attribute: "coachingOutfieldPlayers".into(),
                value: 20,
            }],
            ..Default::default()
        };
        let (output, report) = transaction::apply(&input, &plan).unwrap();
        let (after_db, after_base) = database::load(&output).unwrap();
        assert_eq!(
            serde_json::to_value(db.players(base).unwrap()).unwrap(),
            serde_json::to_value(after_db.players(after_base).unwrap()).unwrap()
        );
        let after = staff::profiles(&after_db, &output, after_base).unwrap();
        let a = before.iter().find(|p| p.uid == 600).unwrap();
        let b = after.iter().find(|p| p.uid == 600).unwrap();
        assert_eq!(
            (b.current_ability, b.potential_ability, b.reputation),
            (100, 180, [8000, 8500, 9000])
        );
        assert_eq!(
            b.attributes.as_ref().unwrap()["coachingOutfieldPlayers"],
            20
        );
        for key in staff::FIELDS {
            if key == "coachingOutfieldPlayers" {
                continue;
            }
            if policy == AbilityPolicy::PreserveValues {
                assert_eq!(
                    a.attributes.as_ref().unwrap()[key],
                    b.attributes.as_ref().unwrap()[key]
                );
            } else {
                assert_eq!(
                    a.raw_attributes.as_ref().unwrap()[key],
                    b.raw_attributes.as_ref().unwrap()[key]
                );
            }
        }
        assert!(
            report
                .changes
                .iter()
                .any(|c| c.attribute == "currentAbility" && !c.derived)
        );
        assert!(report.changes.iter().any(|c| c.derived));
        let allowed = report
            .changes
            .iter()
            .flat_map(|c| c.offset..c.offset + usize::from(c.width))
            .collect::<std::collections::BTreeSet<_>>();
        let actual = input
            .iter()
            .zip(&output)
            .enumerate()
            .filter_map(|(i, (a, b))| (a != b).then_some(i))
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), report.field_changed_bytes);
        assert!(actual.iter().all(|i| allowed.contains(i)));
        for (a, b) in before.iter().zip(&after) {
            if a.uid != 600 {
                assert_eq!(
                    serde_json::to_value(a).unwrap(),
                    serde_json::to_value(b).unwrap()
                );
            }
        }
    }
}

#[test]
fn header_validation_requires_policy_and_rejects_invalid_and_duplicate_targets() {
    let input = sample();
    let ca = staff_header::Edit {
        uid: 4950,
        current_ability: Some(100),
        policy: Some(AbilityPolicy::PreserveValues),
        ..Default::default()
    };
    for invalid in [
        staff_header::Edit {
            policy: None,
            ..ca.clone()
        },
        staff_header::Edit {
            current_ability: Some(201),
            ..ca.clone()
        },
        staff_header::Edit {
            current_ability: Some(-1),
            ..ca.clone()
        },
        staff_header::Edit {
            potential_ability: Some(-1),
            ..ca.clone()
        },
        staff_header::Edit {
            reputation: Some([0, 10001, 0]),
            ..ca.clone()
        },
        staff_header::Edit {
            uid: i32::MAX,
            ..ca.clone()
        },
        staff_header::Edit {
            uid: 4950,
            ..Default::default()
        },
        staff_header::Edit {
            uid: 4950,
            potential_ability: Some(180),
            policy: Some(AbilityPolicy::PreserveValues),
            ..Default::default()
        },
    ] {
        assert!(staff_header::apply(&input, &[invalid], &[]).is_err());
    }
    assert!(staff_header::apply(&input, &[ca.clone(), ca.clone()], &[]).is_err());
    let bad = staff_editor::Edit {
        uid: 4950,
        attribute: "discipline".into(),
        value: 20,
    };
    assert!(staff_header::apply(&input, &[ca], &[bad.clone(), bad]).is_err());
}
