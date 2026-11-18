use cm0304_core::{
    database,
    editor::{self, AbilityPolicy, Edit},
};
use std::{fs, path::PathBuf};
fn sample() -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4"),
    )
    .unwrap()
}
fn edit(key: &str, value: i32, policy: Option<AbilityPolicy>) -> Edit {
    Edit {
        uid: 2001642,
        attribute: key.into(),
        value,
        policy,
    }
}

#[test]
fn player_fields_preserve_values_when_ca_changes_and_roundtrip_multibyte_data() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let before = db
        .players(base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 2001642)
        .unwrap();
    let nation = before.fields["nationality"].value;
    // Independent source evidence: Pregame editor labels height as inches,
    // and Clodoaldo's source record stores 63 inches (160 cm), weight unknown.
    assert_eq!(before.fields["height"].value, 160);
    assert_eq!(before.fields["weight"].value, 0);
    let plan = vec![
        edit("currentAbility", 155, Some(AbilityPolicy::PreserveValues)),
        edit("potentialAbility", 180, None),
        edit("height", 170, None),
        edit("weight", 65, None),
        edit("birth.year", 1980, None),
        edit("birth.day", 59, None),
        edit("nationality", (nation + 1) % 230, None),
        edit("personality.professionalism", 20, None),
        edit("reputation.current", 7000, None),
        edit("leftFoot", 19, None),
        edit("consistency", 20, None),
    ];
    let (output, report) = editor::apply(&input, &plan).unwrap();
    let (after_db, after_base) = database::load(&output).unwrap();
    let after = after_db
        .players(after_base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 2001642)
        .unwrap();
    assert_eq!(after.current_ability, 155);
    assert_eq!(after.potential_ability, 180);
    assert_eq!(after.fields["height"].value, 170);
    let height_offset = after.fields["height"].offset;
    assert_eq!(
        i16::from_le_bytes(output[height_offset..height_offset + 2].try_into().unwrap()),
        67
    );
    assert_eq!(after.fields["weight"].value, 65);
    assert_eq!(after.date_of_birth.year, 1980);
    assert_eq!(after.date_of_birth.day, 59);
    for key in &cm0304_core::attributes::FIELDS[12..37] {
        assert_eq!(before.attributes[*key], after.attributes[*key], "{key}");
    }
    assert_eq!(after.personality.as_ref().unwrap()["professionalism"], 20);
    assert_eq!(after.fields["nationality"].value, (nation + 1) % 230);
    assert_ne!(after.nation, before.nation);
    assert!(report.changes.iter().any(|c| c.derived));
    assert!(report.changes.iter().any(|c| c.width == 2));
    let expected = report
        .changes
        .iter()
        .flat_map(|c| c.offset..c.offset + c.width as usize)
        .collect::<std::collections::BTreeSet<_>>();
    for (i, (a, b)) in input.iter().zip(&output).enumerate() {
        if a != b {
            assert!(expected.contains(&i));
        }
    }
}
#[test]
fn relative_policy_keeps_bytes_and_reports_indirect_attribute_effects() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let before = db
        .players(base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 2001642)
        .unwrap();
    let (output, report) = editor::apply(
        &input,
        &[edit(
            "currentAbility",
            150,
            Some(AbilityPolicy::PreserveRelative),
        )],
    )
    .unwrap();
    let (db, base) = database::load(&output).unwrap();
    let after = db
        .players(base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 2001642)
        .unwrap();
    assert_eq!(before.raw_attributes, after.raw_attributes);
    assert_ne!(before.attributes, after.attributes);
    assert!(
        report
            .changes
            .iter()
            .any(|c| c.derived && c.before != c.after && c.raw_before == c.raw_after)
    );
    assert_eq!(report.changed_bytes, 1);
    let (output, _) = editor::apply(
        &input,
        &[
            edit(
                "position.goalkeeper",
                20,
                Some(AbilityPolicy::PreserveRelative),
            ),
            edit("passing", 20, None),
        ],
    )
    .unwrap();
    let (db, base) = database::load(&output).unwrap();
    let after = db
        .players(base)
        .unwrap()
        .into_iter()
        .find(|p| p.uid == 2001642)
        .unwrap();
    assert_eq!(after.attributes["goalkeeper"], 20);
    assert_eq!(after.attributes["passing"], 20);
}
#[test]
fn dependent_edits_require_explicit_policy_and_invalid_dates_are_rejected() {
    let input = sample();
    assert!(editor::apply(&input, &[edit("currentAbility", 150, None)]).is_err());
    assert!(editor::apply(&input, &[edit("position.goalkeeper", 20, None)]).is_err());
    assert!(editor::apply(&input, &[edit("potentialAbility", 100, None)]).is_err());
    assert!(editor::apply(&input, &[edit("potentialAbility", 0, None)]).is_err());
    assert!(
        editor::apply(
            &input,
            &[edit("birth.day", 365, None), edit("birth.year", 1979, None)]
        )
        .is_err()
    );
    assert!(editor::apply(&input, &[edit("nationality", 230, None)]).is_err());
    assert!(editor::apply(&input, &[edit("height", 171, None)]).is_err());
    let (output, _) = editor::apply(&input, &[edit("potentialAbility", -1, None)]).unwrap();
    let (db, base) = database::load(&output).unwrap();
    assert_eq!(
        db.players(base)
            .unwrap()
            .into_iter()
            .find(|p| p.uid == 2001642)
            .unwrap()
            .potential_ability,
        -1
    );
}
