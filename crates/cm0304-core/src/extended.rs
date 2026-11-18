//! Transaction planning for player fields, including CA/position dependencies.
use crate::{
    Result,
    attributes::{self, FIELDS},
    container::hash,
    database::{self, Field, Player},
    editor::{AbilityPolicy, Change, Edit, Report},
};
use std::collections::{BTreeMap, BTreeSet};

fn raw(bytes: &[u8], field: &Field) -> i32 {
    match field.width {
        1 => bytes[field.offset] as i8 as i32,
        2 => i16::from_le_bytes(bytes[field.offset..field.offset + 2].try_into().unwrap()) as i32,
        4 => i32::from_le_bytes(bytes[field.offset..field.offset + 4].try_into().unwrap()),
        _ => unreachable!(),
    }
}
fn patch(input: &[u8], patches: &mut BTreeMap<usize, u8>, field: &Field, value: i32) -> Result<()> {
    let bytes = value.to_le_bytes();
    for (i, b) in bytes.iter().enumerate().take(field.width as usize) {
        let offset = field.offset + i;
        if *b != input[offset]
            && let Some(old) = patches.insert(offset, *b)
            && old != *b
        {
            return Err(format!("Conflicting changes at {offset}"));
        }
    }
    Ok(())
}
pub fn validate_value(p: &Player, e: &Edit) -> Result<()> {
    let f = p
        .fields
        .get(&e.attribute)
        .ok_or_else(|| format!("Unknown field {}", e.attribute))?;
    if e.value < f.min || e.value > f.max {
        return Err(format!("{} must be {}–{}", e.attribute, f.min, f.max));
    }
    if e.attribute == "potentialAbility" && e.value == 0 {
        return Err("Use fixed PA 1–200 or the explicit special values -1/-2".into());
    }
    if e.policy.is_some() && e.attribute != "currentAbility" && e.attribute != "position.goalkeeper"
    {
        return Err("Ability policy is only accepted for CA or goalkeeper position".into());
    }
    Ok(())
}
pub fn apply(input: &[u8], edits: &[Edit]) -> Result<(Vec<u8>, Report)> {
    if edits.is_empty() {
        return Err("Empty edit plan".into());
    }
    let (db, base) = database::load(input)?;
    let players = db.players(base)?;
    let mut by_uid = BTreeMap::new();
    for p in &players {
        if by_uid.insert(p.uid, p).is_some() {
            return Err(format!("Duplicate player UID {}", p.uid));
        }
    }
    let mut groups: BTreeMap<i32, BTreeMap<&str, &Edit>> = BTreeMap::new();
    for e in edits {
        let p = by_uid
            .get(&e.uid)
            .ok_or_else(|| format!("Unknown player UID {}", e.uid))?;
        validate_value(p, e)?;
        if groups
            .entry(e.uid)
            .or_default()
            .insert(&e.attribute, e)
            .is_some()
        {
            return Err("Duplicate player field".into());
        }
    }
    let mut patches = BTreeMap::new();
    let mut expected: BTreeMap<(i32, String), i32> = BTreeMap::new();
    for (uid, group) in &groups {
        let p = by_uid[uid];
        let person = db
            .people
            .iter()
            .find(|v| v.uid == *uid && v.playing.is_some())
            .ok_or("Missing person")?;
        crate::career::validate(&db, p, person, group, edits)?;
        let value = |key: &str| {
            group
                .get(key)
                .map(|e| e.value)
                .unwrap_or(p.fields[key].value)
        };
        let ca = value("currentAbility") as i16;
        let goalkeeper = value("position.goalkeeper") as i8;
        let dependencies_changed =
            ca != p.current_ability || (goalkeeper >= 15) != (p.raw_attributes["goalkeeper"] >= 15);
        let policies = group.values().filter_map(|e| e.policy).collect::<Vec<_>>();
        if policies.windows(2).any(|w| w[0] != w[1]) {
            return Err("Conflicting CA/position policies".into());
        }
        let policy = policies.first().copied();
        if dependencies_changed && policy.is_none() {
            return Err("Choose preserveValues or preserveRelative when changing CA or goalkeeper classification".into());
        }
        if (group.contains_key("currentAbility") || group.contains_key("potentialAbility"))
            && value("potentialAbility") > 0
            && ca as i32 > value("potentialAbility")
        {
            return Err("Current ability cannot exceed fixed potential ability; edit PA explicitly if needed".into());
        }
        if group.contains_key("birth.year") || group.contains_key("birth.day") {
            let year = value("birth.year");
            let day = value("birth.day");
            let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
            if day >= if leap { 366 } else { 365 }
                || (year, day) > (db.game_date.year as i32, db.game_date.day as i32)
            {
                return Err("Invalid or future birth date".into());
            }
        }
        let mut targets = group
            .iter()
            .map(|(key, e)| (key.to_string(), e.value))
            .collect::<BTreeMap<_, _>>();
        if dependencies_changed && policy == Some(AbilityPolicy::PreserveValues) {
            for key in &FIELDS[12..37] {
                targets
                    .entry(key.to_string())
                    .or_insert(p.fields[*key].value);
            }
        }
        for (key, target) in targets {
            let f = &p.fields[&key];
            let playing_field = key.starts_with("position.") || FIELDS.contains(&key.as_str());
            if playing_field
                && db
                    .people
                    .iter()
                    .filter(|v| {
                        v.playing.as_ref().is_some_and(|info| {
                            info.playing_index == person.playing.as_ref().unwrap().playing_index
                        })
                    })
                    .count()
                    != 1
            {
                return Err("Shared playing data cannot be edited".into());
            }
            if key.starts_with("personality.")
                && db
                    .people
                    .iter()
                    .filter(|v| v.personality == person.personality)
                    .count()
                    != 1
            {
                return Err("Shared personality data cannot be edited".into());
            }
            let encoded = if let Some(i) = FIELDS.iter().position(|v| *v == key) {
                match attributes::mode(i, goalkeeper) {
                    Some(low) => {
                        attributes::encode(target as i8, ca, low, p.raw_attributes[&key])? as i32
                    }
                    None => target,
                }
            } else if key == "height" {
                (0..=118)
                    .find(|inches| (*inches as f64 * 2.54).round() as i32 == target)
                    .ok_or_else(|| {
                        format!("Height {target} cm cannot be represented exactly in whole inches")
                    })?
            } else {
                target
            };
            patch(input, &mut patches, f, encoded)?;
            expected.insert((*uid, key), target);
        }
    }
    if patches.is_empty() {
        return Err("No effective changes".into());
    }
    let mut output = input.to_vec();
    for (offset, value) in &patches {
        output[*offset] = *value;
    }
    let (new_db, new_base) = database::load(&output)?;
    let new_players = new_db.players(new_base)?;
    let new_index = new_players
        .iter()
        .map(|p| (p.uid, p))
        .collect::<BTreeMap<_, _>>();
    for ((uid, key), value) in &expected {
        if new_index
            .get(uid)
            .and_then(|p| p.fields.get(key))
            .map(|f| f.value)
            != Some(*value)
        {
            return Err(format!("Post-write verification failed: {uid}/{key}"));
        }
    }
    let mut changes = Vec::new();
    for uid in groups.keys() {
        let before = by_uid[uid];
        let after = new_index[uid];
        let person = db
            .people
            .iter()
            .find(|p| p.uid == *uid)
            .ok_or("Missing person")?;
        let contracts = crate::career::contracts(&db, person);
        for (key, old) in &before.fields {
            let new = &after.fields[key];
            let raw_before = raw(input, old);
            let raw_after = raw(&output, new);
            if old.value != new.value || raw_before != raw_after {
                changes.push(Change {
                    uid: *uid,
                    name: before.name.clone(),
                    attribute: key.clone(),
                    before: old.value,
                    after: new.value,
                    raw_before,
                    raw_after,
                    offset: old.offset,
                    width: old.width,
                    derived: !groups[uid].contains_key(key.as_str()),
                    context: key
                        .strip_prefix("contract.")
                        .and_then(|v| v.split('.').next())
                        .and_then(|v| v.parse::<usize>().ok())
                        .and_then(|i| contracts.get(i))
                        .map(|c| {
                            format!(
                                "{} · {}",
                                c.kind_label,
                                c.entity.as_deref().unwrap_or("Sem entidade")
                            )
                        }),
                    display_before: crate::career::display_field(before, key),
                    display_after: crate::career::display_field(after, key),
                });
            }
        }
    }
    let differences = input
        .iter()
        .zip(&output)
        .enumerate()
        .filter_map(|(i, (a, b))| (a != b).then_some(i))
        .collect::<BTreeSet<_>>();
    if differences != patches.keys().copied().collect() {
        return Err("Unexpected byte differences".into());
    }
    let report = Report {
        source_sha256: hash(input),
        copy_sha256: hash(&output),
        size: output.len(),
        changed_bytes: differences.len(),
        changes,
    };
    Ok((output, report))
}
