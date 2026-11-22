//! Experimental relocation of a player and their existing club contract.
use crate::{Result, container::hash, database, editor};
use serde::Serialize;
use std::{path::Path, sync::atomic::AtomicBool};

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferHistoryEntry {
    pub payload_offset: usize,
    pub person_id: i32,
    pub destination_team_index: i32,
    pub source_team_index: i32,
    pub fee_raw: u32,
    pub status_raw: u8,
    pub day: u16,
    pub year: u16,
    pub kind_raw: u8,
}

fn counted_rows<const WIDTH: usize>(payload: &[u8], offset: usize) -> Option<&[[u8; WIDTH]]> {
    let count = u32::from_le_bytes(payload.get(offset..offset + 4)?.try_into().ok()?) as usize;
    let start = offset + 4;
    let end = start.checked_add(count.checked_mul(WIDTH)?)?;
    Some(payload.get(start..end)?.as_chunks::<WIDTH>().0)
}

fn history_date(bytes: &[u8]) -> Option<(u16, u16)> {
    let day = u16::from_le_bytes([bytes[0], bytes[1]]);
    let year = u16::from_le_bytes([bytes[2], bytes[3]]);
    (day < 366 && (1900..=2100).contains(&year)).then_some((year, day))
}

/// Finds dated transfer history in saves with both observed collections populated.
pub fn inspect_history(input: &[u8]) -> Result<Vec<TransferHistoryEntry>> {
    const WIDTH: usize = 25;
    let (payload, _) = crate::container::find_block(input, "transfer_man", ".dat")?;
    let mut candidates = (0..payload.len().saturating_sub(3)).filter_map(|offset| {
        let rows = counted_rows::<WIDTH>(payload, offset)?;
        if rows.len() < 2 {
            return None;
        }
        let next = counted_rows::<8>(payload, offset + 4 + rows.len() * WIDTH)?;
        if next.is_empty() || next.iter().any(|row| history_date(row).is_none()) {
            return None;
        }
        let dates = rows.iter().map(|row| history_date(&row[20..24]));
        if !dates.clone().all(|date| date.is_some()) || !dates.is_sorted() {
            return None;
        }
        Some((offset + 4, rows))
    });
    let (start, rows) = candidates
        .next()
        .ok_or("Histórico de transferências não identificado neste save")?;
    if candidates.next().is_some() {
        return Err("Histórico de transferências ambíguo".into());
    }
    Ok(rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let word = |offset| i32::from_le_bytes(row[offset..offset + 4].try_into().unwrap());
            let (year, day) = history_date(&row[20..24]).unwrap();
            TransferHistoryEntry {
                payload_offset: start + index * WIDTH,
                person_id: word(0),
                destination_team_index: word(6),
                source_team_index: word(11),
                fee_raw: word(15) as u32,
                status_raw: row[19],
                day,
                year,
                kind_raw: row[24],
            }
        })
        .collect())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptedOffer {
    pub person_id: u32,
    pub destination_team_index: u32,
    pub upfront_raw: u32,
    pub installments_raw: u32,
    pub installment_months: u32,
    pub sell_on_percent: u32,
}

/// Reads the installment-and-sell-on offer variant captured from the game.
pub fn inspect_accepted_offers(input: &[u8]) -> Result<Vec<AcceptedOffer>> {
    const PREFIX: [u8; 12] = [0, b'.', 0, b'd', 0, b'a', 0, b't', 0, 15, 0, 0];
    const ENTRY_LEN: usize = 320;
    let (payload, _) = crate::container::find_block(input, "edit_session_man", ".dat")?;
    let body = payload
        .strip_prefix(&PREFIX)
        .ok_or("Cabeçalho de diário incompatível")?;
    let (entries, remainder) = body.as_chunks::<ENTRY_LEN>();
    if !remainder.is_empty() {
        return Err("Diário fora da variante de aceitação observada".into());
    }
    entries
        .iter()
        .map(|entry| {
            let word = |offset| u32::from_le_bytes(entry[offset..offset + 4].try_into().unwrap());
            // Both copies of the observed terms must agree.
            if !entry.starts_with(&[1, 0, 0x13, 0x80])
                || entry[198..222] != entry[287..311]
                || entry[211] != 2
                || !(1..=100).contains(&word(202))
                || word(214) == 0
                || word(218) == 0
            {
                return Err("Entrada fora da variante de aceitação observada".into());
            }
            Ok(AcceptedOffer {
                person_id: word(12),
                destination_team_index: word(8),
                upfront_raw: word(198),
                sell_on_percent: word(202),
                installments_raw: word(214),
                installment_months: word(218),
            })
        })
        .collect()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferReference {
    pub payload_offset: usize,
    pub unknown_word: u32,
    pub installment_amount_raw: u32,
    pub destination_club_index: u32,
    pub source_club_index: u32,
    pub unknown_byte: u8,
    pub installments_remaining: u8,
    pub person_id: u32,
}

/// Reads the transfer reference table observed in the 2003–04 checkpoints.
pub fn inspect_references(input: &[u8]) -> Result<Vec<TransferReference>> {
    const HEADER: [u8; 4] = [0xd3, 0x07, 0x28, 0x00];
    const ROW_LEN: usize = 17;
    const FOOTER_ROW_LEN: usize = 24;
    let (payload, _) = crate::container::find_block(input, "transfer_man", ".dat")?;
    let mut candidates = payload
        .windows(6)
        .enumerate()
        .filter_map(|(offset, header)| {
            if !header.starts_with(&HEADER) {
                return None;
            }
            let count = u16::from_le_bytes([header[4], header[5]]) as usize;
            let start = offset + header.len();
            let end = start + count * ROW_LEN;
            let footer = payload.get(end..)?;
            let footer_count = u32::from_le_bytes(footer.get(..4)?.try_into().ok()?) as usize;
            let footer_body_len = footer.len() - 4;
            (footer_body_len % FOOTER_ROW_LEN == 0
                && footer_body_len / FOOTER_ROW_LEN == footer_count)
                .then_some((start, end))
        });
    let (start, end) = candidates
        .next()
        .ok_or("Tabela de transferências fora do formato observado")?;
    if candidates.next().is_some() {
        return Err("Tabela de transferências ambígua".into());
    }
    Ok(payload[start..end]
        .as_chunks::<ROW_LEN>()
        .0
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let word = |offset| u32::from_le_bytes(row[offset..offset + 4].try_into().unwrap());
            TransferReference {
                payload_offset: start + index * ROW_LEN,
                unknown_word: word(0),
                installment_amount_raw: word(0),
                destination_club_index: word(4),
                source_club_index: word(8),
                unknown_byte: row[12],
                installments_remaining: row[12],
                person_id: word(13),
            }
        })
        .collect())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    player_uid: i32,
    source_club_uid: i32,
    destination_club_uid: i32,
    source_team: TeamImpact,
    destination_team: TeamImpact,
    contract_terms: ContractTerms,
    #[serde(skip_serializing_if = "Option::is_none")]
    history_entry: Option<TransferHistoryEntry>,
    source_sha256: String,
    copy_sha256: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TeamImpact {
    team_index: usize,
    squad: u8,
    roster_before: usize,
    roster_after: usize,
    club_full_player_wages_before: i64,
    club_full_player_wages_after: i64,
    club_recorded_wages_before: Option<i32>,
    club_recorded_wages_after: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ContractTerms {
    weekly_wage_gbp: i32,
    started: database::Date,
    expires: database::Date,
    squad_status: i8,
    transfer_status: u8,
    squad_number: i8,
    appearance_bonus_gbp: Option<i32>,
    goal_bonus_gbp: Option<i32>,
    assist_bonus_gbp: Option<i32>,
    release_clause_gbp: Option<i32>,
}

fn full_player_wages(db: &database::Database, club_index: i32) -> i64 {
    db.people
        .iter()
        .filter(|person| person.playing.is_some())
        .flat_map(|person| &person.contracts)
        .filter(|contract| {
            contract.kind == 0 && contract.entity_type == 0 && contract.entity_index == club_index
        })
        .map(|contract| i64::from(contract.weekly_wage))
        .sum()
}

pub fn preview(input: &[u8], player_uid: i32, destination_uid: i32) -> Result<Report> {
    preview_with_options(input, player_uid, destination_uid, false)
}

pub fn save(source: &Path, copy: &Path, player_uid: i32, destination_uid: i32) -> Result<Report> {
    save_with_options(source, copy, player_uid, destination_uid, false)
}

pub fn preview_with_options(
    input: &[u8],
    player_uid: i32,
    destination_uid: i32,
    with_history: bool,
) -> Result<Report> {
    relocate(input, player_uid, destination_uid, with_history).map(|(_, report)| report)
}

pub fn save_with_options(
    source: &Path,
    copy: &Path,
    player_uid: i32,
    destination_uid: i32,
    with_history: bool,
) -> Result<Report> {
    let input = std::fs::read(source).map_err(|e| e.to_string())?;
    editor::save_transformed_snapshot(source, copy, &input, &AtomicBool::new(false), |bytes| {
        relocate(bytes, player_uid, destination_uid, with_history)
    })
}

fn append_free_transfer(
    input: &[u8],
    person: i32,
    source: i32,
    destination: i32,
    date: &database::Date,
) -> Result<(Vec<u8>, TransferHistoryEntry)> {
    let history = inspect_history(input)?;
    let (payload, _) = crate::container::find_block(input, "transfer_man", ".dat")?;
    let start = history
        .first()
        .ok_or("Histórico vazio sem formato validado")?
        .payload_offset;
    let last = history
        .last()
        .ok_or("Histórico vazio sem formato validado")?;
    if (last.year, last.day) > (date.year, date.day) {
        return Err("O histórico termina depois da data do save".into());
    }
    if history.iter().any(|row| {
        row.person_id == person
            && row.source_team_index == source
            && row.destination_team_index == destination
            && (row.year, row.day) == (date.year, date.day)
    }) {
        return Err("Já existe uma transferência igual para este jogador nessa data".into());
    }
    let insertion = last.payload_offset + 25;
    let mut row = [0; 25];
    row[..4].copy_from_slice(&person.to_le_bytes());
    row[6..10].copy_from_slice(&destination.to_le_bytes());
    row[11..15].copy_from_slice(&source.to_le_bytes());
    row[19] = 1;
    row[20..22].copy_from_slice(&date.day.to_le_bytes());
    row[22..24].copy_from_slice(&date.year.to_le_bytes());
    row[24] = 1;
    let count = u32::try_from(history.len() + 1).map_err(|_| "Histórico grande demais")?;
    let mut updated = Vec::with_capacity(payload.len() + row.len());
    updated.extend_from_slice(&payload[..start - 4]);
    updated.extend_from_slice(&count.to_le_bytes());
    updated.extend_from_slice(&payload[start..insertion]);
    updated.extend_from_slice(&row);
    updated.extend_from_slice(&payload[insertion..]);
    let output = crate::container::replace_block(input, "transfer_man", ".dat", &updated)?;
    let mut verified = inspect_history(&output)?;
    let added = verified
        .pop()
        .ok_or("A releitura não encontrou a nova transferência")?;
    if verified != history
        || added.payload_offset != insertion
        || added.person_id != person
        || added.fee_raw != 0
        || added.source_team_index != source
        || added.destination_team_index != destination
        || (added.year, added.day, added.status_raw, added.kind_raw) != (date.year, date.day, 1, 1)
    {
        return Err("A releitura não confirmou a transferência sem taxa no histórico".into());
    }
    Ok((output, added))
}

fn relocate(
    input: &[u8],
    player_uid: i32,
    destination_uid: i32,
    with_history: bool,
) -> Result<(Vec<u8>, Report)> {
    let (db, base) = database::load(input)?;
    let mut players = db.people.iter().filter(|p| p.uid == player_uid);
    let person = players.next().ok_or("Jogador não encontrado")?;
    if players.next().is_some() || person.kind != 2 || person.id as usize != person.index {
        return Err("Identidade ou tipo de jogador incompatível com a prova de laboratório".into());
    }
    match inspect_accepted_offers(input) {
        Ok(offers)
            if offers
                .iter()
                .any(|offer| offer.person_id == person.id as u32) =>
        {
            return Err("O jogador tem uma proposta aceita no diário. Aguarde o processamento no CM0304 antes de realocá-lo.".into());
        }
        Err(_) if with_history => {
            let mentioned = crate::market_lab::journal_mentions_player(input, person.id)
                .map_err(|_| "Diário de operações fora dos formatos reconhecidos para gravar a transferência com histórico")?;
            if mentioned {
                return Err("O jogador tem uma edição de mercado no diário. Aguarde o processamento no CM0304 antes de realocá-lo.".into());
            }
        }
        _ => {}
    }
    let [contract] = person.contracts.as_slice() else {
        return Err("O jogador precisa ter um único contrato".into());
    };
    let source_index = usize::try_from(person.team).map_err(|_| "Equipe atual inválida")?;
    let source = db.teams.get(source_index).ok_or("Equipe atual ausente")?;
    if source.container_type != 0
        || contract.kind != 0
        || contract.entity_type != 0
        || contract.entity_index != source.container_index
    {
        return Err("Equipe e contrato completo precisam pertencer ao mesmo clube".into());
    }
    let source_reference = source.container_index.to_le_bytes();
    if person
        .squad_entries_raw
        .iter()
        .any(|entry| entry[4..] == source_reference)
    {
        return Err("O jogador tem registros de elenco associados ao clube atual. Use o CM0304 para concluir a transferência e atualizar essas referências.".into());
    }
    let source_club = db
        .clubs
        .get(source.container_index as usize)
        .ok_or("Clube atual ausente")?;
    if source_club.details.as_ref().is_some_and(|details| {
        details
            .sell_on_clauses
            .iter()
            .any(|clause| clause.player_uid == player_uid)
    }) {
        return Err("O clube atual tem uma cláusula de revenda para este jogador. Use o CM0304 para concluir a saída e processar a cláusula.".into());
    }
    if source_club.uid == destination_uid {
        return Err("Escolha outro clube de destino".into());
    }
    let mut clubs = db
        .clubs
        .iter()
        .enumerate()
        .filter(|(_, c)| c.uid == destination_uid);
    let (club_index, _) = clubs.next().ok_or("Clube de destino ausente")?;
    if clubs.next().is_some() {
        return Err("UID do clube de destino repetido".into());
    }
    if contract.squad_number > 0
        && let Some(other) = db.people.iter().find(|other| {
            other.contracts.iter().any(|other_contract| {
                other_contract.entity_type == 0
                    && other_contract.entity_index as usize == club_index
                    && other_contract.squad_number == contract.squad_number
            })
        })
    {
        return Err(format!(
            "Camisa {} já usada no clube de destino pelo UID {}",
            contract.squad_number, other.uid
        ));
    }
    let mut teams = db.teams.iter().enumerate().filter(|(_, t)| {
        t.container_type == 0 && t.container_index as usize == club_index && t.squad == 0
    });
    let (destination_index, destination) = teams.next().ok_or("Destino sem equipe principal")?;
    if teams.next().is_some() {
        return Err("Destino com equipes principais ambíguas".into());
    }
    let mut memberships = db.teams.iter().enumerate().flat_map(|(index, team)| {
        team.roster_person_indices
            .iter()
            .enumerate()
            .filter(|(_, id)| **id == person.id)
            .map(move |(slot, _)| (index, slot))
    });
    let (actual_team, slot) = memberships
        .next()
        .ok_or("Jogador ausente das listas de elenco")?;
    if actual_team != source_index || memberships.next().is_some() {
        return Err("Jogador tem uma associação de elenco inconsistente".into());
    }
    let destination_count = u16::try_from(destination.roster_person_indices.len() + 1)
        .map_err(|_| "Lista do destino sem espaço")?;
    let source_count = (source.roster_person_indices.len() - 1) as u16;
    let source_offset = base + source.roster_offset;
    let destination_offset = base + destination.roster_offset;
    let remove = source_offset + 2 + slot * 4;
    let append = destination_offset + 2 + destination.roster_person_indices.len() * 4;
    let mut output = input.to_vec();
    let person_team = base + person.common_offset + 33;
    let contract_club = base + contract.common_offset + 1;
    output[person_team..person_team + 4].copy_from_slice(&(destination_index as i32).to_le_bytes());
    output[contract_club..contract_club + 4].copy_from_slice(&(club_index as i32).to_le_bytes());
    output[source_offset..source_offset + 2].copy_from_slice(&source_count.to_le_bytes());
    output[destination_offset..destination_offset + 2]
        .copy_from_slice(&destination_count.to_le_bytes());
    let entry = person.id.to_le_bytes();
    if remove > append {
        output.drain(remove..remove + 4);
        output.splice(append..append, entry);
    } else {
        output.splice(append..append, entry);
        output.drain(remove..remove + 4);
    }
    let (verified, verified_base) = database::load(&output)?;
    let moved = &verified.people[person.index];
    let [moved_contract] = moved.contracts.as_slice() else {
        return Err("A releitura não encontrou um único contrato para o jogador".into());
    };
    let verified_club = verified_base + moved_contract.common_offset + 1;
    if input[base + contract.offset..contract_club]
        != output[verified_base + moved_contract.offset..verified_club]
        || input[contract_club + 4..base + contract.end_offset]
            != output[verified_club + 4..verified_base + moved_contract.end_offset]
    {
        return Err("A releitura encontrou mudanças nos termos ou na estrutura do contrato".into());
    }
    if moved.uid != player_uid
        || moved.team as usize != destination_index
        || moved_contract.entity_index as usize != club_index
        || !verified.teams[source_index]
            .roster_person_indices
            .iter()
            .eq(source
                .roster_person_indices
                .iter()
                .filter(|&&id| id != person.id))
        || !verified.teams[destination_index]
            .roster_person_indices
            .iter()
            .eq(destination
                .roster_person_indices
                .iter()
                .chain(std::iter::once(&person.id)))
    {
        return Err("A releitura não confirmou a realocação".into());
    }
    let recorded_wages = |database: &database::Database, club_index: i32| {
        usize::try_from(club_index)
            .ok()
            .and_then(|index| database.clubs.get(index))
            .and_then(|club| club.details.as_ref())
            .and_then(|details| details.current_wages_weekly)
    };
    let impact = |team_index: usize, club_index: i32| TeamImpact {
        team_index,
        squad: db.teams[team_index].squad,
        roster_before: db.teams[team_index].roster_person_indices.len(),
        roster_after: verified.teams[team_index].roster_person_indices.len(),
        club_full_player_wages_before: full_player_wages(&db, club_index),
        club_full_player_wages_after: full_player_wages(&verified, club_index),
        club_recorded_wages_before: recorded_wages(&db, club_index),
        club_recorded_wages_after: recorded_wages(&verified, club_index),
    };
    let (output, history_entry) = if with_history {
        let (updated, entry) = append_free_transfer(
            &output,
            person.id,
            source_index as i32,
            destination_index as i32,
            &db.game_date,
        )?;
        (updated, Some(entry))
    } else {
        (output, None)
    };
    let report = Report {
        player_uid,
        source_club_uid: source_club.uid,
        destination_club_uid: destination_uid,
        source_team: impact(source_index, source.container_index),
        destination_team: impact(destination_index, club_index as i32),
        history_entry,
        contract_terms: ContractTerms {
            weekly_wage_gbp: contract.weekly_wage,
            started: contract.started.clone(),
            expires: contract.expires.clone(),
            squad_status: contract.squad_status,
            transfer_status: contract.transfer_status,
            squad_number: contract.squad_number,
            appearance_bonus_gbp: contract.appearance_bonus,
            goal_bonus_gbp: contract.goal_bonus,
            assist_bonus_gbp: contract.assist_bonus,
            release_clause_gbp: contract.release_clause.as_ref().map(|clause| clause.amount),
        },
        source_sha256: hash(input),
        copy_sha256: hash(&output),
    };
    Ok((output, report))
}
