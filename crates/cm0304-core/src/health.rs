//! Structural removal of injury and suspension records, preserving booking counts.
use crate::{Result, container, database};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RemovalKind {
    Injuries,
    Suspensions,
}
impl RemovalKind {
    fn records(self, health: &database::HealthRecords) -> &[database::RecordSpan] {
        match self {
            Self::Injuries => &health.injuries,
            Self::Suspensions => &health.bans,
        }
    }
    fn bounds(self, health: &database::HealthRecords) -> Option<(usize, usize)> {
        match self {
            Self::Injuries => Some((health.injury_count_offset?, health.ban_count_offset?)),
            Self::Suspensions => {
                Some((health.ban_count_offset?, health.injury_status_flag_offset?))
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InjuryView {
    pub name: String,
}
pub fn injuries(db: &database::Database, person: &database::Person) -> Vec<InjuryView> {
    person
        .playing
        .as_ref()
        .map(|p| {
            p.health_records
                .injuries
                .iter()
                .map(|record| {
                    let index = i32::from_le_bytes(record.bytes[..4].try_into().unwrap());
                    InjuryView {
                        name: db
                            .injury_types
                            .get(index as usize)
                            .map(|v| v.name.clone())
                            .unwrap_or_else(|| "Lesão não identificada".into()),
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Removal {
    pub kind: RemovalKind,
    pub uid: i32,
    pub name: String,
    pub count: usize,
    pub count_offset: usize,
    pub offset: usize,
    pub removed_bytes: usize,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub source_sha256: String,
    pub copy_sha256: String,
    pub size_before: usize,
    pub size_after: usize,
    pub removals: Vec<Removal>,
}

/// Applies all selected removals atomically in memory, preserving status,
/// contracts, booking counts and unrelated payloads. Healthy players are no-ops.
pub fn remove_injuries(input: &[u8], uids: &[i32]) -> Result<(Vec<u8>, Report)> {
    remove_records(
        input,
        &uids
            .iter()
            .map(|uid| (*uid, RemovalKind::Injuries))
            .collect::<Vec<_>>(),
    )
}
pub fn remove_records(
    input: &[u8],
    selections: &[(i32, RemovalKind)],
) -> Result<(Vec<u8>, Report)> {
    if selections.is_empty() {
        return Err("Nenhum jogador selecionado".into());
    }
    let selected = selections.iter().copied().collect::<BTreeSet<_>>();
    if selected.len() != selections.len() {
        return Err("Jogador repetido na operação".into());
    }
    let (db, base) = database::load(input)?;
    let players = db.players(base)?;
    let names = players
        .iter()
        .map(|p| (p.uid, p.name.as_str()))
        .collect::<BTreeMap<_, _>>();
    if names.len() != players.len() {
        return Err("UID de jogador duplicado".into());
    }
    let container = container::read_container(input)?;
    let block = container
        .blocks
        .iter()
        .find(|b| b.name == "game_db" && b.extension == ".dat")
        .ok_or("Banco não encontrado")?;
    let mut payload = input[base..base + block.size].to_vec();
    let mut removals = Vec::new();
    for (uid, kind) in selected {
        let name = names
            .get(&uid)
            .ok_or_else(|| format!("Jogador não encontrado: {uid}"))?;
        let person = db
            .people
            .iter()
            .find(|p| p.uid == uid && p.playing.is_some())
            .ok_or("Dados do jogador ausentes")?;
        let health = &person.playing.as_ref().unwrap().health_records;
        let records = kind.records(health);
        if records.is_empty() {
            continue;
        }
        let (count_offset, end) = kind
            .bounds(health)
            .ok_or("Limites dos registros ausentes")?;
        let start = count_offset + 1;
        let length = records.iter().map(|r| r.bytes.len()).sum::<usize>();
        if start + length != end
            || payload[count_offset] as usize != records.len()
            || records.first().is_none_or(|r| r.offset != start)
        {
            return Err("Limites dos registros inconsistentes".into());
        }
        removals.push(Removal {
            kind,
            uid,
            name: (*name).to_owned(),
            count: records.len(),
            count_offset: base + count_offset,
            offset: base + start,
            removed_bytes: length,
        });
    }
    // Patch counters before draining: all offsets still refer to the source.
    for removal in &removals {
        payload[removal.count_offset - base] = 0;
    }
    removals.sort_by_key(|r| r.offset);
    for removal in removals.iter().rev() {
        let start = removal.offset - base;
        payload.drain(start..start + removal.removed_bytes);
    }
    let output = if removals.is_empty() {
        input.to_vec()
    } else {
        container::replace_block(input, "game_db", ".dat", &payload)?
    };
    let (after, new_base) = database::load(&output)?;
    let after_players = after.players(new_base)?;
    if players.len() != after_players.len() || db.people.len() != after.people.len() {
        return Err("Quantidade de entidades alterada ao remover registros".into());
    }
    for (before, after) in players.iter().zip(&after_players) {
        if before.uid != after.uid
            || before.id != after.id
            || before.kind != after.kind
            || before.name != after.name
            || before.first_name != after.first_name
            || before.surname != after.surname
            || before.common_name != after.common_name
            || before.club != after.club
            || before.nation != after.nation
            || before.attributes != after.attributes
            || before.raw_attributes != after.raw_attributes
            || before.personality != after.personality
            || before.fields.len() != after.fields.len()
            || before.fields.iter().any(|(key, value)| {
                after.fields.get(key).is_none_or(|v| {
                    (v.value, v.width, v.min, v.max)
                        != (value.value, value.width, value.min, value.max)
                })
            })
        {
            return Err(format!(
                "Valores do jogador {} mudaram durante a remoção",
                before.uid
            ));
        }
    }
    for removal in &removals {
        let health = &after
            .people
            .iter()
            .find(|p| p.uid == removal.uid)
            .ok_or("Jogador ausente após remoção")?
            .playing
            .as_ref()
            .ok_or("Dados ausentes após remoção")?
            .health_records;
        if !removal.kind.records(health).is_empty() {
            return Err("Registros não foram removidos".into());
        }
    }
    let report = Report {
        source_sha256: container::hash(input),
        copy_sha256: container::hash(&output),
        size_before: input.len(),
        size_after: output.len(),
        removals,
    };
    Ok((output, report))
}
