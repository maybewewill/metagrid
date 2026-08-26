use tauri::{AppHandle, Emitter, State};
use tauri_plugin_autostart::ManagerExt;

use crate::events;
use crate::grid::GridOptions;
use crate::model::MetaSnapshot;
use crate::pipeline;
use crate::services::{account_dtos, AccountDto, Services};
use crate::settings::{data_dir, save_to, Settings};
use crate::state::{Shared, Status};
use crate::steam::SteamLocator;

#[tauri::command]
pub fn get_snapshot(state: State<Shared>) -> Option<MetaSnapshot> {
    state.get_snapshot()
}

#[tauri::command]
pub fn get_status(state: State<Shared>) -> Status {
    state.get_status()
}

#[tauri::command]
pub fn get_settings(state: State<Shared>) -> Settings {
    state.get_settings()
}

#[tauri::command]
pub fn save_settings(new: Settings, state: State<Shared>) -> Result<(), String> {
    save_to(&data_dir(), &new).map_err(|e| e.to_string())?;
    state.set_settings(new);
    Ok(())
}

#[tauri::command]
pub fn list_accounts() -> Result<Vec<AccountDto>, String> {
    match SteamLocator::detect() {
        Some(loc) => Ok(account_dtos(&loc)),
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
pub async fn refresh_now(
    app: AppHandle,
    services: State<'_, Services>,
    state: State<'_, Shared>,
) -> Result<MetaSnapshot, String> {
    state.set_status(Status::Refreshing);
    let _ = app.emit(events::REFRESH_STARTED, ());

    let settings = state.get_settings();
    let opts = GridOptions {
        sort: settings.sort,
        layout_columns: settings.layout_columns,
    };

    let Some(steam) = SteamLocator::detect() else {
        let msg = "could not detect a Steam installation".to_string();
        state.set_status(Status::Error(msg.clone()));
        let _ = app.emit(events::REFRESH_ERROR, msg.clone());
        return Err(msg);
    };

    match pipeline::refresh_all(
        &*services.provider,
        &services.map,
        &steam,
        &opts,
        settings.top_n,
        settings.account_id.as_deref(),
    )
    .await
    {
        Ok(snap) => {
            state.set_snapshot(snap.clone());
            state.set_status(Status::Ok);
            let _ = app.emit(events::REFRESH_DONE, snap.clone());
            Ok(snap)
        }
        Err(err) => {
            let msg = err.to_string();
            state.set_status(Status::Error(msg.clone()));
            let _ = app.emit(events::REFRESH_ERROR, msg.clone());
            Err(msg)
        }
    }
}

#[tauri::command]
pub fn get_autostart(app: AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool, state: State<Shared>) -> Result<(), String> {
    if enabled {
        app.autolaunch().enable().map_err(|e| e.to_string())?;
    } else {
        app.autolaunch().disable().map_err(|e| e.to_string())?;
    }

    let mut settings = state.get_settings();
    settings.autostart = enabled;
    save_to(&data_dir(), &settings).map_err(|e| e.to_string())?;
    state.set_settings(settings);

    Ok(())
}
