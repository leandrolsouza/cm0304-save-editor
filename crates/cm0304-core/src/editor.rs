use crate::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{ErrorKind, Read, Write},
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
        if e.value == p.fields[e.attribute.as_str()].value && !relative(&e.attribute) {
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
        e.value != index[&e.uid].fields[e.attribute.as_str()].value
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
    pub context: Option<String>,
    pub display_before: Option<String>,
    pub display_after: Option<String>,
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
pub const CANCELLED: &str = "Operation cancelled";

fn cancelled(cancel: &AtomicBool) -> Result<()> {
    if cancel.load(Ordering::Relaxed) {
        Err(CANCELLED.into())
    } else {
        Ok(())
    }
}

/// Checks that a freshly opened file still contains the exact snapshot.
pub fn matches_snapshot(mut file: &File, expected: &[u8], cancel: &AtomicBool) -> Result<bool> {
    let mut buffer = [0; 64 * 1024];
    for chunk in expected.chunks(buffer.len()) {
        cancelled(cancel)?;
        let readback = &mut buffer[..chunk.len()];
        match file.read_exact(readback) {
            Ok(()) if readback == chunk => {}
            Ok(()) => return Ok(false),
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => return Ok(false),
            Err(error) => return Err(error.to_string()),
        }
    }
    cancelled(cancel)?;
    match file.read_exact(&mut buffer[..1]) {
        Ok(()) => Ok(false),
        Err(error) if error.kind() == ErrorKind::UnexpectedEof => Ok(true),
        Err(error) => Err(error.to_string()),
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
    save_transformed_snapshot(source, destination, input, cancel, |bytes| {
        apply(bytes, edits)
    })
}

#[cfg(windows)]
fn open_source_for_publication(path: &Path) -> std::io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_SHARE_READ: u32 = 1;
    File::options()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .open(path)
}

#[cfg(not(windows))]
fn open_source_for_publication(path: &Path) -> std::io::Result<File> {
    File::open(path)
}

pub(crate) fn save_transformed_snapshot<T>(
    source: &Path,
    destination: &Path,
    input: &[u8],
    cancel: &AtomicBool,
    transform: impl FnOnce(&[u8]) -> Result<(Vec<u8>, T)>,
) -> Result<T> {
    const DESTINATION_EXISTS: &str =
        "Já existe um arquivo no destino. Escolha um nome novo para a cópia.";
    cancelled(cancel)?;
    if destination
        .extension()
        .and_then(|v| v.to_str())
        .is_none_or(|v| !v.eq_ignore_ascii_case("cm4"))
    {
        return Err("O nome da cópia precisa terminar em .cm4.".into());
    }
    if destination
        .file_stem()
        .and_then(|s| s.to_str())
        .is_none_or(|s| s.encode_utf16().count() > 26)
    {
        return Err("O nome da cópia pode ter até 26 caracteres antes da extensão.".into());
    }
    if destination.exists() {
        return Err(DESTINATION_EXISTS.into());
    }
    if !matches_snapshot(
        &File::open(source).map_err(|e| e.to_string())?,
        input,
        cancel,
    )? {
        return Err(
            "O save de origem mudou desde a abertura. Reabra o arquivo antes de editar.".into(),
        );
    }
    let (output, report) = transform(input)?;
    cancelled(cancel)?;
    let parent = destination
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    for chunk in output.chunks(1024 * 1024) {
        cancelled(cancel)?;
        temp.write_all(chunk).map_err(|e| e.to_string())?;
    }
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    if !matches_snapshot(
        &File::open(temp.path()).map_err(|e| e.to_string())?,
        &output,
        cancel,
    )? {
        return Err("A verificação da cópia temporária falhou.".into());
    }
    drop(output);
    // On Windows the open handle excludes writes and renames through publication.
    let source_handle = open_source_for_publication(source).map_err(|error| {
        if cfg!(windows) && matches!(error.raw_os_error(), Some(32 | 33)) {
            "O save de origem está em uso por outro processo. Feche o arquivo nesse programa e tente novamente.".into()
        } else {
            format!("Não foi possível conferir o save de origem: {error}")
        }
    })?;
    if !matches_snapshot(&source_handle, input, cancel)? {
        return Err(
            "O save de origem mudou durante a gravação. Reabra o arquivo antes de editar.".into(),
        );
    }
    cancelled(cancel)?;
    temp.persist_noclobber(destination).map_err(|error| {
        if error.error.kind() == std::io::ErrorKind::AlreadyExists {
            DESTINATION_EXISTS.into()
        } else {
            format!("Não foi possível publicar a cópia: {error}")
        }
    })?;
    drop(source_handle);
    Ok(report)
}

#[cfg(test)]
mod publication_tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn publication_handle_allows_reads_and_blocks_source_mutation() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.cm4");
        let renamed = directory.path().join("renamed.cm4");
        fs::write(&source, b"original").unwrap();

        let handle = open_source_for_publication(&source).unwrap();
        assert_eq!(fs::read(&source).unwrap(), b"original");
        assert!(fs::write(&source, b"modified").is_err());
        assert!(fs::rename(&source, &renamed).is_err());
        assert_eq!(fs::read(&source).unwrap(), b"original");
        drop(handle);

        fs::rename(&source, &renamed).unwrap();
        fs::write(&renamed, b"modified").unwrap();
    }

    #[test]
    fn cancellation_after_transform_leaves_no_destination() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.cm4");
        let destination = directory.path().join("copy.cm4");
        fs::write(&source, b"original").unwrap();
        let cancel = AtomicBool::new(false);
        let result = save_transformed_snapshot(&source, &destination, b"original", &cancel, |_| {
            cancel.store(true, Ordering::Relaxed);
            Ok((b"edited".to_vec(), ()))
        });
        assert_eq!(result.unwrap_err(), CANCELLED);
        assert_eq!(fs::read(&source).unwrap(), b"original");
        assert!(!destination.exists());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }

    #[cfg(windows)]
    #[test]
    fn busy_source_can_be_retried_without_leaving_a_partial_copy() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.cm4");
        let destination = directory.path().join("copy.cm4");
        fs::write(&source, b"original").unwrap();
        let writer = File::options().write(true).open(&source).unwrap();
        let cancel = AtomicBool::new(false);
        let transform = |_: &[u8]| Ok((b"edited".to_vec(), ()));
        let result =
            save_transformed_snapshot(&source, &destination, b"original", &cancel, transform);
        assert!(result.unwrap_err().contains("em uso por outro processo"));
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);

        drop(writer);
        save_transformed_snapshot(&source, &destination, b"original", &cancel, |_| {
            Ok((b"edited".to_vec(), ()))
        })
        .unwrap();
        assert_eq!(fs::read(&source).unwrap(), b"original");
        assert_eq!(fs::read(&destination).unwrap(), b"edited");
    }

    #[test]
    fn preserves_destination_created_during_edit() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.cm4");
        let destination = directory.path().join("copy.cm4");
        fs::write(&source, b"original").unwrap();

        let result = save_transformed_snapshot(
            &source,
            &destination,
            b"original",
            &AtomicBool::new(false),
            |_| {
                fs::write(&destination, b"other writer").unwrap();
                Ok((b"edited".to_vec(), ()))
            },
        );

        assert!(result.unwrap_err().contains("Escolha um nome novo"));
        assert_eq!(fs::read(&source).unwrap(), b"original");
        assert_eq!(fs::read(&destination).unwrap(), b"other writer");
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 2);
    }

    #[test]
    fn discards_temp_copy_when_source_changes_during_edit() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.cm4");
        let destination = directory.path().join("copy.cm4");
        fs::write(&source, b"original").unwrap();

        let result = save_transformed_snapshot(
            &source,
            &destination,
            b"original",
            &AtomicBool::new(false),
            |_| {
                fs::write(&source, b"modified").unwrap();
                Ok((b"edited".to_vec(), ()))
            },
        );

        assert!(result.unwrap_err().contains("mudou durante a gravação"));
        assert_eq!(fs::read(&source).unwrap(), b"modified");
        assert!(!destination.exists());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }
}
