#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cm0304_core::{
    container::hash,
    database::{self, Date, Player},
    editor::{self, Edit},
    search,
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
}
#[derive(Default)]
struct Session {
    loaded: Option<Arc<Loaded>>,
    edits: BTreeMap<(i32, String), Edit>,
    undo: Vec<Vec<Edit>>,
    redo: Vec<Vec<Edit>>,
}
type Shared = Mutex<Session>;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Summary {
    name: String,
    path: String,
    players: usize,
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
struct Detail {
    player: Player,
    effective: Player,
    age: Option<u16>,
    editable: Vec<String>,
    pending: Vec<Edit>,
    nations: Vec<String>,
}
#[derive(Serialize)]
struct Pending {
    edits: Vec<Edit>,
    undo: bool,
    redo: bool,
}
fn pending(s: &Session) -> Pending {
    Pending {
        edits: s.edits.values().cloned().collect(),
        undo: !s.undo.is_empty(),
        redo: !s.redo.is_empty(),
    }
}
fn record(s: &mut Session) {
    let edits = s.edits.values().cloned().collect();
    s.undo.push(edits);
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
            let nations = db.nations.iter().map(|n| n.name.clone()).collect();
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
    let pending = state
        .lock()
        .map_err(|e| e.to_string())?
        .edits
        .values()
        .filter(|e| e.uid == uid)
        .cloned()
        .collect::<Vec<_>>();
    let effective = if pending.is_empty() {
        p.clone()
    } else {
        let bytes = data.bytes.clone();
        let edits = pending.clone();
        tauri::async_runtime::spawn_blocking(move || -> Result<Player> {
            let (output, _) = editor::apply(&bytes, &edits)?;
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
        pending,
        nations: data.nations.clone(),
    })
}
#[tauri::command]
async fn stage_edits(edits: Vec<Edit>, state: State<'_, Shared>) -> Result<Pending> {
    let data = loaded(&state)?;
    let previous = state.lock().map_err(|e| e.to_string())?.edits.clone();
    let next = editor::merge_plan(&data.players, &previous, edits)?;
    if !next.is_empty() {
        let bytes = data.bytes.clone();
        let plan = next.values().cloned().collect::<Vec<_>>();
        tauri::async_runtime::spawn_blocking(move || editor::apply(&bytes, &plan).map(|_| ()))
            .await
            .map_err(|e| e.to_string())??;
    }
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if s.edits != previous || !s.loaded.as_ref().is_some_and(|p| Arc::ptr_eq(p, &data)) {
        return Err("A sessão mudou durante a validação; tente novamente".into());
    }
    if s.edits == next {
        return Ok(pending(&s));
    }
    record(&mut s);
    s.edits = next;
    Ok(pending(&s))
}
#[tauri::command]
fn pending_edits(state: State<Shared>) -> Result<Pending> {
    let session = state.lock().map_err(|e| e.to_string())?;
    Ok(pending(&session))
}
#[tauri::command]
fn history_action(action: String, state: State<Shared>) -> Result<Pending> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let current = s.edits.values().cloned().collect();
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
            Some(Vec::new())
        }
        _ => return Err("Ação inválida".into()),
    };
    if let Some(edits) = previous {
        s.edits = edits
            .into_iter()
            .map(|e| ((e.uid, e.attribute.clone()), e))
            .collect();
    }
    Ok(pending(&s))
}
#[tauri::command]
async fn preview_edits(state: State<'_, Shared>) -> Result<editor::Report> {
    let data = loaded(&state)?;
    let edits = state
        .lock()
        .map_err(|e| e.to_string())?
        .edits
        .values()
        .cloned()
        .collect::<Vec<_>>();
    tauri::async_runtime::spawn_blocking(move || editor::apply(&data.bytes, &edits).map(|(_, r)| r))
        .await
        .map_err(|e| e.to_string())?
}
#[derive(Serialize)]
struct Saved {
    path: String,
    report: editor::Report,
}
#[tauri::command]
async fn save_copy(app: tauri::AppHandle, state: State<'_, Shared>) -> Result<Option<Saved>> {
    let data = loaded(&state)?;
    let edits = state
        .lock()
        .map_err(|e| e.to_string())?
        .edits
        .values()
        .cloned()
        .collect::<Vec<_>>();
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
        let report = editor::save_snapshot(
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
