use crate::{
    Result,
    attributes::{self, DIRECT, FIELDS},
    container::hash,
    database,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    let mut seen = BTreeSet::new();
    let mut patches = BTreeMap::new();
    let mut changes = Vec::new();
    for edit in edits {
        if !seen.insert((edit.uid, &edit.attribute)) {
            return Err(format!(
                "Duplicate field {} for UID {}",
                edit.attribute, edit.uid
            ));
        }
        if !(1..=20).contains(&edit.value) {
            return Err("Attribute must be 1–20".into());
        }
        let p = by_uid
            .get(&edit.uid)
            .ok_or_else(|| format!("Unknown player UID {}", edit.uid))?;
        let i = FIELDS
            .iter()
            .position(|f| *f == edit.attribute)
            .ok_or("Unknown attribute")?;
        let mode = attributes::mode(i, p.raw_attributes["goalkeeper"]);
        if mode.is_none() && !DIRECT.contains(&edit.attribute.as_str()) {
            return Err(format!("Unsupported attribute {}", edit.attribute));
        }
        let person = db
            .people
            .iter()
            .find(|v| v.uid == edit.uid && v.playing.is_some())
            .ok_or("Missing person")?;
        let index = person.playing.as_ref().unwrap().playing_index;
        if db
            .people
            .iter()
            .filter(|v| v.playing.as_ref().is_some_and(|d| d.playing_index == index))
            .count()
            != 1
        {
            return Err("Shared playing data cannot be edited".into());
        }
        let before = p.attributes[&edit.attribute];
        if before == edit.value {
            return Err(format!("No change for {} / {}", edit.uid, edit.attribute));
        }
        let raw_before = p.raw_attributes[&edit.attribute];
        let raw_after = match mode {
            Some(low) => attributes::encode(edit.value, p.current_ability, low, raw_before)?,
            None => edit.value,
        };
        let offset = p.offsets.attributes + i;
        if patches.insert(offset, raw_after as u8).is_some() {
            return Err("Overlapping edit offsets".into());
        }
        changes.push(Change {
            uid: edit.uid,
            name: p.name.clone(),
            attribute: edit.attribute.clone(),
            before,
            after: edit.value,
            raw_before,
            raw_after,
            offset,
        });
    }
    let mut output = input.to_vec();
    for (&offset, &value) in &patches {
        output[offset] = value;
    }
    let (verified, verified_base) = database::load(&output)?;
    let verified_players = verified.players(verified_base)?;
    for change in &changes {
        let p = verified_players
            .iter()
            .find(|p| p.uid == change.uid)
            .ok_or("Edited player missing")?;
        if p.attributes[&change.attribute] != change.after {
            return Err("Post-write value mismatch".into());
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
