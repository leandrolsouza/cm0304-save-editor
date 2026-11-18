use crate::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Edit {
    pub uid: i32,
    pub attribute: String,
    pub value: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<AbilityPolicy>,
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AbilityPolicy {
    PreserveValues,
    PreserveRelative,
}
pub type Plan = BTreeMap<(i32, String), Edit>;

/// Merge a form submission against source values. An explicit relative target
/// equal to the source may still override a pending CA/goalkeeper recalculation.
/// Once that dependency is removed, redundant source-value targets disappear.
pub fn merge_plan(
    players: &[crate::database::Player],
    previous: &Plan,
    edits: Vec<Edit>,
) -> Result<Plan> {
    let index = players
        .iter()
        .map(|p| (p.uid, p))
        .collect::<BTreeMap<_, _>>();
    let mut next = previous.clone();
    let mut seen = BTreeSet::new();
    let relative = |key: &str| crate::attributes::FIELDS[12..37].contains(&key);
    for e in edits {
        let p = index.get(&e.uid).ok_or("Jogador não encontrado")?;
        crate::extended::validate_value(p, &e)?;
        let key = (e.uid, e.attribute.clone());
        if !seen.insert(key.clone()) {
            return Err("Campo repetido na mesma operação".into());
        }
        if e.value == p.fields[&e.attribute].value && !relative(&e.attribute) {
            next.remove(&key);
        } else {
            next.insert(key, e);
        }
    }
    let dependencies = next
        .values()
        .filter(|e| e.attribute == "currentAbility" || e.attribute == "position.goalkeeper")
        .map(|e| e.uid)
        .collect::<BTreeSet<_>>();
    next.retain(|_, e| {
        e.value != index[&e.uid].fields[&e.attribute].value
            || (relative(&e.attribute) && dependencies.contains(&e.uid))
    });
    Ok(next)
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub uid: i32,
    pub name: String,
    pub attribute: String,
    pub before: i32,
    pub after: i32,
    pub raw_before: i32,
    pub raw_after: i32,
    pub offset: usize,
    pub width: u8,
    pub derived: bool,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub source_sha256: String,
    pub copy_sha256: String,
    pub size: usize,
    pub changed_bytes: usize,
    pub changes: Vec<Change>,
}
pub fn apply(input: &[u8], edits: &[Edit]) -> Result<(Vec<u8>, Report)> {
    crate::extended::apply(input, edits)
}
fn cancelled(cancel: &AtomicBool) -> Result<()> {
    if cancel.load(Ordering::Relaxed) {
        Err("Operation cancelled".into())
    } else {
        Ok(())
    }
}
pub fn save_copy(
    source: &Path,
    destination: &Path,
    edits: &[Edit],
    cancel: &AtomicBool,
) -> Result<Report> {
    let input = fs::read(source).map_err(|e| e.to_string())?;
    save_snapshot(source, destination, edits, &input, cancel)
}

/// Saves edits against the exact snapshot the user opened, rejecting changes to
/// the source made while an editing session was in progress.
pub fn save_snapshot(
    source: &Path,
    destination: &Path,
    edits: &[Edit],
    input: &[u8],
    cancel: &AtomicBool,
) -> Result<Report> {
    cancelled(cancel)?;
    if destination
        .extension()
        .and_then(|v| v.to_str())
        .is_none_or(|v| !v.eq_ignore_ascii_case("cm4"))
    {
        return Err("Destination must end in .cm4".into());
    }
    if destination
        .file_stem()
        .and_then(|s| s.to_str())
        .is_none_or(|s| s.encode_utf16().count() > 26)
    {
        return Err("Save name must have at most 26 characters".into());
    }
    if destination.exists() {
        return Err(
            "Destination already exists; original and existing copies cannot be overwritten".into(),
        );
    }
    if fs::read(source).map_err(|e| e.to_string())? != input {
        return Err("Source changed since it was opened".into());
    }
    let (output, report) = apply(input, edits)?;
    cancelled(cancel)?;
    let parent = destination
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(&output).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    let mut readback = Vec::new();
    File::open(temp.path())
        .and_then(|mut f| f.read_to_end(&mut readback))
        .map_err(|e| e.to_string())?;
    if readback != output {
        return Err("Temporary copy verification failed".into());
    }
    if fs::read(source).map_err(|e| e.to_string())? != input {
        return Err("Source changed during editing".into());
    }
    cancelled(cancel)?;
    temp.persist_noclobber(destination)
        .map_err(|e| e.to_string())?;
    Ok(report)
}
