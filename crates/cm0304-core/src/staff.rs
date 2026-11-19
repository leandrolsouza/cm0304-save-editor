//! Staff reading and attribute conversion. Writing requires a separate transaction.
use crate::{
    Result, attributes,
    binary::Reader,
    container,
    database::{Database, Date},
};
use serde::Serialize;
use std::collections::BTreeMap;

pub const FIELDS: [&str; 28] = [
    "attacking",
    "business",
    "coachingTechnique",
    "directness",
    "discipline",
    "freeRoles",
    "interference",
    "marking",
    "offside",
    "patience",
    "pressing",
    "resources",
    "workingWithYoungsters",
    "determination",
    "buyingPlayers",
    "mindGames",
    "sellingPlayers",
    "sittingBack",
    "useOfPlaymaker",
    "useOfSubs",
    "coachingOutfieldPlayers",
    "coachingGoalkeepers",
    "judgingPlayerAbility",
    "judgingPlayerPotential",
    "manManagement",
    "motivating",
    "physiotherapy",
    "tacticalKnowledge",
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Staff {
    pub uid: i32,
    pub name: String,
    pub person_kind: u8,
    pub is_player: bool,
    pub is_human: bool,
    pub team: Option<String>,
    pub club_job_raw: u8,
    pub club_job: &'static str,
    pub national_team: Option<String>,
    pub national_job_raw: u8,
    pub national_job: &'static str,
    pub nation: Option<String>,
    pub date_of_birth: Option<Date>,
    pub reputation: [i16; 3],
    pub current_ability: i16,
    pub player_current_ability: Option<i16>,
    pub potential_ability: i16,
    pub preferred_formation_raw: i8,
    pub attribute_index: i32,
    pub raw_attributes: Option<BTreeMap<String, i8>>,
    pub attributes: Option<BTreeMap<String, i8>>,
    pub personality: Option<attributes::Attributes>,
    pub info_offset: usize,
    pub attributes_offset: Option<usize>,
}

pub const RELATIVE: [&str; 8] = [
    "coachingOutfieldPlayers",
    "coachingGoalkeepers",
    "judgingPlayerAbility",
    "judgingPlayerPotential",
    "manManagement",
    "motivating",
    "physiotherapy",
    "tacticalKnowledge",
];

/// In the game, player/staff professional ratings use the playing role's CA.
pub fn attribute_ability(profile: &Staff) -> i16 {
    profile
        .player_current_ability
        .unwrap_or(profile.current_ability)
}

pub fn decoded(raw: &BTreeMap<String, i8>, ca: i16) -> Result<BTreeMap<String, i8>> {
    FIELDS
        .iter()
        .map(|key| {
            let value = *raw
                .get(*key)
                .ok_or_else(|| format!("Atributo de staff ausente: {key}"))?;
            let value = if RELATIVE.contains(key) {
                attributes::convert(value, ca, false)?
            } else {
                if !(0..=20).contains(&value) {
                    return Err(format!("Atributo direto de staff inválido: {key}={value}"));
                }
                value
            };
            Ok((key.to_string(), value))
        })
        .collect()
}

pub fn encode_attribute(key: &str, target: i8, ca: i16, original: i8) -> Result<i8> {
    if !FIELDS.contains(&key) {
        return Err(format!("Atributo de staff desconhecido: {key}"));
    }
    if !(1..=20).contains(&target) {
        return Err("Atributo de staff deve estar entre 1 e 20".into());
    }
    if RELATIVE.contains(&key) {
        attributes::encode(target, ca, false, original)
    } else {
        Ok(target)
    }
}

pub fn job_label(job: u8) -> &'static str {
    match job {
        0 => "Não informado",
        1 => "Jogador",
        2 => "Preparador",
        3 => "Jogador/preparador",
        4 => "Presidente",
        6 => "Diretor",
        8 => "Diretor executivo",
        10 => "Diretor de futebol",
        12 => "Fisioterapeuta",
        14 => "Olheiro",
        16 => "Treinador",
        17 => "Jogador/treinador",
        20 => "Treinador adjunto",
        21 => "Jogador/treinador adjunto",
        24 => "Diretor geral",
        _ => "Função desconhecida",
    }
}

/// Reads the table following playingData, keeping the historical parser boundary unchanged.
pub fn profiles(db: &Database, input: &[u8], base: usize) -> Result<Vec<Staff>> {
    let container = container::read_container(input)?;
    let block = container
        .blocks
        .iter()
        .find(|b| b.payload_offset == base && b.name == "game_db" && b.extension == ".dat")
        .ok_or("Banco de staff não encontrado")?;
    let payload = input
        .get(base..base + block.size)
        .ok_or("Banco de staff truncado")?;
    let mut reader = Reader::new(payload);
    reader.pos = db.parsed_until;
    let count = reader.count()?;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        reader.kind(&[0])?;
        let offset = reader.pos;
        let mut attributes = BTreeMap::new();
        for field in FIELDS {
            attributes.insert(field.to_string(), reader.u8()? as i8);
        }
        reader.identity()?;
        records.push((offset, attributes));
    }
    let name = |table: usize, index: i32| -> Result<String> {
        if index == -1 {
            return Ok(String::new());
        }
        usize::try_from(index)
            .ok()
            .and_then(|i| db.names[table].get(i))
            .cloned()
            .ok_or_else(|| format!("Nome de staff inválido: {index}"))
    };
    let team_name = |index: i32| -> Result<Option<String>> {
        if index == -1 {
            return Ok(None);
        }
        let team = usize::try_from(index)
            .ok()
            .and_then(|i| db.teams.get(i))
            .ok_or("Equipe de staff inválida")?;
        let table = match team.container_type {
            0 => &db.clubs,
            1 => &db.nations,
            _ => return Err("Tipo de equipe de staff inválido".into()),
        };
        Ok(Some(
            usize::try_from(team.container_index)
                .ok()
                .and_then(|i| table.get(i))
                .ok_or("Clube/seleção de staff inválido")?
                .name
                .clone(),
        ))
    };
    db.people
        .iter()
        .filter(|person| matches!(person.kind, 1 | 3 | 9))
        .map(|person| {
            let info_offset = person
                .common_offset
                .checked_sub(15)
                .ok_or("Cabeçalho de staff inválido")?;
            let mut info = Reader::new(payload);
            info.pos = info_offset;
            let reputation = [info.i16()?, info.i16()?, info.i16()?];
            let current_ability = info.i16()?;
            let potential_ability = info.i16()?;
            let preferred_formation_raw = info.u8()? as i8;
            let attribute_index = info.i32()?;
            let record = if attribute_index == -1 {
                None
            } else {
                Some(
                    usize::try_from(attribute_index)
                        .ok()
                        .and_then(|i| records.get(i))
                        .ok_or_else(|| {
                            format!(
                                "Staff {}: referência inválida {attribute_index}",
                                person.uid
                            )
                        })?,
                )
            };
            let common = name(2, person.common)?;
            let full_name = if common.is_empty() {
                format!("{} {}", name(0, person.first)?, name(1, person.surname)?)
                    .trim()
                    .to_owned()
            } else {
                common
            };
            let team = team_name(person.team)?;
            info.pos = person.common_offset + 24;
            let national_team = team_name(info.i32()?)?;
            let national_job_raw = info.u8()?;
            info.skip(8)?;
            let club_job_raw = info.u8()?;
            let nation = if person.nation == -1 {
                None
            } else {
                Some(
                    usize::try_from(person.nation)
                        .ok()
                        .and_then(|i| db.nations.get(i))
                        .ok_or("Nacionalidade de staff inválida")?
                        .name
                        .clone(),
                )
            };
            let personality = if person.personality == -1 {
                None
            } else {
                Some(
                    usize::try_from(person.personality)
                        .ok()
                        .and_then(|i| db.personalities.get(i))
                        .ok_or("Personalidade de staff inválida")?
                        .clone(),
                )
            };
            Ok(Staff {
                uid: person.uid,
                name: full_name,
                person_kind: person.kind,
                is_player: person.kind == 3,
                is_human: person.kind == 9,
                team,
                club_job_raw,
                club_job: job_label(club_job_raw),
                national_team,
                national_job_raw,
                national_job: job_label(national_job_raw),
                nation,
                date_of_birth: person.birth.clone(),
                reputation,
                current_ability,
                player_current_ability: person.playing.as_ref().map(|p| p.ca),
                potential_ability,
                preferred_formation_raw,
                attribute_index,
                raw_attributes: record.map(|(_, a)| a.clone()),
                attributes: record
                    .map(|(_, raw)| {
                        decoded(
                            raw,
                            person.playing.as_ref().map_or(current_ability, |p| p.ca),
                        )
                    })
                    .transpose()?,
                personality,
                info_offset: base + info_offset,
                attributes_offset: record.map(|(o, _)| base + o),
            })
        })
        .collect()
}
