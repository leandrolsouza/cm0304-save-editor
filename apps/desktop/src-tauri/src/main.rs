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
    edits: Vec<Edit>,
    heal_injuries: Vec<i32>,
    clear_suspensions: Vec<i32>,
    undo: bool,
    redo: bool,
}
fn pending(s: &Session) -> Pending {
    Pending {
        edits: s.plan.edits.clone(),
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
                staff: staff.len(),
                sha256: hash(&bytes),
            };
            Ok(Some((
                Arc::new(Loaded {
                    source,
                    bytes: Arc::new(bytes),
                    players,
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
    if pending_player_fields > 0 {
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
    Ok(StaffDetail {
        profile,
        age,
        pending_player_fields,
    })
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
    let effective = if pending.is_empty() && !heal_pending && !suspension_pending {
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
            search_staff,
            staff_detail,
            player_detail,
            stage_edits,
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
