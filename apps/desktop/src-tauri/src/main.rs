#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cm0304_core::{
    career::{self, ContractView},
    container::hash,
    database::{self, Date, Player},
    editor::Edit,
    health::{self, InjuryView},
    search,
    staff::{self, Staff},
    transaction::{self, Transaction},
};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{Arc, Mutex, atomic::AtomicBool},
};
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
type Result<T> = std::result::Result<T, String>;

struct Loaded {
    source: PathBuf,
    bytes: Arc<Vec<u8>>,
    players: Vec<Player>,
    clubs: Vec<cm0304_core::clubs::ClubProfile>,
    club_info: BTreeMap<i32, ClubInfo>,
    club_contract_dates: BTreeMap<i32, Vec<ClubContractDate>>,
    nations: Vec<String>,
    game_date: Date,
    search_keys: Vec<String>,
    staff: Vec<Staff>,
    staff_search_keys: Vec<String>,
    contracts: BTreeMap<i32, Vec<ContractView>>,
    injuries: BTreeMap<i32, Vec<InjuryView>>,
    suspensions: BTreeMap<i32, usize>,
}
#[derive(Default)]
struct Session {
    loaded: Option<Arc<Loaded>>,
    plan: Transaction,
    undo: Vec<Transaction>,
    redo: Vec<Transaction>,
}
type Shared = Mutex<Session>;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Summary {
    name: String,
    path: String,
    players: usize,
    clubs: usize,
    staff: usize,
    sha256: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    uid: i32,
    name: String,
    club: Option<String>,
    age: Option<u16>,
    ca: i16,
    pa: i16,
}
#[derive(Serialize)]
struct Page {
    total: usize,
    rows: Vec<Row>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StaffRow {
    uid: i32,
    name: String,
    team: Option<String>,
    role: String,
    national_team: Option<String>,
    age: Option<u16>,
    ca: i16,
    pa: i16,
    is_player: bool,
}
#[derive(Serialize)]
struct StaffPage {
    total: usize,
    rows: Vec<StaffRow>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StaffDetail {
    profile: Staff,
    age: Option<u16>,
    pending_player_fields: usize,
    editable: Vec<String>,
    pending_header: Option<cm0304_core::staff_header::Edit>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Detail {
    player: Player,
    effective: Player,
    age: Option<u16>,
    contracts: Vec<ContractView>,
    editable: Vec<String>,
    pending: Vec<Edit>,
    nations: Vec<String>,
    injuries: Vec<InjuryView>,
    heal_pending: bool,
    suspension_count: usize,
    suspension_pending: bool,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Pending {
    club_transfer_budget_field_count: usize,
    edits: Vec<Edit>,
    staff_edits: Vec<cm0304_core::staff_editor::Edit>,
    club_balances: BTreeMap<i32, i32>,
    club_transfer_budgets: BTreeMap<i32, transaction::TransferBudgets>,
    club_training: BTreeMap<i32, u8>,
    club_reputation: BTreeMap<i32, i16>,
    club_youth_setup: BTreeMap<i32, u8>,
    staff_header_edits: Vec<cm0304_core::staff_header::Edit>,
    heal_injuries: Vec<i32>,
    clear_suspensions: Vec<i32>,
    undo: bool,
    redo: bool,
}
fn pending(s: &Session) -> Pending {
    let club_transfer_budget_field_count = s
        .plan
        .club_transfer_budgets
        .iter()
        .filter_map(|(uid, targets)| {
            let original = s.loaded.as_ref()?.club_info.get(uid)?;
            Some(
                usize::from(original.transfer_budget_season != Some(targets.season))
                    + usize::from(original.transfer_budget_remaining != Some(targets.remaining)),
            )
        })
        .sum();
    Pending {
        club_transfer_budget_field_count,
        edits: s.plan.edits.clone(),
        staff_edits: s.plan.staff_edits.clone(),
        club_balances: s.plan.club_balances.clone(),
        club_transfer_budgets: s.plan.club_transfer_budgets.clone(),
        club_training: s.plan.club_training.clone(),
        club_reputation: s.plan.club_reputation.clone(),
        club_youth_setup: s.plan.club_youth_setup.clone(),
        staff_header_edits: s.plan.staff_header_edits.clone(),
        heal_injuries: s.plan.heal_injuries.iter().copied().collect(),
        clear_suspensions: s.plan.clear_suspensions.iter().copied().collect(),
        undo: !s.undo.is_empty(),
        redo: !s.redo.is_empty(),
    }
}
fn record(s: &mut Session) {
    s.undo.push(s.plan.clone());
    s.redo.clear();
}
fn loaded(state: &State<Shared>) -> Result<Arc<Loaded>> {
    state
        .lock()
        .map_err(|e| e.to_string())?
        .loaded
        .clone()
        .ok_or("Abra um save primeiro".into())
}

#[tauri::command]
async fn open_save(app: tauri::AppHandle, state: State<'_, Shared>) -> Result<Option<Summary>> {
    let result =
        tauri::async_runtime::spawn_blocking(move || -> Result<Option<(Arc<Loaded>, Summary)>> {
            let Some(file) = app
                .dialog()
                .file()
                .add_filter("Save CM0304", &["cm4"])
                .blocking_pick_file()
            else {
                return Ok(None);
            };
            let source = file.into_path().map_err(|e| e.to_string())?;
            let bytes = std::fs::read(&source).map_err(|e| e.to_string())?;
            let (db, base) = database::load(&bytes)?;
            let players = db.players(base)?;
            let clubs = cm0304_core::clubs::profiles(&db)?;
            let player_wages = career::player_wages_by_club(&db);
            let loan_offsets = career::loan_wage_offsets_by_club(&db);
            let mut reputations = BTreeMap::new();
            for team in db
                .teams
                .iter()
                .filter(|t| t.container_type == 0 && t.squad == 0)
            {
                reputations
                    .entry(team.container_index)
                    .and_modify(|value| *value = None)
                    .or_insert(Some(team.reputation));
            }
            let club_info = db
                .clubs
                .iter()
                .enumerate()
                .filter_map(|(index, club)| {
                    let d = club.details.as_ref()?;
                    Some((
                        club.uid,
                        ClubInfo {
                            balance: d.balance,
                            reputation: reputations.get(&(index as i32)).copied().flatten(),
                            loan_wage_offset_weekly: loan_offsets
                                .get(&index)
                                .copied()
                                .unwrap_or_default(),
                            player_contract_wages_weekly: player_wages
                                .get(&index)
                                .copied()
                                .unwrap_or_default(),
                            transfer_budget_season: d.transfer_budget_season,
                            transfer_budget_remaining: d.transfer_budget_remaining,
                            balance_editable: d.finance_kind == 1,
                            training_raw: d.training_raw,
                            youth_setup_raw: d.youth_setup_raw,
                            average_attendance: d.average_attendance,
                        },
                    ))
                })
                .collect();
            let mut club_contract_dates = BTreeMap::<i32, Vec<ClubContractDate>>::new();

            let player_names = players
                .iter()
                .map(|p| (p.uid, p.name.as_str()))
                .collect::<BTreeMap<_, _>>();
            for person in &db.people {
                let Some(name) = player_names.get(&person.uid) else {
                    continue;
                };
                for (index, contract) in person.contracts.iter().enumerate() {
                    if contract.entity_type != 0 || !matches!(contract.kind, 0 | 1) {
                        continue;
                    }
                    let Some(club) = usize::try_from(contract.entity_index)
                        .ok()
                        .and_then(|index| db.clubs.get(index))
                    else {
                        continue;
                    };
                    club_contract_dates
                        .entry(club.uid)
                        .or_default()
                        .push(ClubContractDate {
                            uid: person.uid,
                            name: (*name).to_owned(),
                            expires: contract.expires.clone(),
                            index,
                            loan: contract.kind == 1,
                        });
                }
            }

            let staff = staff::profiles(&db, &bytes, base)
                .map_err(|error| format!("Falha ao ler staff: {error}"))?;
            let staff_search_keys = staff
                .iter()
                .map(|p| {
                    search::key(&format!(
                        "{} {} {} {} {} {}",
                        p.uid,
                        p.name,
                        p.team.as_deref().unwrap_or(""),
                        p.club_job,
                        p.national_team.as_deref().unwrap_or(""),
                        p.national_job
                    ))
                })
                .collect();
            let contracts = db
                .people
                .iter()
                .filter(|p| p.playing.is_some())
                .map(|p| (p.uid, career::contracts(&db, p)))
                .collect();
            let nations = db.nations.iter().map(|n| n.name.clone()).collect();
            let injuries = db
                .people
                .iter()
                .filter(|p| p.playing.is_some())
                .map(|p| (p.uid, health::injuries(&db, p)))
                .collect();
            let suspensions = db
                .people
                .iter()
                .filter_map(|p| {
                    p.playing
                        .as_ref()
                        .map(|v| (p.uid, v.health_records.bans.len()))
                })
                .collect();
            let search_keys = players
                .iter()
                .map(|p| {
                    search::key(&format!(
                        "{} {} {} {} {}",
                        p.uid,
                        p.name,
                        p.first_name,
                        p.surname,
                        p.club.as_deref().unwrap_or("")
                    ))
                })
                .collect();
            let summary = Summary {
                name: source
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into(),
                path: source.display().to_string(),
                players: players.len(),
                clubs: clubs.len(),
                staff: staff.len(),
                sha256: hash(&bytes),
            };
            Ok(Some((
                Arc::new(Loaded {
                    source,
                    bytes: Arc::new(bytes),
                    players,
                    clubs,
                    club_info,
                    club_contract_dates,
                    nations,
                    game_date: db.game_date,
                    search_keys,
                    staff,
                    staff_search_keys,
                    contracts,
                    injuries,
                    suspensions,
                }),
                summary,
            )))
        })
        .await
        .map_err(|e| e.to_string())??;
    if let Some((data, summary)) = result {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        *s = Session {
            loaded: Some(data),
            ..Default::default()
        };
        Ok(Some(summary))
    } else {
        Ok(None)
    }
}
#[tauri::command]
fn search_staff(query: String, page: usize, state: State<Shared>) -> Result<StaffPage> {
    let data = loaded(&state)?;
    let q = search::key(query.trim());
    let matches = data
        .staff
        .iter()
        .zip(&data.staff_search_keys)
        .filter(|(_, key)| key.contains(&q))
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    Ok(StaffPage {
        total: matches.len(),
        rows: matches
            .into_iter()
            .skip(page.saturating_mul(50))
            .take(50)
            .map(|p| StaffRow {
                uid: p.uid,
                name: p.name.clone(),
                team: p.team.clone(),
                role: if p.team.is_none() && p.national_team.is_some() {
                    p.national_job
                } else {
                    p.club_job
                }
                .to_owned(),
                national_team: p.national_team.clone(),
                age: p
                    .date_of_birth
                    .as_ref()
                    .and_then(|d| d.age_on(&data.game_date)),
                ca: p.current_ability,
                pa: p.potential_ability,
                is_player: p.is_player,
            })
            .collect(),
    })
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubRow {
    uid: i32,
    name: String,
    short_name: String,
    players: usize,
    staff: usize,
}
#[derive(Serialize)]
struct ClubPage {
    total: usize,
    rows: Vec<ClubRow>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubStaff {
    uid: i32,
    name: String,
    role: String,
    is_player: bool,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubInfo {
    reputation: Option<i16>,
    balance: Option<i32>,
    balance_editable: bool,
    player_contract_wages_weekly: i64,
    loan_wage_offset_weekly: i64,
    transfer_budget_season: Option<i32>,
    transfer_budget_remaining: Option<i32>,
    training_raw: u8,
    youth_setup_raw: u8,
    average_attendance: i32,
}
struct ClubContractDate {
    uid: i32,
    name: String,
    expires: Date,
    index: usize,
    loan: bool,
}

#[derive(Serialize)]
struct ClubExpiry {
    uid: i32,
    index: usize,
    name: String,
    expires: Date,
    days: i32,
    loan: bool,
}

fn date_ordinal(date: &Date) -> Option<i32> {
    date.month_day()?;
    let y = i32::from(date.year).checked_sub(1)?;
    (y >= 0).then(|| y * 365 + y / 4 - y / 100 + y / 400 + i32::from(date.day))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubDetail {
    info: Option<ClubInfo>,
    expiring_contracts: Vec<ClubExpiry>,
    profile: cm0304_core::clubs::ClubProfile,
    players: Vec<Row>,
    staff: Vec<ClubStaff>,
}

#[tauri::command]
fn search_clubs(query: String, page: usize, state: State<Shared>) -> Result<ClubPage> {
    let data = loaded(&state)?;
    let q = search::key(query.trim());
    let uid = q
        .strip_prefix("uid:")
        .map(|n| n.parse::<i32>().map_err(|_| "UID de clube inválido"))
        .transpose()?;
    let found = data
        .clubs
        .iter()
        .filter(|c| {
            uid.map_or_else(
                || search::key(&format!("{} {} {}", c.uid, c.name, c.short_name)).contains(&q),
                |u| c.uid == u,
            )
        })
        .collect::<Vec<_>>();
    Ok(ClubPage {
        total: found.len(),
        rows: found
            .into_iter()
            .skip(page.saturating_mul(50))
            .take(50)
            .map(|c| ClubRow {
                uid: c.uid,
                name: c.name.clone(),
                short_name: c.short_name.clone(),
                players: c.player_uids.len(),
                staff: c.staff_uids.len(),
            })
            .collect(),
    })
}
fn project_club_expiries(
    records: &[ClubContractDate],
    game_date: &Date,
    edits: &[Edit],
) -> Vec<ClubExpiry> {
    let dates = edits
        .iter()
        .filter_map(|edit| {
            let (index, field) = edit
                .attribute
                .strip_prefix("contract.")?
                .split_once(".expires.")?;
            let index = index.parse::<usize>().ok()?;
            let value = u16::try_from(edit.value).ok()?;
            Some(((edit.uid, index, field), value))
        })
        .collect::<BTreeMap<_, _>>();
    let today = date_ordinal(game_date);
    let mut expiring_contracts = records
        .iter()
        .filter_map(|record| {
            let expires = Date {
                year: dates
                    .get(&(record.uid, record.index, "year"))
                    .copied()
                    .unwrap_or(record.expires.year),
                day: dates
                    .get(&(record.uid, record.index, "day"))
                    .copied()
                    .unwrap_or(record.expires.day),
            };
            if career::is_monthly(&expires) {
                return None;
            }
            let days = date_ordinal(&expires)? - today?;
            (0..=90).contains(&days).then(|| ClubExpiry {
                uid: record.uid,
                index: record.index,
                name: record.name.clone(),
                expires,
                days,
                loan: record.loan,
            })
        })
        .collect::<Vec<_>>();
    expiring_contracts.sort_by_key(|row| (row.days, row.uid, row.loan));
    expiring_contracts
}

#[tauri::command]
fn club_detail(uid: i32, state: State<Shared>) -> Result<ClubDetail> {
    let (data, edits) = {
        let session = state.lock().map_err(|e| e.to_string())?;
        (
            session.loaded.clone().ok_or("Abra um save primeiro")?,
            session.plan.edits.clone(),
        )
    };
    let records = data
        .club_contract_dates
        .get(&uid)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let expiring_contracts = project_club_expiries(records, &data.game_date, &edits);
    let c = data
        .clubs
        .iter()
        .find(|c| c.uid == uid)
        .ok_or("Clube não encontrado")?;
    let players = data
        .players
        .iter()
        .filter(|p| c.player_uids.binary_search(&p.uid).is_ok())
        .map(|p| Row {
            uid: p.uid,
            name: p.name.clone(),
            club: p.club.clone(),
            age: p.date_of_birth.age_on(&data.game_date),
            ca: p.current_ability,
            pa: p.potential_ability,
        })
        .collect();
    let staff = data
        .staff
        .iter()
        .filter(|p| c.staff_uids.binary_search(&p.uid).is_ok())
        .map(|p| ClubStaff {
            uid: p.uid,
            name: p.name.clone(),
            role: p.club_job.to_owned(),
            is_player: p.is_player,
        })
        .collect();
    Ok(ClubDetail {
        info: data.club_info.get(&uid).cloned(),
        expiring_contracts,
        profile: c.clone(),
        players,
        staff,
    })
}
#[tauri::command]
async fn staff_detail(uid: i32, state: State<'_, Shared>) -> Result<StaffDetail> {
    let data = loaded(&state)?;
    let mut profile = data
        .staff
        .iter()
        .find(|p| p.uid == uid)
        .cloned()
        .ok_or("Staff não encontrado")?;
    let plan = state.lock().map_err(|e| e.to_string())?.plan.clone();
    let pending_player_fields = plan.edits.iter().filter(|e| e.uid == uid).count();
    let pending_header = plan
        .staff_header_edits
        .iter()
        .find(|h| h.uid == uid)
        .cloned();
    if pending_player_fields > 0
        || plan.staff_edits.iter().any(|e| e.uid == uid)
        || pending_header.is_some()
    {
        let bytes = data.bytes.clone();
        profile = tauri::async_runtime::spawn_blocking(move || -> Result<Staff> {
            let (output, _) = transaction::apply(&bytes, &plan)?;
            let (db, base) = database::load(&output)?;
            staff::profiles(&db, &output, base)?
                .into_iter()
                .find(|p| p.uid == uid)
                .ok_or("Staff ausente após projeção".into())
        })
        .await
        .map_err(|e| e.to_string())??;
    }
    let age = profile
        .date_of_birth
        .as_ref()
        .and_then(|date| date.age_on(&data.game_date));
    let mut editable = if profile.attributes.is_some() {
        cm0304_core::staff_editor::EDITABLE
            .iter()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    if profile.personality.is_some() {
        editable.extend(
            database::PERSONALITY_FIELDS
                .iter()
                .map(|k| format!("personality.{k}")),
        );
    }
    Ok(StaffDetail {
        editable,
        profile,
        age,
        pending_player_fields,
        pending_header,
    })
}
#[tauri::command]
async fn stage_staff_edits(
    edits: Vec<cm0304_core::staff_editor::Edit>,
    header: Option<cm0304_core::staff_header::Edit>,
    state: State<'_, Shared>,
) -> Result<Pending> {
    let data = loaded(&state)?;
    let previous = state.lock().map_err(|e| e.to_string())?.plan.clone();
    let mut next = previous.clone();
    next.merge_staff(&data.staff, edits, header)?;
    if !next.is_empty() {
        let bytes = data.bytes.clone();
        let plan = next.clone();
        tauri::async_runtime::spawn_blocking(move || transaction::apply(&bytes, &plan).map(|_| ()))
            .await
            .map_err(|e| e.to_string())??;
    }
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if s.plan != previous || !s.loaded.as_ref().is_some_and(|p| Arc::ptr_eq(p, &data)) {
        return Err("A sessão mudou durante a validação; tente novamente".into());
    }
    if s.plan != next {
        record(&mut s);
        s.plan = next;
    }
    Ok(pending(&s))
}
#[tauri::command]
fn stage_club_balance(
    uid: i32,
    value: i32,
    budgets: Option<transaction::TransferBudgets>,
    state: State<Shared>,
) -> Result<Pending> {
    let mut session = state.lock().map_err(|e| e.to_string())?;
    let info = session
        .loaded
        .as_ref()
        .and_then(|d| d.club_info.get(&uid))
        .filter(|info| info.balance_editable)
        .ok_or("Clube sem registro financeiro completo")?;
    let original = info.balance.ok_or("Balanço indisponível")?;
    let target = (value != original).then_some(value);
    let mut budget_target = session.plan.club_transfer_budgets.get(&uid).cloned();
    if let Some(budgets) = budgets {
        if budgets.season < 0 || budgets.remaining < 0 {
            return Err("Informe orçamentos não negativos".into());
        }
        let original = info
            .transfer_budget_season
            .zip(info.transfer_budget_remaining)
            .ok_or("Orçamentos indisponíveis")?;
        budget_target = ((budgets.season, budgets.remaining) != original).then_some(budgets);
    }
    if session.plan.club_balances.get(&uid).copied() != target
        || session.plan.club_transfer_budgets.get(&uid) != budget_target.as_ref()
    {
        record(&mut session);
        if let Some(value) = target {
            session.plan.club_balances.insert(uid, value);
        } else {
            session.plan.club_balances.remove(&uid);
        }
        if let Some(budgets) = budget_target {
            session.plan.club_transfer_budgets.insert(uid, budgets);
        } else {
            session.plan.club_transfer_budgets.remove(&uid);
        }
    }
    Ok(pending(&session))
}

#[tauri::command]
fn stage_club_infrastructure(
    uid: i32,
    training: Option<u8>,
    youth: Option<u8>,
    reputation: Option<i16>,
    state: State<Shared>,
) -> Result<Pending> {
    let mut session = state.lock().map_err(|e| e.to_string())?;
    let info = session
        .loaded
        .as_ref()
        .and_then(|d| d.club_info.get(&uid))
        .ok_or("Infraestrutura indisponível")?;
    let previous_reputation = session.plan.club_reputation.get(&uid).copied();
    let next_reputation = match reputation {
        Some(value) => {
            let original = info.reputation.ok_or("Equipe principal indisponível")?;
            if value != original && !(0..=10_000).contains(&value) {
                return Err("Informe uma reputação entre 0 e 10000".into());
            }
            (value != original).then_some(value)
        }
        None => previous_reputation,
    };
    let targets = [
        (training, info.training_raw, &session.plan.club_training),
        (youth, info.youth_setup_raw, &session.plan.club_youth_setup),
    ];
    let mut next = [None; 2];
    for (i, (value, original, staged)) in targets.into_iter().enumerate() {
        next[i] = match value {
            Some(value) if value == original => None,
            Some(value) if (1..=20).contains(&value) => Some(value),
            Some(_) => return Err("Informe um nível de infraestrutura entre 1 e 20".into()),
            None => staged.get(&uid).copied(),
        };
    }
    if next_reputation != previous_reputation
        || next
            != [
                session.plan.club_training.get(&uid).copied(),
                session.plan.club_youth_setup.get(&uid).copied(),
            ]
    {
        record(&mut session);
        let plan = &mut session.plan;
        if let Some(value) = next_reputation {
            plan.club_reputation.insert(uid, value);
        } else {
            plan.club_reputation.remove(&uid);
        }
        for (target, staged) in next
            .into_iter()
            .zip([&mut plan.club_training, &mut plan.club_youth_setup])
        {
            if let Some(value) = target {
                staged.insert(uid, value);
            } else {
                staged.remove(&uid);
            }
        }
    }
    Ok(pending(&session))
}
#[tauri::command]
fn search_players(query: String, page: usize, state: State<Shared>) -> Result<Page> {
    let data = loaded(&state)?;
    let q = search::key(query.trim());
    let matches = data
        .players
        .iter()
        .zip(&data.search_keys)
        .filter(|(_, key)| key.contains(&q))
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    Ok(Page {
        total: matches.len(),
        rows: matches
            .into_iter()
            .skip(page.saturating_mul(50))
            .take(50)
            .map(|p| Row {
                uid: p.uid,
                name: p.name.clone(),
                club: p.club.clone(),
                age: p.date_of_birth.age_on(&data.game_date),
                ca: p.current_ability,
                pa: p.potential_ability,
            })
            .collect(),
    })
}
#[tauri::command]
async fn player_detail(uid: i32, state: State<'_, Shared>) -> Result<Detail> {
    let data = loaded(&state)?;
    let p = data
        .players
        .iter()
        .find(|p| p.uid == uid)
        .ok_or("Jogador não encontrado")?;
    let editable = p.fields.keys().cloned().collect();
    let plan = state.lock().map_err(|e| e.to_string())?.plan.clone();
    let pending = plan
        .edits
        .iter()
        .filter(|e| e.uid == uid)
        .cloned()
        .collect::<Vec<_>>();
    let heal_pending = plan.heal_injuries.contains(&uid);
    let suspension_pending = plan.clear_suspensions.contains(&uid);
    let shared_personality = plan
        .staff_edits
        .iter()
        .any(|e| e.uid == uid && e.attribute.starts_with("personality."));
    let effective =
        if pending.is_empty() && !heal_pending && !suspension_pending && !shared_personality {
            p.clone()
        } else {
            let bytes = data.bytes.clone();
            // Related entities can be necessary for validity (e.g. swapping shirts).
            // Always project the complete transaction, then select this player.
            let edits = plan;
            tauri::async_runtime::spawn_blocking(move || -> Result<Player> {
                let (output, _) = transaction::apply(&bytes, &edits)?;
                let (db, base) = database::load(&output)?;
                db.players(base)?
                    .into_iter()
                    .find(|p| p.uid == uid)
                    .ok_or("Jogador não encontrado após revisão".into())
            })
            .await
            .map_err(|e| e.to_string())??
        };
    Ok(Detail {
        player: p.clone(),
        age: effective.date_of_birth.age_on(&data.game_date),
        effective,
        editable,
        contracts: data.contracts.get(&uid).cloned().unwrap_or_default(),
        pending,
        nations: data.nations.clone(),
        injuries: data.injuries.get(&uid).cloned().unwrap_or_default(),
        heal_pending,
        suspension_count: data.suspensions.get(&uid).copied().unwrap_or(0),
        suspension_pending,
    })
}
#[tauri::command]
async fn stage_edits(
    edits: Vec<Edit>,
    healing: Option<HealingUpdate>,
    suspension: Option<HealingUpdate>,
    state: State<'_, Shared>,
) -> Result<Pending> {
    let data = loaded(&state)?;
    let previous = state.lock().map_err(|e| e.to_string())?.plan.clone();
    let mut next = previous.clone();
    next.merge_fields(&data.players, edits)?;
    // Returning playing CA to its source removes redundant professional targets.
    next.merge_staff(&data.staff, Vec::new(), None)?;
    if let Some(update) = healing {
        if !data.players.iter().any(|p| p.uid == update.uid) {
            return Err("Jogador não encontrado".into());
        }
        if update.enabled {
            if data.injuries.get(&update.uid).is_none_or(|v| v.is_empty()) {
                return Err("Jogador sem registros de lesão".into());
            }
            next.heal_injuries.insert(update.uid);
        } else {
            next.heal_injuries.remove(&update.uid);
        }
    }
    if let Some(update) = suspension {
        let count = data
            .suspensions
            .get(&update.uid)
            .ok_or("Jogador não encontrado")?;
        if update.enabled {
            if *count == 0 {
                return Err("Jogador sem registros de suspensão".into());
            }
            next.clear_suspensions.insert(update.uid);
        } else {
            next.clear_suspensions.remove(&update.uid);
        }
    }
    if !next.is_empty() {
        let bytes = data.bytes.clone();
        let plan = next.clone();
        tauri::async_runtime::spawn_blocking(move || transaction::apply(&bytes, &plan).map(|_| ()))
            .await
            .map_err(|e| e.to_string())??;
    }
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if s.plan != previous || !s.loaded.as_ref().is_some_and(|p| Arc::ptr_eq(p, &data)) {
        return Err("A sessão mudou durante a validação; tente novamente".into());
    }
    if s.plan == next {
        return Ok(pending(&s));
    }
    record(&mut s);
    s.plan = next;
    Ok(pending(&s))
}
#[derive(serde::Deserialize)]
struct HealingUpdate {
    uid: i32,
    enabled: bool,
}
#[tauri::command]
fn pending_edits(state: State<Shared>) -> Result<Pending> {
    let session = state.lock().map_err(|e| e.to_string())?;
    Ok(pending(&session))
}
#[tauri::command]
fn history_action(action: String, state: State<Shared>) -> Result<Pending> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let current = s.plan.clone();
    let previous = match action.as_str() {
        "undo" => {
            let p = s.undo.pop();
            if p.is_some() {
                s.redo.push(current);
            }
            p
        }
        "redo" => {
            let p = s.redo.pop();
            if p.is_some() {
                s.undo.push(current);
            }
            p
        }
        "clear" => {
            record(&mut s);
            Some(Transaction::default())
        }
        _ => return Err("Ação inválida".into()),
    };
    if let Some(plan) = previous {
        s.plan = plan;
    }
    Ok(pending(&s))
}
#[tauri::command]
async fn preview_edits(state: State<'_, Shared>) -> Result<transaction::Report> {
    let data = loaded(&state)?;
    let edits = state.lock().map_err(|e| e.to_string())?.plan.clone();
    tauri::async_runtime::spawn_blocking(move || {
        transaction::apply(&data.bytes, &edits).map(|(_, r)| r)
    })
    .await
    .map_err(|e| e.to_string())?
}
#[derive(Serialize)]
struct Saved {
    path: String,
    report: transaction::Report,
}
#[tauri::command]
async fn save_copy(app: tauri::AppHandle, state: State<'_, Shared>) -> Result<Option<Saved>> {
    let data = loaded(&state)?;
    let edits = state.lock().map_err(|e| e.to_string())?.plan.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<Option<Saved>> {
        let Some(file) = app
            .dialog()
            .file()
            .add_filter("Save CM0304", &["cm4"])
            .set_file_name("save_editado.cm4")
            .blocking_save_file()
        else {
            return Ok(None);
        };
        let path = file.into_path().map_err(|e| e.to_string())?;
        let report = transaction::save_snapshot(
            &data.source,
            &path,
            &edits,
            &data.bytes,
            &AtomicBool::new(false),
        )?;
        Ok(Some(Saved {
            path: path.display().to_string(),
            report,
        }))
    })
    .await
    .map_err(|e| e.to_string())?
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(Session::default()))
        .invoke_handler(tauri::generate_handler![
            open_save,
            search_players,
            search_clubs,
            club_detail,
            search_staff,
            staff_detail,
            player_detail,
            stage_edits,
            stage_staff_edits,
            stage_club_balance,
            stage_club_infrastructure,
            pending_edits,
            history_action,
            preview_edits,
            save_copy
        ])
        .setup(|app| {
            let _ = app.get_webview_window("main");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Falha ao iniciar o editor");
}

#[cfg(test)]
mod club_expiry_tests {
    use super::*;

    #[test]
    fn projects_only_the_selected_contract_across_a_leap_year_boundary() {
        let today = Date {
            year: 2003,
            day: 364,
        };
        let record = |index, year, day| ClubContractDate {
            uid: 42,
            name: "Jogador".into(),
            expires: Date { year, day },
            index,
            loan: index == 1,
        };
        let records = [
            record(0, 2004, 90),
            record(1, 2004, 89),
            record(2, 1900, 1),
            record(3, 2003, 363),
            record(4, 2003, 364),
        ];
        let original = project_club_expiries(&records, &today, &[]);
        assert_eq!(original.iter().map(|e| e.days).collect::<Vec<_>>(), [0, 90]);
        let edit = Edit {
            uid: 42,
            attribute: "contract.0.expires.day".into(),
            value: 0,
            policy: None,
        };
        let projected = project_club_expiries(&records, &today, &[edit]);
        assert_eq!(
            projected
                .iter()
                .map(|e| (e.days, e.loan))
                .collect::<Vec<_>>(),
            [(0, false), (1, false), (90, true)]
        );
    }
}
