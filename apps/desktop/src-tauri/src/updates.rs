use std::{sync::Mutex, time::Duration};
use tauri::{AppHandle, State};
use tauri_plugin_updater::{Update, UpdaterExt};

use crate::{OpenSaveTask, Result, SaveCopyTask, Shared};

#[derive(Default)]
pub(crate) struct DownloadedUpdate(Mutex<Option<(Update, Vec<u8>)>>);

#[tauri::command]
pub(crate) async fn download_update(
    app: AppHandle,
    downloaded: State<'_, DownloadedUpdate>,
) -> Result<bool> {
    if !app.config().plugins.0.contains_key("updater") {
        return Ok(false);
    }
    let updater = app
        .updater_builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;
    let Some(mut update) = updater.check().await.map_err(|error| error.to_string())? else {
        return Ok(false);
    };
    update.timeout = Some(Duration::from_secs(120));
    let bytes = update
        .download(|_, _| {}, || {})
        .await
        .map_err(|error| error.to_string())?;
    *downloaded.0.lock().map_err(|error| error.to_string())? = Some((update, bytes));
    Ok(true)
}

#[tauri::command]
pub(crate) fn install_update(
    session: State<'_, Shared>,
    opening: State<'_, OpenSaveTask>,
    saving: State<'_, SaveCopyTask>,
    downloaded: State<'_, DownloadedUpdate>,
) -> Result<()> {
    // Match open_save's lock order and keep a new open from starting during installation.
    let opening = opening.0.lock().map_err(|error| error.to_string())?;
    let saving = saving.0.lock().map_err(|error| error.to_string())?;
    let session = session.lock().map_err(|error| error.to_string())?;
    if opening.is_some() || saving.is_some() || session.loaded.is_some() {
        return Ok(());
    }
    let pending = downloaded
        .0
        .lock()
        .map_err(|error| error.to_string())?
        .take();
    if let Some((update, bytes)) = pending {
        update.install(bytes).map_err(|error| error.to_string())?;
    }
    Ok(())
}
