//! Experimental staff ability and reputation writes at fixed header offsets.
use crate::{
    Result, database,
    editor::{AbilityPolicy, Change},
    staff, staff_editor,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit {
    pub uid: i32,
    pub current_ability: Option<i16>,
    pub potential_ability: Option<i16>,
    pub reputation: Option<[i16; 3]>,
    pub policy: Option<AbilityPolicy>,
}

pub fn apply(
    input: &[u8],
    edits: &[Edit],
    attributes: &[staff_editor::Edit],
) -> Result<(Vec<u8>, Vec<Change>)> {
    if edits.is_empty() {
        return Err("Plano de cabeçalho de staff vazio".into());
    }
    let (db, base) = database::load(input)?;
    let profiles = staff::profiles(&db, input, base)?;
    let mut output = input.to_vec();
    let mut changes = Vec::new();
    let mut seen = BTreeSet::new();
    for edit in edits {
        if !seen.insert(edit.uid) {
            return Err("Staff repetido na edição de cabeçalho".into());
        }
        let p = profiles
            .iter()
            .find(|p| p.uid == edit.uid)
            .ok_or("Staff não encontrado")?;
        if edit.current_ability.is_none()
            && edit.potential_ability.is_none()
            && edit.reputation.is_none()
        {
            return Err("Edição de cabeçalho de staff vazia".into());
        }
        if edit.policy.is_some() && edit.current_ability.is_none() {
            return Err("Política de CA sem alvo de CA".into());
        }
        let mut write = |key: &str,
                         offset: usize,
                         before: i32,
                         after: i32,
                         raw_before: i32,
                         raw_after: i32,
                         width: u8,
                         derived: bool|
         -> Result<()> {
            let bytes = output
                .get_mut(offset..offset + usize::from(width))
                .ok_or("Cabeçalho de staff truncado")?;
            if width == 2 {
                bytes.copy_from_slice(&(raw_after as i16).to_le_bytes());
            } else {
                bytes[0] = raw_after as i8 as u8;
            }
            if raw_before != raw_after || before != after {
                changes.push(Change {
                    uid: p.uid,
                    name: p.name.clone(),
                    attribute: key.into(),
                    before,
                    after,
                    raw_before,
                    raw_after,
                    offset,
                    width,
                    derived,
                    context: Some("Staff".into()),
                    display_before: None,
                    display_after: None,
                });
            }
            Ok(())
        };
        if let Some(reputation) = edit.reputation {
            for (i, value) in reputation.into_iter().enumerate() {
                if !(0..=10000).contains(&value) {
                    return Err("Reputação de staff deve estar entre 0 e 10000".into());
                }
                write(
                    ["reputation.home", "reputation.current", "reputation.world"][i],
                    p.info_offset + i * 2,
                    i32::from(p.reputation[i]),
                    i32::from(value),
                    i32::from(p.reputation[i]),
                    i32::from(value),
                    2,
                    false,
                )?;
            }
        }
        if let Some(pa) = edit.potential_ability {
            if !(0..=200).contains(&pa) {
                return Err("PA fixa de staff deve estar entre 0 e 200".into());
            }
            write(
                "potentialAbility",
                p.info_offset + 8,
                i32::from(p.potential_ability),
                i32::from(pa),
                i32::from(p.potential_ability),
                i32::from(pa),
                2,
                false,
            )?;
        }
        if let Some(ca) = edit.current_ability {
            let policy = edit
                .policy
                .ok_or("Escolha uma política para a CA de staff")?;
            let mut targets = BTreeMap::new();
            for target in attributes.iter().filter(|a| a.uid == p.uid) {
                if targets
                    .insert(target.attribute.clone(), target.value)
                    .is_some()
                {
                    return Err("Campo de staff repetido na mesma operação".into());
                }
            }
            let projection = staff_editor::project_ability(p, ca, policy, &targets)?;
            let record = p.attributes_offset.ok_or("Staff sem atributos")?;
            if profiles
                .iter()
                .any(|other| other.uid != p.uid && other.attributes_offset == Some(record))
            {
                return Err("Registro de atributos de staff compartilhado".into());
            }
            write(
                "currentAbility",
                p.info_offset + 6,
                i32::from(p.current_ability),
                i32::from(ca),
                i32::from(p.current_ability),
                i32::from(ca),
                2,
                false,
            )?;
            for (i, key) in staff::FIELDS.iter().enumerate() {
                let before = projection.attributes_before[*key];
                let after = projection.attributes_after[*key];
                let raw_before = p.raw_attributes.as_ref().ok_or("Staff sem atributos")?[*key];
                let raw_after = projection.raw_attributes_after[*key];
                write(
                    key,
                    record + i,
                    i32::from(before),
                    i32::from(after),
                    i32::from(raw_before),
                    i32::from(raw_after),
                    1,
                    !targets.contains_key(*key),
                )?;
            }
        }
    }
    let allowed = changes
        .iter()
        .flat_map(|c| c.offset..c.offset + usize::from(c.width))
        .collect::<BTreeSet<_>>();
    if input.len() != output.len()
        || input
            .iter()
            .zip(&output)
            .enumerate()
            .any(|(offset, (a, b))| a != b && !allowed.contains(&offset))
    {
        return Err("Diferença binária fora do plano de cabeçalho de staff".into());
    }
    let (after_db, after_base) = database::load(&output)?;
    let after = staff::profiles(&after_db, &output, after_base)?;
    for edit in edits {
        let p = after
            .iter()
            .find(|p| p.uid == edit.uid)
            .ok_or("Staff perdido após edição")?;
        if edit.current_ability.is_some_and(|v| p.current_ability != v)
            || edit
                .potential_ability
                .is_some_and(|v| p.potential_ability != v)
            || edit.reputation.is_some_and(|v| p.reputation != v)
        {
            return Err("Releitura do cabeçalho de staff divergiu do plano".into());
        }
    }
    Ok((output, changes))
}
