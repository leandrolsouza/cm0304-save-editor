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
    pub heal_injuries: BTreeSet<i32>,
    #[serde(default)]
    pub clear_suspensions: BTreeSet<i32>,
}
impl Transaction {
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty() && self.heal_injuries.is_empty() && self.clear_suspensions.is_empty()
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
    pub removals: Vec<health::Removal>,
}
pub fn apply(input: &[u8], plan: &Transaction) -> Result<(Vec<u8>, Report)> {
    if plan.is_empty() {
        return Err("Plano de alterações vazio".into());
    }
    // Fixed fields are applied first, while their source offsets still hold.
    let (fields_output, field_changed_bytes, changes) = if plan.edits.is_empty() {
        (input.to_vec(), 0, Vec::new())
    } else {
        let (bytes, report) = editor::apply(input, &plan.edits)?;
        (bytes, report.changed_bytes, report.changes)
    };
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
