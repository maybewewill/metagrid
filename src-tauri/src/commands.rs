use tauri::{AppHandle, State};
use tauri_plugin_autostart::ManagerExt;

use crate::model::MetaSnapshot;
use crate::scheduler;
use crate::services::{account_dtos, AccountDto};
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
pub async fn refresh_now(app: AppHandle) -> Result<MetaSnapshot, String> {
    scheduler::run_refresh(&app).await
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
