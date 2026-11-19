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
