//! Fixed-size staff attribute edits, independent of player ability and contracts.
use crate::{Result, container, database, staff};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edit {
    pub uid: i32,
    pub attribute: String,
    pub value: i8,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub uid: i32,
    pub name: String,
    pub attribute: String,
    pub before: i8,
    pub after: i8,
    pub raw_before: i8,
    pub raw_after: i8,
    pub offset: usize,
}

/// Professional ratings whose meaning and scale have been checked in the game.
pub const EDITABLE: [&str; 11] = [
    "coachingOutfieldPlayers",
    "coachingGoalkeepers",
    "judgingPlayerAbility",
    "judgingPlayerPotential",
    "manManagement",
    "motivating",
    "physiotherapy",
    "tacticalKnowledge",
    "workingWithYoungsters",
    "determination",
    "discipline",
];

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityProjection {
    pub current_ability_before: i16,
    pub current_ability_after: i16,
    pub attribute_ability_before: i16,
    pub attribute_ability_after: i16,
    pub attributes_before: BTreeMap<String, i8>,
    pub attributes_after: BTreeMap<String, i8>,
    pub raw_attributes_after: BTreeMap<String, i8>,
}

/// Compute CA effects before any write. Explicit targets override either policy.
/// This does not change PA or the ability of the player's other role.
pub fn project_ability(
    profile: &staff::Staff,
    target_ca: i16,
    policy: crate::editor::AbilityPolicy,
    targets: &BTreeMap<String, i8>,
) -> Result<AbilityProjection> {
    if !(0..=200).contains(&target_ca) {
        return Err("CA de staff deve estar entre 0 e 200".into());
    }
    let original_raw = profile
        .raw_attributes
        .as_ref()
        .ok_or("Staff sem atributos brutos")?;
    let before = staff::decoded(original_raw, staff::attribute_ability(profile))?;
    let effective_ca = profile.player_current_ability.unwrap_or(target_ca);
    let mut raw = original_raw.clone();
    for key in targets.keys() {
        if !EDITABLE.contains(&key.as_str()) {
            return Err(format!("Campo de staff sem edição suportada: {key}"));
        }
    }
    for key in staff::FIELDS {
        let explicit = targets.get(key).copied();
        let preserve = policy == crate::editor::AbilityPolicy::PreserveValues
            && staff::RELATIVE.contains(&key);
        if let Some(value) = explicit.or_else(|| preserve.then_some(before[key])) {
            let encoded = staff::encode_attribute(key, value, effective_ca, original_raw[key])
                .map_err(|error| format!("Staff {} · {key}: {error}", profile.uid))?;
            raw.insert(key.to_owned(), encoded);
        }
    }
    Ok(AbilityProjection {
        current_ability_before: profile.current_ability,
        current_ability_after: target_ca,
        attribute_ability_before: staff::attribute_ability(profile),
        attribute_ability_after: effective_ca,
        attributes_before: before,
        attributes_after: staff::decoded(&raw, effective_ca)?,
        raw_attributes_after: raw,
    })
}

/// Merge intentions against the original snapshot; failures never mutate the old plan.
pub fn merge_plan(
    profiles: &[staff::Staff],
    previous: &[Edit],
    edits: Vec<Edit>,
) -> Result<Vec<Edit>> {
    let index = profiles
        .iter()
        .map(|p| (p.uid, p))
        .collect::<BTreeMap<_, _>>();
    let mut next = previous
        .iter()
        .cloned()
        .map(|e| ((e.uid, e.attribute.clone()), e))
        .collect::<BTreeMap<_, _>>();
    if next.len() != previous.len() {
        return Err("Plano de staff contém campos repetidos".into());
    }
    let mut seen = BTreeSet::new();
    for edit in edits {
        let key = (edit.uid, edit.attribute.clone());
        if !seen.insert(key.clone()) {
            return Err("Campo de staff repetido na mesma operação".into());
        }
        if !EDITABLE.contains(&edit.attribute.as_str()) {
            return Err("Campo de staff sem edição suportada".into());
        }
        let profile = index.get(&edit.uid).ok_or("Staff não encontrado")?;
        let original = profile
            .attributes
            .as_ref()
            .and_then(|a| a.get(&edit.attribute))
            .ok_or("Staff sem atributo disponível")?;
        let raw = profile
            .raw_attributes
            .as_ref()
            .and_then(|a| a.get(&edit.attribute))
            .ok_or("Staff sem atributo bruto disponível")?;
        let record = profile.attributes_offset.ok_or("Staff sem atributos")?;
        if profiles
            .iter()
            .any(|p| p.uid != edit.uid && p.attributes_offset == Some(record))
        {
            return Err("Registro de atributos de staff compartilhado".into());
        }
        staff::encode_attribute(
            &edit.attribute,
            edit.value,
            staff::attribute_ability(profile),
            *raw,
        )?;
        if edit.value == *original {
            next.remove(&key);
        } else {
            next.insert(key, edit);
        }
    }
    Ok(next.into_values().collect())
}

pub fn apply(input: &[u8], edits: &[Edit]) -> Result<(Vec<u8>, Vec<Change>)> {
    if edits.is_empty() {
        return Err("Plano de staff vazio".into());
    }
    let (db, base) = database::load(input)?;
    let profiles = staff::profiles(&db, input, base)?;
    let index = profiles
        .iter()
        .map(|p| (p.uid, p))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut output = input.to_vec();
    let mut changes = Vec::new();
    for edit in edits {
        if !seen.insert((edit.uid, edit.attribute.as_str())) {
            return Err("Campo de staff repetido na mesma operação".into());
        }
        if !EDITABLE.contains(&edit.attribute.as_str()) {
            return Err(format!(
                "Campo de staff sem edição suportada: {}",
                edit.attribute
            ));
        }
        let profile = index.get(&edit.uid).ok_or("Staff não encontrado")?;
        let record = profile.attributes_offset.ok_or("Staff sem atributos")?;
        if profiles
            .iter()
            .any(|p| p.uid != edit.uid && p.attributes_offset == Some(record))
        {
            return Err("Registro de atributos de staff compartilhado".into());
        }
        let field = staff::FIELDS
            .iter()
            .position(|k| *k == edit.attribute)
            .ok_or("Campo de staff desconhecido")?;
        let offset = record
            .checked_add(field)
            .ok_or("Offset de staff inválido")?;
        let raw_before = *input.get(offset).ok_or("Atributo de staff truncado")? as i8;
        let before = profile
            .attributes
            .as_ref()
            .ok_or("Staff sem atributos convertidos")?[&edit.attribute];
        let raw_after = staff::encode_attribute(
            &edit.attribute,
            edit.value,
            staff::attribute_ability(profile),
            raw_before,
        )?;
        output[offset] = raw_after as u8;
        if raw_after != raw_before {
            changes.push(Change {
                uid: edit.uid,
                name: profile.name.clone(),
                attribute: edit.attribute.clone(),
                before,
                after: edit.value,
                raw_before,
                raw_after,
                offset,
            });
        }
    }
    // Reparse the entire staff table and all players, then verify requested ratings.
    let (after_db, after_base) = database::load(&output)?;
    let after = staff::profiles(&after_db, &output, after_base)?;
    for edit in edits {
        let profile = after
            .iter()
            .find(|p| p.uid == edit.uid)
            .ok_or("Staff perdido após edição")?;
        if profile
            .attributes
            .as_ref()
            .ok_or("Atributos perdidos após edição")?[&edit.attribute]
            != edit.value
        {
            return Err("Releitura de staff divergiu do plano".into());
        }
    }
    if db.players(base)?.len() != after_db.players(after_base)?.len() {
        return Err("Edição de staff alterou quantidade de jogadores".into());
    }
    let allowed = changes.iter().map(|c| c.offset).collect::<BTreeSet<_>>();
    if input.len() != output.len()
        || input
            .iter()
            .zip(&output)
            .enumerate()
            .any(|(offset, (a, b))| a != b && !allowed.contains(&offset))
    {
        return Err("Diferença binária fora do plano de staff".into());
    }
    // Preserve the container and all blocks other than the edited database.
    let before_blocks = container::read_container(input)?;
    let after_blocks = container::read_container(&output)?;
    for (before, after) in before_blocks.blocks.iter().zip(&after_blocks.blocks) {
        if before.payload_offset != base && before.sha256 != after.sha256 {
            return Err("Edição de staff alterou outro bloco".into());
        }
    }
    Ok((output, changes))
}
