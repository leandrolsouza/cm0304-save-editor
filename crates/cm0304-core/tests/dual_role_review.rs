use cm0304_core::{
    database,
    editor::{AbilityPolicy, Edit},
    staff, staff_editor, transaction,
};
use std::{fs, path::PathBuf};

#[test]
fn review_reconciles_playing_ca_and_explicit_staff_targets_against_source() {
    let input = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap();
    let (db, base) = database::load(&input).unwrap();
    let original = staff::profiles(&db, &input, base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 600)
        .unwrap();
    let base_plan = transaction::Transaction {
        edits: vec![Edit {
            uid: 600,
            attribute: "currentAbility".into(),
            value: 100,
            policy: Some(AbilityPolicy::PreserveValues),
        }],
        ..Default::default()
    };
    let (without_target, report) = transaction::apply(&input, &base_plan).unwrap();
    let derived = report
        .changes
        .iter()
        .filter(|c| c.uid == 600 && c.context.as_deref() == Some("Staff · efeito da CA de jogador"))
        .collect::<Vec<_>>();
    assert!(!derived.is_empty());
    assert!(
        derived
            .iter()
            .all(|c| c.derived && c.raw_before == c.raw_after)
    );
    let (after_db, after_base) = database::load(&without_target).unwrap();
    let projected = staff::profiles(&after_db, &without_target, after_base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 600)
        .unwrap();
    assert_eq!(projected.player_current_ability, Some(100));
    for c in derived {
        assert_eq!(
            c.before,
            i32::from(original.attributes.as_ref().unwrap()[&c.attribute])
        );
        assert_eq!(
            c.after,
            i32::from(projected.attributes.as_ref().unwrap()[&c.attribute])
        );
    }
    let mut explicit = base_plan;
    explicit.staff_edits = vec![staff_editor::Edit {
        uid: 600,
        attribute: "manManagement".into(),
        value: 12,
    }];
    explicit.staff_header_edits = vec![cm0304_core::staff_header::Edit {
        uid: 600,
        current_ability: Some(100),
        potential_ability: Some(180),
        policy: Some(AbilityPolicy::PreserveRelative),
        ..Default::default()
    }];
    let (output, report) = transaction::apply(&input, &explicit).unwrap();
    let matches = report
        .changes
        .iter()
        .filter(|c| {
            c.uid == 600
                && c.attribute == "manManagement"
                && c.context.as_deref().is_some_and(|s| s.starts_with("Staff"))
        })
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1);
    assert_eq!(
        (matches[0].before, matches[0].after, matches[0].derived),
        (12, 12, false)
    );
    assert_ne!(matches[0].raw_before, matches[0].raw_after);
    assert!(
        !report
            .staff_changes
            .iter()
            .any(|c| c.uid == 600 && c.attribute == "manManagement")
    );
    let actual = input.iter().zip(&output).filter(|(a, b)| a != b).count();
    assert_eq!(report.field_changed_bytes, actual);
    let (db, base) = database::load(&output).unwrap();
    let p = staff::profiles(&db, &output, base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 600)
        .unwrap();
    assert_eq!(p.attributes.as_ref().unwrap()["manManagement"], 12);
}
