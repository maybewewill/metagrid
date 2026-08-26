use tauri::{AppHandle, State};
use tauri_plugin_autostart::ManagerExt;

use crate::model::MetaSnapshot;
use crate::scheduler;
use crate::services::{account_dtos, AccountDto, DataDir};
use crate::settings::{save_to, Settings};
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
pub fn save_settings(
    new: Settings,
    state: State<Shared>,
    data: State<DataDir>,
) -> Result<(), String> {
    save_to(&data.0, &new).map_err(|e| e.to_string())?;
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
pub async fn fetch_only(
    state: State<'_, Shared>,
    services: State<'_, crate::services::Services>,
) -> Result<MetaSnapshot, String> {
    let settings = state.get_settings();
    let snap = services
        .provider
        .fetch(&services.map, settings.top_n)
        .await
        .map_err(|e| e.to_string())?;
    state.set_snapshot(snap.clone());
    Ok(snap)
}

#[tauri::command]
pub fn get_autostart(app: AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_autostart(
    app: AppHandle,
    enabled: bool,
    state: State<Shared>,
    data: State<DataDir>,
) -> Result<(), String> {
    let mgr = app.autolaunch();
    if enabled {
        mgr.enable().map_err(|e| e.to_string())?;
    } else {
        mgr.disable().map_err(|e| e.to_string())?;
    }

    let mut settings = state.get_settings();
    settings.autostart = enabled;
    save_to(&data.0, &settings).map_err(|e| e.to_string())?;
    state.set_settings(settings);

    Ok(())
}

#[tauri::command]
pub fn get_portrait_dir(data: State<DataDir>) -> String {
    data.0.join("portraits").to_string_lossy().into_owned()
}

#[tauri::command]
pub async fn check_update() -> Result<crate::updater::UpdateInfo, String> {
    crate::updater::check_for_updates().await
}

#[tauri::command]
pub fn list_grid_configs(state: State<Shared>) -> Vec<String> {
    let settings = state.get_settings();
    let Some(loc) = SteamLocator::detect() else {
        return Vec::new();
    };
    let accounts = loc.accounts();
    let account = match settings.account_id.as_deref() {
        Some(id) => accounts.iter().find(|a| a.id == id),
        None => accounts.first(),
    };
    let Some(account) = account else {
        return Vec::new();
    };
    let Ok(contents) = std::fs::read_to_string(&account.grid_path) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&contents) else {
        return Vec::new();
    };
    value
        .get("configs")
        .and_then(|c| c.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|c| c.get("config_name").and_then(|n| n.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default()
}
