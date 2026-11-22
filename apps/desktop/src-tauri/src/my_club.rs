use super::{BatchHealth, Loaded, Pending, Result, Shared, pending, record};
use cm0304_core::{
    database::Database,
    editor::Edit,
    transaction::{self, Transaction, TransferBudgets},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
use tauri::State;

const CASH_INCREASE: i32 = 100_000_000;
const TRANSFER_TARGET: i32 = 200_000_000;
const WAGE_INCREASE: i32 = 25_000;
const CONDITION_TARGET: i32 = 10_000;
const MORALE_TARGET: i32 = 20;

#[derive(Clone, Serialize)]
pub(super) struct ManagedClub {
    pub uid: i32,
    pub name: String,
}

pub(super) fn managed_squads(db: &Database) -> BTreeMap<i32, Vec<i32>> {
    let mut first_teams = BTreeMap::new();
    for person in db.people.iter().filter(|person| person.kind == 9) {
        let Some(team) = usize::try_from(person.team)
            .ok()
            .and_then(|index| db.teams.get(index))
            .filter(|team| team.container_type == 0 && team.squad == 0)
        else {
            continue;
        };
        let Some(club) = usize::try_from(team.container_index)
            .ok()
            .and_then(|index| db.clubs.get(index))
        else {
            continue;
        };
        first_teams
            .entry(club.uid)
            .and_modify(|team| {
                if *team != Some(person.team) {
                    *team = None;
                }
            })
            .or_insert(Some(person.team));
    }
    let mut players_by_team: BTreeMap<i32, Vec<i32>> = first_teams
        .values()
        .flatten()
        .map(|&team| (team, Vec::new()))
        .collect();
    for person in db.people.iter().filter(|person| person.playing.is_some()) {
        if let Some(players) = players_by_team.get_mut(&person.team) {
            players.push(person.uid);
        }
    }
    first_teams
        .into_iter()
        .filter_map(|(club, team)| {
            let mut players = players_by_team.remove(&team?)?;
            players.sort_unstable();
            Some((club, players))
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum Command {
    Cash,
    Transfers,
    Wages,
    Injuries,
    Suspensions,
    Condition,
    Morale,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Shortcut {
    command: Command,
    value: Option<i32>,
    target: Option<i32>,
    affected: usize,
    in_review: usize,
    unavailable: usize,
    reason: Option<&'static str>,
}

struct Action {
    shortcut: Shortcut,
    players: Vec<i32>,
    season_budget: Option<i32>,
}

impl Action {
    fn new(command: Command) -> Self {
        Self {
            shortcut: Shortcut {
                command,
                value: None,
                target: None,
                affected: 0,
                in_review: 0,
                unavailable: 0,
                reason: None,
            },
            players: Vec::new(),
            season_budget: None,
        }
    }
}

#[derive(Serialize)]
pub(super) struct MyClub {
    shortcuts: Vec<Shortcut>,
}

#[derive(Serialize)]
pub(super) struct StagedAction {
    plan: Pending,
    club: MyClub,
    applied: Shortcut,
}

fn financial_action(data: &Loaded, plan: &Transaction, uid: i32, command: Command) -> Action {
    let mut action = Action::new(command);
    let shortcut = &mut action.shortcut;
    let Some(info) = data.club_info.get(&uid) else {
        shortcut.reason = Some("Dados financeiros ausentes");
        return action;
    };
    let (original, pending) = match command {
        Command::Cash => (info.balance, plan.club_balances.get(&uid).copied()),
        Command::Transfers => {
            let pending = plan.club_transfer_budgets.get(&uid);
            action.season_budget = pending
                .map(|budget| budget.season)
                .or(info.transfer_budget_season);
            (
                info.transfer_budget_remaining,
                pending.map(|budget| budget.remaining),
            )
        }
        Command::Wages => (
            info.wage_budget_weekly,
            plan.club_wage_budgets.get(&uid).copied(),
        ),
        _ => unreachable!(),
    };
    shortcut.value = pending.or(original);
    shortcut.in_review = usize::from(pending.is_some() && pending != original);
    if !info.balance_editable {
        shortcut.reason = Some("Formato financeiro sem suporte à edição");
        return action;
    }
    let Some(value) = shortcut.value else {
        shortcut.reason = Some("Campo financeiro ausente");
        return action;
    };
    shortcut.target = match command {
        Command::Cash => value.checked_add(CASH_INCREASE),
        Command::Transfers => {
            if !action.season_budget.is_some_and(|budget| budget >= 0) {
                shortcut.reason = Some("Orçamento da temporada indisponível");
                return action;
            }
            if value >= TRANSFER_TARGET {
                shortcut.reason = Some("A verba já alcança £200 milhões");
                return action;
            }
            Some(TRANSFER_TARGET)
        }
        Command::Wages => value
            .checked_add(WAGE_INCREASE)
            .filter(|target| (0..=i32::MAX / 4).contains(target)),
        _ => unreachable!(),
    };
    if shortcut.target.is_none() {
        shortcut.reason = Some("O valor ficaria fora do intervalo permitido");
    }
    action
}

fn roster_actions(data: &Loaded, plan: &Transaction, squad: &[i32]) -> [Action; 4] {
    let [mut injuries, mut suspensions, mut condition, mut morale] = [
        Command::Injuries,
        Command::Suspensions,
        Command::Condition,
        Command::Morale,
    ]
    .map(Action::new);
    let effective: BTreeMap<_, _> = plan
        .edits
        .iter()
        .filter(|edit| matches!(edit.attribute.as_str(), "state.condition" | "state.morale"))
        .map(|edit| ((edit.uid, edit.attribute.as_str()), edit.value))
        .collect();
    let players: BTreeMap<_, _> = data
        .players
        .iter()
        .filter(|player| squad.binary_search(&player.uid).is_ok())
        .map(|player| (player.uid, player))
        .collect();
    for &uid in squad {
        let health = BatchHealth::from_save(data, uid);
        for (action, count, pending) in [
            (&mut injuries, health.injury_count, &plan.heal_injuries),
            (
                &mut suspensions,
                health.suspension_count,
                &plan.clear_suspensions,
            ),
        ] {
            if count > 0 {
                if pending.contains(&uid) {
                    action.shortcut.in_review += 1;
                } else {
                    action.players.push(uid);
                }
            }
        }
        for (action, key, target) in [
            (&mut condition, "state.condition", CONDITION_TARGET),
            (&mut morale, "state.morale", MORALE_TARGET),
        ] {
            let Some(field) = players.get(&uid).and_then(|player| player.fields.get(key)) else {
                action.shortcut.unavailable += 1;
                continue;
            };
            let value = effective.get(&(uid, key)).copied().unwrap_or(field.value);
            if !(field.min..=field.max).contains(&value)
                || !(field.min..=field.max).contains(&target)
            {
                action.shortcut.unavailable += 1;
            } else if value != target {
                action.players.push(uid);
            } else if value != field.value {
                action.shortcut.in_review += 1;
            }
        }
    }
    [injuries, suspensions, condition, morale].map(|mut action| {
        action.shortcut.affected = action.players.len();
        if squad.is_empty() {
            action.shortcut.reason = Some("Equipe principal sem jogadores");
        }
        action
    })
}

fn actions(data: &Loaded, plan: &Transaction, uid: i32) -> Result<Vec<Action>> {
    let squad = data
        .managed_squads
        .get(&uid)
        .ok_or("O clube não é controlado por um técnico humano neste save")?;
    Ok([Command::Cash, Command::Transfers, Command::Wages]
        .map(|command| financial_action(data, plan, uid, command))
        .into_iter()
        .chain(roster_actions(data, plan, squad))
        .collect())
}

fn detail(data: &Loaded, plan: &Transaction, uid: i32) -> Result<MyClub> {
    Ok(MyClub {
        shortcuts: actions(data, plan, uid)?
            .into_iter()
            .map(|action| action.shortcut)
            .collect(),
    })
}

#[tauri::command]
pub(super) fn my_club_detail(uid: i32, state: State<Shared>) -> Result<MyClub> {
    let session = state.lock().map_err(|e| e.to_string())?;
    let data = session.loaded.as_ref().ok_or("Abra um save primeiro")?;
    detail(data, &session.plan, uid)
}

fn add_to_plan(data: &Loaded, plan: &mut Transaction, uid: i32, action: &Action) -> Result<()> {
    let shortcut = &action.shortcut;
    if let Some(reason) = shortcut.reason {
        return Err(reason.into());
    }
    match shortcut.command {
        Command::Cash => {
            let target = shortcut.target.ok_or("Saldo indisponível")?;
            update_finance(
                &mut plan.club_balances,
                uid,
                data.club_info[&uid].balance,
                target,
            );
        }
        Command::Transfers => {
            let info = &data.club_info[&uid];
            let original = info
                .transfer_budget_season
                .zip(info.transfer_budget_remaining)
                .map(|(season, remaining)| TransferBudgets { season, remaining });
            update_finance(
                &mut plan.club_transfer_budgets,
                uid,
                original,
                TransferBudgets {
                    season: action
                        .season_budget
                        .ok_or("Orçamento da temporada indisponível")?,
                    remaining: shortcut
                        .target
                        .ok_or("Verba de transferências indisponível")?,
                },
            );
        }
        Command::Wages => {
            let target = shortcut.target.ok_or("Orçamento salarial indisponível")?;
            update_finance(
                &mut plan.club_wage_budgets,
                uid,
                data.club_info[&uid].wage_budget_weekly,
                target,
            );
        }
        Command::Injuries => plan.heal_injuries.extend(action.players.iter().copied()),
        Command::Suspensions => plan
            .clear_suspensions
            .extend(action.players.iter().copied()),
        Command::Condition | Command::Morale => {
            let (field, value) = if shortcut.command == Command::Condition {
                ("state.condition", CONDITION_TARGET)
            } else {
                ("state.morale", MORALE_TARGET)
            };
            plan.merge_fields(
                &data.players,
                action
                    .players
                    .iter()
                    .map(|&uid| Edit {
                        uid,
                        attribute: field.into(),
                        value,
                        policy: None,
                    })
                    .collect(),
            )?;
        }
    }
    Ok(())
}

fn update_finance<T: PartialEq>(
    fields: &mut BTreeMap<i32, T>,
    uid: i32,
    original: Option<T>,
    target: T,
) {
    if original.as_ref() == Some(&target) {
        fields.remove(&uid);
    } else {
        fields.insert(uid, target);
    }
}

#[tauri::command]
pub(super) async fn stage_my_club_action(
    uid: i32,
    command: Command,
    state: State<'_, Shared>,
) -> Result<StagedAction> {
    let (source, baseline) = {
        let session = state.lock().map_err(|e| e.to_string())?;
        (
            session.loaded.clone().ok_or("Abra um save primeiro")?,
            session.plan.clone(),
        )
    };
    let data = Arc::clone(&source);
    let mut next = baseline.clone();
    let (next, club, applied) = tauri::async_runtime::spawn_blocking(move || -> Result<_> {
        let action = actions(&data, &next, uid)?
            .into_iter()
            .find(|action| action.shortcut.command == command)
            .ok_or("Atalho indisponível")?;
        if action.shortcut.reason.is_none()
            && action.shortcut.target.is_none()
            && action.players.is_empty()
        {
            return Err("Nenhum jogador precisa desse ajuste".into());
        }
        add_to_plan(&data, &mut next, uid, &action)?;
        if !next.is_empty() {
            transaction::apply(&data.bytes, &next)?;
        }
        let club = detail(&data, &next, uid)?;
        Ok((next, club, action.shortcut))
    })
    .await
    .map_err(|e| e.to_string())??;
    let mut session = state.lock().map_err(|e| e.to_string())?;
    if session.plan != baseline
        || !session
            .loaded
            .as_ref()
            .is_some_and(|data| Arc::ptr_eq(data, &source))
    {
        return Err("A sessão mudou durante a validação do atalho. Tente novamente.".into());
    }
    if next != session.plan {
        record(&mut session);
        session.plan = next;
    }
    Ok(StagedAction {
        plan: pending(&session),
        club,
        applied,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClubInfo, Discipline};
    use cm0304_core::{database, health};
    use std::path::Path;

    #[test]
    fn quick_actions_apply_to_the_managed_first_team_and_preserve_other_finances() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../game/user data/games/save_modificado.cm4");
        let bytes = Arc::new(std::fs::read(&source).unwrap());
        let (db, base) = database::load(&bytes).unwrap();
        let managed_squads = managed_squads(&db);
        let uid = 1488;
        assert_eq!(managed_squads.len(), 1);
        let squad = &managed_squads[&uid];
        assert!(squad.contains(&1855)); // Quim
        assert!(!squad.contains(&736046)); // Quínio, Braga B
        let original = db
            .clubs
            .iter()
            .find(|club| club.uid == uid)
            .unwrap()
            .details
            .as_ref()
            .unwrap();
        let data = Loaded {
            source,
            bytes: Arc::clone(&bytes),
            players: db.players(base).unwrap(),
            clubs: Vec::new(),
            managed_squads,
            club_info: BTreeMap::from([(
                uid,
                ClubInfo {
                    balance: original.balance,
                    balance_editable: original.finance_kind == 1,
                    transfer_budget_season: original.transfer_budget_season,
                    transfer_budget_remaining: original.transfer_budget_remaining,
                    wage_budget_weekly: original.wage_budget_weekly,
                    current_wages_weekly: None,
                    payroll_expenses: None,
                    finance_summary: None,
                    sell_on_clauses: Vec::new(),
                    transfer_installments: None,
                    reputation: None,
                    player_contract_wages_weekly: 0,
                    loan_wage_offset_weekly: 0,
                    training_raw: 0,
                    youth_setup_raw: 0,
                    average_attendance: 0,
                },
            )]),
            club_stadiums: BTreeMap::new(),
            club_contract_dates: BTreeMap::new(),
            club_loan_wages: BTreeMap::new(),
            club_tactics: Ok(BTreeMap::new()),
            nations: Vec::new(),
            game_date: db.game_date.clone(),
            search_keys: Vec::new(),
            staff: Vec::new(),
            staff_search_keys: Vec::new(),
            contracts: BTreeMap::new(),
            market_sources: BTreeMap::new(),
            market_journal_supported: false,
            injuries: db
                .people
                .iter()
                .filter(|p| p.playing.is_some())
                .map(|p| (p.uid, health::injuries(&db, p)))
                .collect(),
            discipline: db
                .people
                .iter()
                .filter_map(|p| {
                    let player = p.playing.as_ref()?;
                    Some((
                        p.uid,
                        Discipline {
                            suspensions: player.health_records.bans.len(),
                            booking_records: player.health_records.bookings.len(),
                        },
                    ))
                })
                .collect(),
        };
        let mut plan = Transaction::default();
        assert!(actions(&data, &plan, -1).is_err());
        let shortcuts = actions(&data, &plan, uid).unwrap();
        assert_eq!(shortcuts.len(), 7);
        assert!(
            shortcuts
                .iter()
                .any(|action| action.shortcut.command == Command::Condition
                    && !action.players.is_empty())
        );
        for action in &shortcuts {
            add_to_plan(&data, &mut plan, uid, action).unwrap();
        }
        assert_eq!(
            plan.club_balances[&uid],
            original.balance.unwrap() + CASH_INCREASE
        );
        assert_eq!(plan.club_transfer_budgets[&uid].remaining, TRANSFER_TARGET);
        assert_eq!(
            plan.club_transfer_budgets[&uid].season,
            original.transfer_budget_season.unwrap()
        );
        assert_eq!(
            plan.club_wage_budgets[&uid],
            original.wage_budget_weekly.unwrap() + WAGE_INCREASE
        );
        let squad = &data.managed_squads[&uid];
        assert!(
            plan.edits
                .iter()
                .map(|edit| &edit.uid)
                .chain(&plan.heal_injuries)
                .chain(&plan.clear_suspensions)
                .all(|uid| squad.contains(uid))
        );
        let (result, report) = transaction::apply(&bytes, &plan).unwrap();
        let (edited, _) = database::load(&result).unwrap();
        let edited_club = edited
            .clubs
            .iter()
            .find(|club| club.uid == uid)
            .unwrap()
            .details
            .as_ref()
            .unwrap();
        assert_eq!(edited_club.transfer_budget_remaining, Some(200_000_000));
        assert_eq!(
            edited_club.transfer_budget_season,
            original.transfer_budget_season
        );
        assert_ne!(report.source_sha256, report.copy_sha256);
        let next = detail(&data, &plan, uid).unwrap();
        assert!(next.shortcuts.iter().all(|shortcut| shortcut.affected == 0));
        assert_eq!(next.shortcuts[1].target, None);
    }
}
