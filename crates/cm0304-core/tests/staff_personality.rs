use cm0304_core::{
    database,
    editor::{AbilityPolicy, Edit},
    staff, staff_editor, staff_header, transaction,
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
fn all_personality_fields_roundtrip_for_staff_and_dual_role_with_header_changes() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let before = staff::profiles(&db, &input, base).unwrap();
    let mut plan = transaction::Transaction::default();
    for uid in [4950, 600] {
        let p = before.iter().find(|p| p.uid == uid).unwrap();
        for key in database::PERSONALITY_FIELDS {
            let old = p.personality.as_ref().unwrap()[key];
            plan.staff_edits.push(staff_editor::Edit {
                uid,
                attribute: format!("personality.{key}"),
                value: if old == 20 { 19 } else { 20 },
            });
        }
        plan.staff_header_edits.push(staff_header::Edit {
            uid,
            current_ability: Some(100),
            policy: Some(AbilityPolicy::PreserveValues),
            ..Default::default()
        });
    }
    let (output, report) = transaction::apply(&input, &plan).unwrap();
    assert_eq!(report.staff_changes.len(), 16);
    let (after_db, after_base) = database::load(&output).unwrap();
    let after = staff::profiles(&after_db, &output, after_base).unwrap();
    for edit in &plan.staff_edits {
        assert_eq!(
            staff_editor::value(
                after.iter().find(|p| p.uid == edit.uid).unwrap(),
                &edit.attribute
            )
            .unwrap(),
            edit.value
        );
    }
    let original_players = db.players(base).unwrap();
    let players = after_db.players(after_base).unwrap();
    for (a, b) in original_players.iter().zip(&players) {
        let mut a = serde_json::to_value(a).unwrap();
        let mut b = serde_json::to_value(b).unwrap();
        if a["uid"] == 600 {
            a.as_object_mut().unwrap().remove("personality");
            b.as_object_mut().unwrap().remove("personality");
            for key in database::PERSONALITY_FIELDS {
                let key = format!("personality.{key}");
                a["fields"].as_object_mut().unwrap().remove(&key);
                b["fields"].as_object_mut().unwrap().remove(&key);
            }
        }
        assert_eq!(a, b);
    }
    let dual = players.iter().find(|p| p.uid == 600).unwrap();
    assert_eq!(
        dual.personality,
        after.iter().find(|p| p.uid == 600).unwrap().personality
    );
    let allowed = report
        .changes
        .iter()
        .flat_map(|c| c.offset..c.offset + usize::from(c.width))
        .chain(report.staff_changes.iter().map(|c| c.offset))
        .collect::<std::collections::BTreeSet<_>>();
    let actual = input
        .iter()
        .zip(&output)
        .enumerate()
        .filter_map(|(i, (a, b))| (a != b).then_some(i))
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), report.field_changed_bytes);
    assert!(actual.iter().all(|i| allowed.contains(i)));
}

#[test]
fn personality_rejects_shared_records_bad_values_and_conflicting_role_intentions() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let target = staff_editor::Edit {
        uid: 4950,
        attribute: "personality.professionalism".into(),
        value: 20,
    };
    let owner = db.people.iter().find(|p| p.uid == 4950).unwrap();
    let other = db.people.iter().find(|p| p.uid == 2001642).unwrap();
    let mut shared = input.clone();
    let offset = base + other.common_offset + 43;
    shared[offset..offset + 4].copy_from_slice(&owner.personality.to_le_bytes());
    assert!(
        staff_editor::apply(&shared, std::slice::from_ref(&target))
            .unwrap_err()
            .contains("compartilhada")
    );
    for value in [0, 21] {
        assert!(
            staff_editor::apply(
                &input,
                &[staff_editor::Edit {
                    value,
                    ..target.clone()
                }]
            )
            .is_err()
        );
    }
    let mut plan = transaction::Transaction {
        edits: vec![Edit {
            uid: 600,
            attribute: target.attribute.clone(),
            value: 19,
            policy: None,
        }],
        staff_edits: vec![staff_editor::Edit { uid: 600, ..target }],
        ..Default::default()
    };
    let profiles = staff::profiles(&db, &input, base).unwrap();
    let old = staff_editor::value(
        profiles.iter().find(|p| p.uid == 600).unwrap(),
        "personality.professionalism",
    )
    .unwrap();
    let mut player_plan = transaction::Transaction {
        edits: vec![Edit {
            uid: 600,
            attribute: "personality.professionalism".into(),
            value: 20,
            policy: None,
        }],
        ..Default::default()
    };
    let snapshot = player_plan.clone();
    assert!(
        player_plan
            .merge_staff(
                &profiles,
                vec![staff_editor::Edit {
                    uid: 600,
                    attribute: "personality.professionalism".into(),
                    value: old
                }],
                None
            )
            .is_err()
    );
    assert_eq!(player_plan, snapshot);
    let mut staff_plan = transaction::Transaction {
        staff_edits: vec![staff_editor::Edit {
            uid: 600,
            attribute: "personality.professionalism".into(),
            value: 20,
        }],
        ..Default::default()
    };
    let snapshot = staff_plan.clone();
    assert!(
        staff_plan
            .merge_fields(
                &db.players(base).unwrap(),
                vec![Edit {
                    uid: 600,
                    attribute: "personality.professionalism".into(),
                    value: i32::from(old),
                    policy: None
                }]
            )
            .is_err()
    );
    assert_eq!(staff_plan, snapshot);
    assert!(
        transaction::apply(&input, &plan)
            .unwrap_err()
            .contains("conflitantes")
    );
    plan.edits[0].value = 20;
    let (output, report) = transaction::apply(&input, &plan).unwrap();
    assert!(report.staff_changes.is_empty());
    let (db, base) = database::load(&output).unwrap();
    assert_eq!(
        staff_editor::value(
            staff::profiles(&db, &output, base)
                .unwrap()
                .iter()
                .find(|p| p.uid == 600)
                .unwrap(),
            "personality.professionalism"
        )
        .unwrap(),
        20
    );
}
