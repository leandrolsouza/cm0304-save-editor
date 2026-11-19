//! One reviewable transaction combining field edits and structural operations.
use crate::{
    Result,
    container::hash,
    database::Player,
    editor::{self, Edit},
    health,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::atomic::AtomicBool,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(default)]
    pub edits: Vec<Edit>,
    #[serde(default)]
    pub staff_edits: Vec<crate::staff_editor::Edit>,
    #[serde(default)]
    pub staff_header_edits: Vec<crate::staff_header::Edit>,
    #[serde(default)]
    pub heal_injuries: BTreeSet<i32>,
    #[serde(default)]
    pub clear_suspensions: BTreeSet<i32>,
}
impl Transaction {
    /// Merge a staff form atomically against the source snapshot. Final byte validation
    /// remains in apply(), where CA and explicit attribute targets are resolved together.
    pub fn merge_staff(
        &mut self,
        profiles: &[crate::staff::Staff],
        edits: Vec<crate::staff_editor::Edit>,
        header: Option<crate::staff_header::Edit>,
    ) -> Result<()> {
        let mut next = self.clone();
        if let Some(update) = header {
            let p = profiles
                .iter()
                .find(|p| p.uid == update.uid)
                .ok_or("Staff não encontrado")?;
            let mut combined = next
                .staff_header_edits
                .iter()
                .find(|e| e.uid == update.uid)
                .cloned()
                .unwrap_or(crate::staff_header::Edit {
                    uid: update.uid,
                    ..Default::default()
                });
            if let Some(ca) = update.current_ability {
                combined.current_ability = (ca != p.current_ability).then_some(ca);
                combined.policy = if combined.current_ability.is_some() {
                    Some(
                        update
                            .policy
                            .ok_or("Escolha uma política para a CA de staff")?,
                    )
                } else {
                    None
                };
            } else if update.policy.is_some() {
                if combined.current_ability.is_none() {
                    return Err("Política de CA sem alvo de CA".into());
                }
                combined.policy = update.policy;
            }
            if let Some(pa) = update.potential_ability {
                combined.potential_ability = (pa != p.potential_ability).then_some(pa);
            }
            if let Some(rep) = update.reputation {
                combined.reputation = (rep != p.reputation).then_some(rep);
            }
            next.staff_header_edits.retain(|e| e.uid != update.uid);
            if combined.current_ability.is_some()
                || combined.potential_ability.is_some()
                || combined.reputation.is_some()
            {
                next.staff_header_edits.push(combined);
            }
        }
        let dependencies = profiles
            .iter()
            .filter(|p| {
                if p.is_player {
                    next.edits
                        .iter()
                        .any(|e| e.uid == p.uid && e.attribute == "currentAbility")
                } else {
                    next.staff_header_edits
                        .iter()
                        .any(|h| h.uid == p.uid && h.current_ability.is_some())
                }
            })
            .map(|p| p.uid)
            .collect::<BTreeSet<_>>();
        let mut fields = next
            .staff_edits
            .iter()
            .cloned()
            .map(|e| ((e.uid, e.attribute.clone()), e))
            .collect::<BTreeMap<_, _>>();
        let mut seen = BTreeSet::new();
        for edit in edits {
            if !seen.insert((edit.uid, edit.attribute.clone())) {
                return Err("Campo de staff repetido".into());
            }
            if !crate::staff_editor::EDITABLE.contains(&edit.attribute.as_str())
                || !(1..=20).contains(&edit.value)
            {
                return Err("Campo ou valor de staff inválido".into());
            }
            let p = profiles
                .iter()
                .find(|p| p.uid == edit.uid)
                .ok_or("Staff não encontrado")?;
            let original = p
                .attributes
                .as_ref()
                .and_then(|a| a.get(&edit.attribute))
                .ok_or("Staff sem atributo")?;
            let dependent = crate::staff::RELATIVE.contains(&edit.attribute.as_str())
                && dependencies.contains(&edit.uid);
            let key = (edit.uid, edit.attribute.clone());
            if edit.value == *original && !dependent {
                fields.remove(&key);
            } else {
                fields.insert(key, edit);
            }
        }
        fields.retain(|_, e| {
            profiles
                .iter()
                .find(|p| p.uid == e.uid)
                .and_then(|p| p.attributes.as_ref())
                .and_then(|a| a.get(&e.attribute))
                .is_none_or(|v| *v != e.value)
                || (crate::staff::RELATIVE.contains(&e.attribute.as_str())
                    && dependencies.contains(&e.uid))
        });
        next.staff_edits = fields.into_values().collect();
        next.staff_header_edits.sort_by_key(|h| h.uid);
        *self = next;
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
            && self.staff_edits.is_empty()
            && self.staff_header_edits.is_empty()
            && self.heal_injuries.is_empty()
            && self.clear_suspensions.is_empty()
    }
    pub fn merge_fields(&mut self, players: &[Player], edits: Vec<Edit>) -> Result<()> {
        let previous = self
            .edits
            .iter()
            .cloned()
            .map(|e| ((e.uid, e.attribute.clone()), e))
            .collect::<BTreeMap<_, _>>();
        self.edits = editor::merge_plan(players, &previous, edits)?
            .into_values()
            .collect();
        Ok(())
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub source_sha256: String,
    pub copy_sha256: String,
    pub size_before: usize,
    pub size_after: usize,
    pub field_changed_bytes: usize,
    pub removed_bytes: usize,
    pub changes: Vec<editor::Change>,
    pub staff_changes: Vec<crate::staff_editor::Change>,
    pub removals: Vec<health::Removal>,
}
pub fn apply(input: &[u8], plan: &Transaction) -> Result<(Vec<u8>, Report)> {
    if plan.is_empty() {
        return Err("Plano de alterações vazio".into());
    }
    // Fixed fields are applied first, while their source offsets still hold.
    let (fields_output, _, mut changes) = if plan.edits.is_empty() {
        (input.to_vec(), 0, Vec::new())
    } else {
        let (bytes, report) = editor::apply(input, &plan.edits)?;
        (bytes, report.changed_bytes, report.changes)
    };
    let (fields_output, header_changes) = if plan.staff_header_edits.is_empty() {
        (fields_output, Vec::new())
    } else {
        crate::staff_header::apply(&fields_output, &plan.staff_header_edits, &plan.staff_edits)?
    };
    changes.extend(header_changes);
    let remaining_staff = plan
        .staff_edits
        .iter()
        .filter(|e| {
            !plan
                .staff_header_edits
                .iter()
                .any(|h| h.uid == e.uid && h.current_ability.is_some())
        })
        .cloned()
        .collect::<Vec<_>>();
    let (fields_output, staff_changes) = if remaining_staff.is_empty() {
        (fields_output, Vec::new())
    } else {
        crate::staff_editor::apply(&fields_output, &remaining_staff)?
    };
    let field_changed_bytes = input
        .iter()
        .zip(&fields_output)
        .filter(|(a, b)| a != b)
        .count();
    let selections = plan
        .heal_injuries
        .iter()
        .map(|uid| (*uid, health::RemovalKind::Injuries))
        .chain(
            plan.clear_suspensions
                .iter()
                .map(|uid| (*uid, health::RemovalKind::Suspensions)),
        )
        .collect::<Vec<_>>();
    let (output, removals) = if selections.is_empty() {
        (fields_output, Vec::new())
    } else {
        let (bytes, report) = health::remove_records(&fields_output, &selections)?;
        (bytes, report.removals)
    };
    let report = Report {
        source_sha256: hash(input),
        copy_sha256: hash(&output),
        size_before: input.len(),
        size_after: output.len(),
        field_changed_bytes,
        removed_bytes: removals.iter().map(|r| r.removed_bytes).sum(),
        changes,
        staff_changes,
        removals,
    };
    Ok((output, report))
}
pub fn save_snapshot(
    source: &Path,
    destination: &Path,
    plan: &Transaction,
    input: &[u8],
    cancel: &AtomicBool,
) -> Result<Report> {
    editor::save_transformed_snapshot(source, destination, input, cancel, |bytes| {
        apply(bytes, plan)
    })
}
