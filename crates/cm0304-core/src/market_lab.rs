//! Experiments with the native market edit journal.
use crate::{
    Result,
    container::{find_block, hash, replace_block},
    database::{self, Contract, Database, Person},
    editor,
};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::atomic::AtomicBool,
};

const SESSION: &str = "edit_session_man";
const TRANSFERS: &str = "transfer_man";
const ENTRY_SIZE: usize = 65;
const TRANSFER_STATUS_OFFSET: usize = 14;
const SESSION_HEADER: [u8; 12] = [0, b'.', 0, b'd', 0, b'a', 0, b't', 0, 15, 0, 0];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    player_uid: i32,
    before: u8,
    after: u8,
    source_sha256: String,
    copy_sha256: String,
}

pub fn save(source: &Path, copy: &Path, donor: &Path, uid: i32) -> Result<Report> {
    let input = std::fs::read(source).map_err(|e| e.to_string())?;
    let donor = std::fs::read(donor).map_err(|e| e.to_string())?;
    editor::save_transformed_snapshot(source, copy, &input, &AtomicBool::new(false), |bytes| {
        replay(bytes, &donor, uid)
    })
}

pub fn synthesize(source: &Path, copy: &Path, uid: i32, after: u8) -> Result<Report> {
    let input = std::fs::read(source).map_err(|e| e.to_string())?;
    editor::save_transformed_snapshot(source, copy, &input, &AtomicBool::new(false), |bytes| {
        let (output, changes) = apply(bytes, &BTreeMap::from([(uid, after)]))?;
        let report = Report {
            player_uid: uid,
            before: changes[0].before as u8,
            after,
            source_sha256: hash(bytes),
            copy_sha256: hash(&output),
        };
        Ok((output, report))
    })
}

pub(crate) fn apply(
    bytes: &[u8],
    targets: &BTreeMap<i32, u8>,
) -> Result<(Vec<u8>, Vec<editor::Change>)> {
    let (db, base) = database::load(bytes)?;
    let mut updated_session = market_journal(bytes)?.to_vec();
    let managers = club_managers(&db);
    let mut edited = bytes.to_vec();
    let mut changes = Vec::with_capacity(targets.len());
    for (&uid, &after) in targets {
        let (person, contract) = player_contract(&db, uid)?;
        let before = contract.transfer_status;
        if !observed_transition(before, after)
            || db.clubs.get(contract.entity_index as usize).is_none()
        {
            return Err("Vínculo ou transição de mercado fora dos casos observados".into());
        }
        let manager = market_manager(&db, &managers, person, contract)
            .ok_or("O vínculo precisa pertencer a um clube com um único treinador humano na equipe principal")?;
        let estimated = person.playing.as_ref().unwrap().state.estimated_value;
        // The native Braga entry leaves offsets 41–44 zero.
        let mut entry = [0; ENTRY_SIZE];
        entry[0] = 5;
        entry[2..6].copy_from_slice(&manager.id.to_le_bytes());
        entry[7..11].copy_from_slice(&person.id.to_le_bytes());
        entry[13..17].copy_from_slice(&manager.team.to_le_bytes());
        entry[17..21].copy_from_slice(&estimated.to_le_bytes());
        entry[29..33].fill(0xff);
        entry[33] = contract.squad_status as u8;
        entry[34] = before;
        entry[45..49].fill(0xff);
        entry[49] = contract.squad_status as u8;
        entry[50] = after;
        entry[61..65].fill(0xff);

        updated_session.extend_from_slice(&entry);
        let offset = base + contract.common_offset + TRANSFER_STATUS_OFFSET;
        edited[offset] = after;
        let player = db.players_by_uid(base, uid)?.remove(0);
        changes.push(editor::Change {
            uid,
            name: player.name,
            attribute: "contract.0.transferStatus".into(),
            before: before.into(),
            after: after.into(),
            raw_before: before.into(),
            raw_after: after.into(),
            offset,
            width: 1,
            derived: false,
            context: player.club,
            display_before: None,
            display_after: None,
        });
    }
    let copy = replace_block(&edited, SESSION, ".dat", &updated_session)?;
    let (readback, _) = database::load(&copy)?;
    for (&uid, &after) in targets {
        if player_contract(&readback, uid)?.1.transfer_status != after {
            return Err("A releitura da cópia não confirmou a situação de mercado".into());
        }
    }
    Ok((copy, changes))
}

pub fn supports_journal(bytes: &[u8]) -> bool {
    market_journal(bytes).is_ok()
}

fn club_managers(db: &Database) -> BTreeMap<i32, Option<&Person>> {
    let mut managers = BTreeMap::new();
    for person in db.people.iter().filter(|person| person.kind == 9) {
        if let Some(team) = db.teams.get(person.team as usize)
            && team.container_type == 0
            && team.squad == 0
        {
            managers
                .entry(team.container_index)
                .and_modify(|slot| *slot = None)
                .or_insert(Some(person));
        }
    }
    managers
}

fn market_manager<'a>(
    db: &Database,
    managers: &BTreeMap<i32, Option<&'a Person>>,
    person: &Person,
    contract: &Contract,
) -> Option<&'a Person> {
    let team = db.teams.get(person.team as usize)?;
    if team.container_type != 0 || team.container_index != contract.entity_index {
        return None;
    }
    managers.get(&contract.entity_index).copied().flatten()
}

pub fn editable_statuses(db: &Database, bytes: &[u8]) -> BTreeMap<i32, u8> {
    if !supports_journal(bytes) {
        return BTreeMap::new();
    }
    let managers = club_managers(db);
    let mut seen_uids = BTreeSet::new();
    let duplicate_uids: BTreeSet<_> = db
        .people
        .iter()
        .map(|person| person.uid)
        .filter(|&uid| !seen_uids.insert(uid))
        .collect();
    db.people
        .iter()
        .filter_map(|person| {
            let [contract] = person.contracts.as_slice() else {
                return None;
            };
            (!duplicate_uids.contains(&person.uid)
                && person.playing.is_some()
                && contract.kind == 0
                && contract.entity_type == 0
                && db.clubs.get(contract.entity_index as usize).is_some()
                && !supported_targets(contract.transfer_status).is_empty()
                && market_manager(db, &managers, person, contract).is_some())
            .then_some((person.uid, contract.transfer_status))
        })
        .collect()
}

fn market_journal(bytes: &[u8]) -> Result<&[u8]> {
    let (session, _) = find_block(bytes, SESSION, ".dat")?;
    let entries = session
        .strip_prefix(&SESSION_HEADER)
        .ok_or("Cabeçalho do diário de mercado incompatível")?;
    let (entries, remainder) = entries.as_chunks::<ENTRY_SIZE>();
    if !remainder.is_empty() || entries.iter().any(|entry| entry[..2] != [5, 0]) {
        return Err("Diário contém operações fora do formato de mercado observado".into());
    }
    Ok(session)
}

pub(crate) fn journal_mentions_player(bytes: &[u8], person_id: i32) -> Result<bool> {
    let journal = market_journal(bytes)?;
    let id = person_id.to_le_bytes();
    Ok(journal[SESSION_HEADER.len()..]
        .as_chunks::<ENTRY_SIZE>()
        .0
        .iter()
        .any(|entry| entry[7..11] == id))
}

fn observed_transition(before: u8, after: u8) -> bool {
    supported_targets(before).contains(&after)
}

pub fn supported_targets(before: u8) -> &'static [u8] {
    match before {
        4 => &[5, 6, 68],
        5 => &[7],
        68 => &[4, 69],
        _ => &[],
    }
}

fn player_contract(db: &Database, uid: i32) -> Result<(&Person, &Contract)> {
    let mut matches = db.people.iter().filter(|person| person.uid == uid);
    let person = matches.next().ok_or("Jogador ausente")?;
    let [contract] = person.contracts.as_slice() else {
        return Err("O ensaio exige um único contrato".into());
    };
    if matches.next().is_some()
        || person.playing.is_none()
        || contract.kind != 0
        || contract.entity_type != 0
    {
        return Err("Jogador ou contrato incompatível com o ensaio".into());
    }
    Ok((person, contract))
}

fn replay(input: &[u8], donor: &[u8], uid: i32) -> Result<(Vec<u8>, Report)> {
    let (source_db, base) = database::load(input)?;
    let (donor_db, _) = database::load(donor)?;
    let (person, contract) = player_contract(&source_db, uid)?;
    let (donor_person, donor_contract) = player_contract(&donor_db, uid)?;
    let club_uid = |db: &Database, contract: &Contract| {
        db.clubs
            .get(contract.entity_index as usize)
            .map(|club| club.uid)
    };
    let identity = club_uid(&source_db, contract).ok_or("Clube do contrato ausente")?;
    let (before, after) = (contract.transfer_status, donor_contract.transfer_status);
    if (source_db.game_date.year, source_db.game_date.day)
        != (donor_db.game_date.year, donor_db.game_date.day)
        || person.id != donor_person.id
        || club_uid(&donor_db, donor_contract) != Some(identity)
        || contract.squad_status != donor_contract.squad_status
        || contract.weekly_wage != donor_contract.weekly_wage
        || (contract.started.year, contract.started.day)
            != (donor_contract.started.year, donor_contract.started.day)
        || (contract.expires.year, contract.expires.day)
            != (donor_contract.expires.year, donor_contract.expires.day)
        || !observed_transition(before, after)
    {
        return Err(
            "O save de referência precisa corresponder ao vínculo e ao estado do ensaio".into(),
        );
    }
    let (source_transfers, _) = find_block(input, TRANSFERS, ".dat")?;
    let (donor_transfers, _) = find_block(donor, TRANSFERS, ".dat")?;
    if source_transfers != donor_transfers {
        return Err("O bloco de transferências diverge no save de referência".into());
    }
    let (source_session, _) = find_block(input, SESSION, ".dat")?;
    let (donor_session, _) = find_block(donor, SESSION, ".dat")?;
    let entry = donor_session
        .strip_prefix(source_session)
        .filter(|entry| entry.len() == ENTRY_SIZE)
        .ok_or("O save de referência deve acrescentar exatamente uma operação à sessão")?;
    let estimated = person.playing.as_ref().unwrap().state.estimated_value;
    if entry[..2] != [5, 0]
        || entry[7..11] != person.id.to_le_bytes()
        || entry[17..21] != estimated.to_le_bytes()
        || entry[33] as i8 != contract.squad_status
        || entry[49] as i8 != contract.squad_status
        || entry[34] != before
        || entry[50] != after
    {
        return Err("A entrada da sessão não corresponde à alteração de mercado".into());
    }

    write_variant(
        input,
        uid,
        base + contract.common_offset,
        before,
        after,
        donor_session,
    )
}

fn write_variant(
    input: &[u8],
    uid: i32,
    contract_offset: usize,
    before: u8,
    after: u8,
    session: &[u8],
) -> Result<(Vec<u8>, Report)> {
    let mut edited = input.to_vec();
    edited[contract_offset + TRANSFER_STATUS_OFFSET] = after;
    let copy = replace_block(&edited, SESSION, ".dat", session)?;
    let (db, _) = database::load(&copy)?;
    if player_contract(&db, uid)?.1.transfer_status != after {
        return Err("A releitura da cópia não confirmou a situação de mercado".into());
    }
    let report = Report {
        player_uid: uid,
        before,
        after,
        source_sha256: hash(input),
        copy_sha256: hash(&copy),
    };
    Ok((copy, report))
}
