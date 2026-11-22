use crate::{
    Result, attributes, career, clubs,
    database::{self, Date},
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterContract {
    kind: u8,
    entity_type: u8,
    entity_uid: Option<i32>,
    entity: Option<String>,
    weekly_wage_gbp: i32,
    monthly: bool,
    expires: Option<(u16, u16)>,
    appearance_bonus_gbp: Option<i32>,
    goal_bonus_gbp: Option<i32>,
    assist_bonus_gbp: Option<i32>,
    release_clause_gbp: Option<i32>,
    squad_number: i8,
    squad_status: i8,
    transfer_status: Option<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayer {
    uid: i32,
    name: String,
    attributes: BTreeMap<String, i32>,
    contracts: Vec<RosterContract>,
}

struct Snapshot {
    club_name: String,
    date: Date,
    players: BTreeMap<i32, RosterPlayer>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeChange {
    field: String,
    before: Option<i32>,
    after: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayerChange {
    uid: i32,
    name: String,
    attributes: Vec<AttributeChange>,
    contracts_before: Option<Vec<RosterContract>>,
    contracts_after: Option<Vec<RosterContract>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterComparison {
    version: u32,
    club_uid: i32,
    before_club_name: String,
    after_club_name: String,
    before_date: Date,
    after_date: Date,
    joined: Vec<RosterPlayer>,
    left: Vec<RosterPlayer>,
    changed: Vec<RosterPlayerChange>,
    unchanged: usize,
}

fn check_cancelled(cancelled: &impl Fn() -> bool) -> Result<()> {
    if cancelled() {
        return Err("Comparação cancelada".into());
    }
    Ok(())
}

fn snapshot(
    bytes: &[u8],
    label: &str,
    club_uid: i32,
    cancelled: &impl Fn() -> bool,
) -> Result<Snapshot> {
    check_cancelled(cancelled)?;
    let parsed = database::load_with_cancel(bytes, cancelled);
    check_cancelled(cancelled)?;
    let (db, base) = parsed.map_err(|error| {
        format!("{label}: selecione um save .cm4 sem compactação do CM0304 4.1.5. Detalhe: {error}")
    })?;
    let club = clubs::profiles(&db)?
        .into_iter()
        .find(|club| club.uid == club_uid)
        .ok_or_else(|| format!("{label}: clube UID {club_uid} ausente"))?;
    let people = db
        .people
        .iter()
        .filter(|person| {
            person.playing.is_some() && club.player_uids.binary_search(&person.uid).is_ok()
        })
        .map(|person| (person.uid, person))
        .collect::<BTreeMap<_, _>>();
    let mut players = BTreeMap::new();
    for player in db.players_matching(base, |uid| !cancelled() && people.contains_key(&uid))? {
        check_cancelled(cancelled)?;
        let person = people[&player.uid];
        let mut contracts = career::contracts(&db, person)
            .into_iter()
            .map(|view| {
                let contract = &person.contracts[view.index];
                RosterContract {
                    kind: view.kind,
                    entity_type: view.entity_type,
                    entity_uid: career::contract_entity(&db, contract).map(|entity| entity.uid),
                    entity: view.entity,
                    weekly_wage_gbp: contract.weekly_wage,
                    monthly: view.monthly,
                    expires: (!view.monthly)
                        .then_some((contract.expires.year, contract.expires.day)),
                    appearance_bonus_gbp: contract.appearance_bonus,
                    goal_bonus_gbp: contract.goal_bonus,
                    assist_bonus_gbp: contract.assist_bonus,
                    release_clause_gbp: contract
                        .release_clause
                        .as_ref()
                        .map(|clause| clause.amount),
                    squad_number: contract.squad_number,
                    squad_status: contract.squad_status,
                    transfer_status: (view.kind == 0 && view.entity_type == 0)
                        .then_some(view.transfer_status),
                }
            })
            .collect::<Vec<_>>();
        // Contract list order may change as the game rewrites a save.
        contracts.sort_unstable();
        let attributes = player
            .fields
            .into_iter()
            .filter(|(key, _)| {
                attributes::FIELDS[12..].contains(&key.as_ref())
                    || key.starts_with("position.")
                    || key.starts_with("personality.")
                    || matches!(key.as_ref(), "currentAbility" | "potentialAbility")
            })
            .map(|(key, field)| (key.into_owned(), field.value))
            .collect();
        let uid = player.uid;
        if players
            .insert(
                uid,
                RosterPlayer {
                    uid,
                    name: player.name,
                    attributes,
                    contracts,
                },
            )
            .is_some()
        {
            return Err(format!("UID de jogador repetido: {uid}"));
        }
    }
    check_cancelled(cancelled)?;
    Ok(Snapshot {
        club_name: club.name,
        date: db.game_date,
        players,
    })
}

pub fn compare(before: &[u8], after: &[u8], club_uid: i32) -> Result<RosterComparison> {
    compare_with_cancel(before, after, club_uid, || false)
}

pub fn compare_with_cancel(
    before: &[u8],
    after: &[u8],
    club_uid: i32,
    cancelled: impl Fn() -> bool,
) -> Result<RosterComparison> {
    let mut before = snapshot(before, "Save de origem", club_uid, &cancelled)?;
    let mut after = snapshot(after, "Save de comparação", club_uid, &cancelled)?;
    let uids = before
        .players
        .keys()
        .chain(after.players.keys())
        .copied()
        .collect::<BTreeSet<_>>();
    let mut result = RosterComparison {
        version: 1,
        club_uid,
        before_club_name: before.club_name,
        after_club_name: after.club_name,
        before_date: before.date,
        after_date: after.date,
        joined: Vec::new(),
        left: Vec::new(),
        changed: Vec::new(),
        unchanged: 0,
    };
    for uid in uids {
        check_cancelled(&cancelled)?;
        match (before.players.remove(&uid), after.players.remove(&uid)) {
            (None, Some(player)) => result.joined.push(player),
            (Some(player), None) => result.left.push(player),
            (Some(old), Some(new)) => {
                let fields = old
                    .attributes
                    .keys()
                    .chain(new.attributes.keys())
                    .collect::<BTreeSet<_>>();
                let attributes = fields
                    .into_iter()
                    .filter_map(|field| {
                        let before = old.attributes.get(field).copied();
                        let after = new.attributes.get(field).copied();
                        (before != after).then(|| AttributeChange {
                            field: field.clone(),
                            before,
                            after,
                        })
                    })
                    .collect::<Vec<_>>();
                let contracts_changed = old.contracts != new.contracts;
                if attributes.is_empty() && !contracts_changed {
                    result.unchanged += 1;
                } else {
                    result.changed.push(RosterPlayerChange {
                        uid,
                        name: new.name,
                        attributes,
                        contracts_before: contracts_changed.then_some(old.contracts),
                        contracts_after: contracts_changed.then_some(new.contracts),
                    });
                }
            }
            (None, None) => unreachable!("UID collected from the two squads"),
        }
    }
    Ok(result)
}
