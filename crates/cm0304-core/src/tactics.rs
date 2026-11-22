//! Tática ativa e titulares de equipes principais controladas por técnicos humanos.
use crate::{Result, binary::Reader, container::find_optional_block, database::Database};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

const TACTIC_BODY_SIZE: usize = 835;
const SELECTION_PREFIX_SIZE: usize = 108;
const SELECTION_SUFFIX_SIZE: usize = 15;
const POSITION_OFFSET: usize = 57;
const POSITION_STRIDE: usize = 74;
const STARTING_SLOTS: usize = 11;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TacticPosition {
    pub row: u8,
    pub column: u8,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubTactic {
    pub name: String,
    pub positions: Vec<TacticPosition>,
    /// UIDs na mesma ordem das posições. Uma escalação incompatível fica indisponível.
    pub starters: Option<Vec<Option<i32>>>,
}

fn decode_position(code: u16) -> Option<TacticPosition> {
    if code == 1 {
        return Some(TacticPosition { row: 0, column: 2 });
    }
    let [role, side] = code.to_le_bytes();
    let row = match role & 0x7f {
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        32 => 5,
        64 => 6,
        _ => return None,
    };
    let column = match (role & 0x80, side) {
        (0x80, 0) => 0,
        (0, 1) => 1,
        (0, 2) => 2,
        (0, 4) => 3,
        (0, 8) => 4,
        _ => return None,
    };
    Some(TacticPosition { row, column })
}

fn positions(body: &[u8]) -> Option<Vec<TacticPosition>> {
    let positions = (0..STARTING_SLOTS)
        .map(|index| {
            let offset = POSITION_OFFSET + POSITION_STRIDE * index;
            decode_position(u16::from_le_bytes(
                body.get(offset..offset + 2)?.try_into().ok()?,
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    let unique = positions.iter().collect::<BTreeSet<_>>();
    (unique.len() == STARTING_SLOTS && positions.iter().filter(|p| p.row == 0).count() == 1)
        .then_some(positions)
}

fn starters(prefix: &[u8], db: &Database, club_index: i32) -> Option<Vec<Option<i32>>> {
    let mut r = Reader::new(prefix);
    let mut seen = BTreeSet::new();
    (0..STARTING_SLOTS)
        .map(|_| {
            let id = r.i32().ok()?;
            if id == -1 {
                return Some(None);
            }
            let player = db.people.get(usize::try_from(id).ok()?)?;
            let team = db.teams.get(usize::try_from(player.team).ok()?)?;
            (player.id == id
                && player.playing.is_some()
                && team.container_type == 0
                && team.container_index == club_index
                && seen.insert(player.uid))
            .then_some(Some(player.uid))
        })
        .collect()
}

pub fn clubs_with_current_tactics(
    save: &[u8],
    db: &Database,
) -> Result<BTreeMap<i32, Result<ClubTactic>>> {
    let Some((block, _)) = find_optional_block(save, "tactics_man", ".dat")? else {
        return Ok(BTreeMap::new());
    };
    let mut r = Reader::new(block);
    if r.take(12)? != b"\x00.\x00d\x00a\x00t\x00\x08\x00\x00" {
        return Err("Cabeçalho de táticas não reconhecido".into());
    }
    let presets = r.u16()?;
    for _ in 0..presets {
        r.string()?;
        r.string()?;
        r.skip(TACTIC_BODY_SIZE)?;
    }
    r.kind(&[0])?;
    let managers = r.u16()?;
    let mut tactics = BTreeMap::new();
    let mut previous_selections = 0;
    for ordinal in 1..=managers {
        if ordinal > 1
            && (r.u32()? != u32::from(previous_selections) || r.u16()? != 1 || r.u16()? != ordinal)
        {
            return Err("Separador de técnicos não reconhecido".into());
        }
        if r.u16()? != 1 {
            return Err("Cabeçalho do técnico não reconhecido".into());
        }
        let manager = r.i32()?;
        let person = usize::try_from(manager)
            .ok()
            .and_then(|index| db.people.get(index))
            .filter(|p| p.id == manager && p.kind == 9)
            .ok_or("Referência de técnico humano inválida")?;
        r.kind(&[0])?;
        let selections = r.u16()?;
        previous_selections = selections;
        for _ in 0..selections {
            let prefix = r.take(SELECTION_PREFIX_SIZE)?;
            r.array(4, 2)?;
            r.skip(SELECTION_SUFFIX_SIZE)?;
            r.string()?;
            let name = r.string()?;
            r.string()?;
            let body = r.take(TACTIC_BODY_SIZE)?;
            if r.i32()? != manager || r.u16()? != 0 {
                return Err("Referência de técnico na seleção inválida".into());
            }
            let team_index = r.i32()?;
            let team = usize::try_from(team_index)
                .ok()
                .and_then(|index| db.teams.get(index))
                .ok_or("Referência de equipe na seleção inválida")?;
            if person.team != team_index || team.container_type != 0 || team.squad != 0 {
                continue;
            }
            let club = usize::try_from(team.container_index)
                .ok()
                .and_then(|index| db.clubs.get(index))
                .ok_or("Referência de clube na seleção inválida")?;
            let tactic = positions(body)
                .map(|positions| ClubTactic {
                    name,
                    positions,
                    starters: starters(prefix, db, team.container_index),
                })
                .ok_or_else(|| "Posições da tática não reconhecidas".into());
            if tactics.insert(club.uid, tactic).is_some() {
                return Err("Tática do clube ambígua".into());
            }
        }
    }
    Ok(tactics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_the_saved_tactic_to_the_human_managers_first_team() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../game/user data/games/save_inicial.cm4");
        let save = std::fs::read(path).unwrap();
        let (db, _) = crate::database::load_with_cancel(&save, &|| false).unwrap();
        let tactics = clubs_with_current_tactics(&save, &db).unwrap();
        assert_eq!(tactics.len(), 1);
        let tactic = tactics[&1488].as_ref().unwrap();
        assert_eq!(tactic.name, "4-4-2 Defensive");
        assert_eq!(tactic.positions.len(), 11);
        assert_eq!(tactic.positions.iter().filter(|p| p.row == 2).count(), 4);
        assert_eq!(
            tactic.starters.as_deref(),
            Some([None; STARTING_SLOTS].as_slice())
        );
        let club_index = db.clubs.iter().position(|club| club.uid == 1488).unwrap() as i32;
        let player = db
            .people
            .iter()
            .find(|player| {
                player.playing.is_some()
                    && usize::try_from(player.team).ok().is_some_and(|index| {
                        let team = &db.teams[index];
                        team.container_type == 0 && team.container_index == club_index
                    })
            })
            .unwrap();
        let mut prefix = [0xff; SELECTION_PREFIX_SIZE];
        prefix[4..8].copy_from_slice(&player.id.to_le_bytes());
        let selected = starters(&prefix, &db, club_index).unwrap();
        assert_eq!(selected[0], None);
        assert_eq!(selected[1], Some(player.uid));
        prefix[12..16].copy_from_slice(&player.id.to_le_bytes());
        assert!(starters(&prefix, &db, club_index).is_none());
    }

    #[test]
    fn reads_diablo_from_the_slot_codes_instead_of_the_preset_mask() {
        let mut body = [0; TACTIC_BODY_SIZE];
        body[4..8].copy_from_slice(&[0xc1, 0x06, 0x1b, 0x28]);
        for (index, code) in [
            1u16, 0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x210, 0x208, 0x440, 0x140,
        ]
        .into_iter()
        .enumerate()
        {
            let offset = POSITION_OFFSET + POSITION_STRIDE * index;
            body[offset..offset + 2].copy_from_slice(&code.to_le_bytes());
        }
        let decoded = positions(&body).unwrap();
        assert_eq!(
            decoded
                .iter()
                .map(|p| (p.row, p.column))
                .collect::<Vec<_>>(),
            [
                (0, 2),
                (2, 4),
                (2, 0),
                (2, 3),
                (2, 1),
                (4, 4),
                (4, 0),
                (4, 2),
                (3, 2),
                (6, 3),
                (6, 1)
            ]
        );
        body[POSITION_OFFSET + POSITION_STRIDE..][..2].copy_from_slice(&1u16.to_le_bytes());
        assert!(positions(&body).is_none());
    }
}
