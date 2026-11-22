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
fn player_edits_reject_references_shared_with_another_person() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let player = db
        .people
        .iter()
        .find(|person| person.uid == 2001642)
        .unwrap();
    let other = db.people.iter().find(|person| person.uid == 1855).unwrap();
    let playing_offset = base + other.playing.as_ref().unwrap().offset + 10;
    let personality_offset = base + other.common_offset + 43;

    let mut shared = input.clone();
    let playing_index = player.playing.as_ref().unwrap().playing_index;
    shared[playing_offset..playing_offset + 4].copy_from_slice(&playing_index.to_le_bytes());
    assert_eq!(
        editor::apply(&shared, &[edit("passing", 20, None)]).unwrap_err(),
        "Shared playing data cannot be edited"
    );

    shared[playing_offset..playing_offset + 4]
        .copy_from_slice(&input[playing_offset..playing_offset + 4]);
    shared[personality_offset..personality_offset + 4]
        .copy_from_slice(&player.personality.to_le_bytes());
    assert_eq!(
        editor::apply(&shared, &[edit("personality.professionalism", 20, None)]).unwrap_err(),
        "Shared personality data cannot be edited"
    );

    let (output, _) = editor::apply(&shared, &[edit("state.morale", 20, None)]).unwrap();
    assert_eq!(shared.len(), output.len());
    assert_eq!(
        shared.iter().zip(&output).filter(|(a, b)| a != b).count(),
        1
    );
}

#[test]
fn merging_forms_retains_overrides_and_cleans_up_removed_dependencies() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let players = db.players(base).unwrap();
    let plan = editor::merge_plan(
        &players,
        &Default::default(),
        vec![edit(
            "currentAbility",
            150,
            Some(AbilityPolicy::PreserveRelative),
        )],
    )
    .unwrap();
    let plan = editor::merge_plan(&players, &plan, vec![edit("finishing", 13, None)]).unwrap();
    assert_eq!(plan.len(), 2); // explicit source-value override is significant
    let reverted =
        editor::merge_plan(&players, &plan, vec![edit("currentAbility", 142, None)]).unwrap();
    assert!(reverted.is_empty()); // no stale no-op target blocks reverting CA
    assert!(
        editor::merge_plan(
            &players,
            &plan,
            vec![edit("height", 170, None), edit("height", 180, None)]
        )
        .is_err()
    );
    let mixed = editor::merge_plan(
        &players,
        &plan,
        vec![edit("height", 170, None), edit("currentAbility", 142, None)],
    )
    .unwrap();
    assert_eq!(mixed.len(), 1);
    assert_eq!(mixed.values().next().unwrap().attribute, "height");
}

#[test]
fn goalkeeper_and_player_staff_keep_their_separate_records() {
    let input = sample();
    let (db, base) = database::load(&input).unwrap();
    let players = db.players(base).unwrap();
    let keeper = players
        .iter()
        .find(|p| p.raw_attributes["goalkeeper"] >= 15 && p.current_ability < 190)
        .unwrap();
    let staff = players
        .iter()
        .find(|p| p.kind == 3 && p.current_ability < 190 && p.uid != keeper.uid)
        .unwrap();
    let make = |p: &cm0304_core::database::Player, key: &str, value, policy| Edit {
        uid: p.uid,
        attribute: key.into(),
        value,
        policy,
    };
    let plan = vec![
        make(keeper, "potentialAbility", 200, None),
        make(
            keeper,
            "currentAbility",
            keeper.current_ability as i32 + 1,
            Some(AbilityPolicy::PreserveValues),
        ),
        make(staff, "potentialAbility", 200, None),
        make(
            staff,
            "currentAbility",
            staff.current_ability as i32 + 1,
            Some(AbilityPolicy::PreserveRelative),
        ),
        make(staff, "reputation.world", 8000, None),
    ];
    let (output, _) = editor::apply(&input, &plan).unwrap();
    let (new_db, new_base) = database::load(&output).unwrap();
    let after = new_db.players(new_base).unwrap();
    let new_keeper = after.iter().find(|p| p.uid == keeper.uid).unwrap();
    let new_staff = after.iter().find(|p| p.uid == staff.uid).unwrap();
    assert_eq!(new_keeper.attributes, keeper.attributes);
    assert_eq!(new_staff.kind, 3);
    assert_eq!(new_staff.raw_attributes, staff.raw_attributes);
    assert_eq!(new_staff.fields["reputation.world"].value, 8000);
    for person in [keeper, staff] {
        let start = person.offsets.person;
        // The common person record and contracts stay untouched. Player/staff
        // also has a separate 15-byte staff record immediately before common.
        let common = db
            .people
            .iter()
            .find(|p| p.uid == person.uid)
            .unwrap()
            .common_offset
            + base;
        let next_person = db
            .people
            .iter()
            .find(|p| p.offset + base > start)
            .map(|p| p.offset + base)
            .unwrap_or(common + 1);
        assert_eq!(&input[common..next_person], &output[common..next_person]);
        if person.kind == 3 {
            assert_eq!(&input[common - 15..common], &output[common - 15..common]);
        }
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
    let error =
        editor::apply(&input, &[edit("flair", 15, None), edit("passing", 1, None)]).unwrap_err();
    assert!(error.contains("Clodoaldo (UID 2001642)"));
    assert!(error.contains("passing: O valor 1 não pode ser representado com CA 142"));
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
