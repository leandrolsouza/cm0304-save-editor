//! Club identities and direct roster links. No financial or infrastructure writes.
use crate::{
    Result,
    database::{Database, Stadium},
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubStadium {
    #[serde(flatten)]
    pub stadium: Stadium,
    pub first_team_club_uids: Vec<i32>,
}

pub fn stadiums_by_club(db: &Database) -> BTreeMap<i32, ClubStadium> {
    let mut by_stadium = BTreeMap::<usize, BTreeSet<i32>>::new();
    let mut first_teams = BTreeMap::new();
    for team in db
        .teams
        .iter()
        .filter(|team| team.squad == 0 && team.container_type == 0)
    {
        let Some(club) = usize::try_from(team.container_index)
            .ok()
            .and_then(|i| db.clubs.get(i))
        else {
            continue;
        };
        first_teams
            .entry(club.uid)
            .and_modify(|index| *index = None)
            .or_insert(team.stadium_index);
        if let Some(index) = team.stadium_index {
            by_stadium.entry(index).or_default().insert(club.uid);
        }
    }
    first_teams
        .into_iter()
        .filter_map(|(uid, index)| {
            let index = index?;
            Some((
                uid,
                ClubStadium {
                    stadium: db.stadiums.get(index)?.clone(),
                    first_team_club_uids: by_stadium.get(&index)?.iter().copied().collect(),
                },
            ))
        })
        .collect()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubProfile {
    pub uid: i32,
    pub id: i32,
    pub index: usize,
    pub name: String,
    pub short_name: String,
    pub player_uids: Vec<i32>,
    pub staff_uids: Vec<i32>,
}

pub fn profiles(db: &Database) -> Result<Vec<ClubProfile>> {
    let mut seen = BTreeSet::new();
    let mut clubs = db
        .clubs
        .iter()
        .enumerate()
        .map(|(index, c)| {
            if !seen.insert(c.uid) {
                return Err(format!("UID de clube repetido: {}", c.uid));
            }
            Ok(ClubProfile {
                uid: c.uid,
                id: c.id,
                index,
                name: c.name.clone(),
                short_name: c.short_name.clone(),
                player_uids: Vec::new(),
                staff_uids: Vec::new(),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    for person in &db.people {
        let is_player = person.playing.is_some();
        let is_staff = matches!(person.kind, 1 | 3 | 9);
        if (!is_player && !is_staff) || person.team == -1 {
            continue;
        }
        let team = usize::try_from(person.team)
            .ok()
            .and_then(|i| db.teams.get(i))
            .ok_or_else(|| format!("Pessoa {}: equipe inválida", person.uid))?;
        if team.container_type == 1 {
            if usize::try_from(team.container_index)
                .ok()
                .and_then(|i| db.nations.get(i))
                .is_none()
            {
                return Err(format!("Pessoa {}: seleção inválida", person.uid));
            }
            continue;
        }
        if team.container_type != 0 {
            return Err(format!("Pessoa {}: tipo de equipe inválido", person.uid));
        }
        let club = usize::try_from(team.container_index)
            .ok()
            .and_then(|i| clubs.get_mut(i))
            .ok_or_else(|| format!("Pessoa {}: clube inválido", person.uid))?;
        if is_player {
            club.player_uids.push(person.uid);
        }
        if is_staff {
            club.staff_uids.push(person.uid);
        }
    }
    for club in &mut clubs {
        club.player_uids.sort_unstable();
        club.staff_uids.sort_unstable();
    }
    Ok(clubs)
}
