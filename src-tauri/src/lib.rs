mod commands;
mod events;
mod grid;
mod grid_writer;
mod hero_map;
mod model;
mod pipeline;
mod portraits;
mod provider;
mod scheduler;
mod services;
mod settings;
mod state;
mod steam;
mod tray;

use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .manage::<state::Shared>(std::sync::Arc::new(state::AppState::new(
            settings::load_from(&settings::data_dir()),
        )))
        .manage(services::Services::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_snapshot,
            commands::get_status,
            commands::get_settings,
            commands::save_settings,
            commands::list_accounts,
            commands::refresh_now,
            commands::get_autostart,
            commands::set_autostart
        ])
        .setup(|app| {
            app.manage(scheduler::Trigger(std::sync::Arc::new(
                tokio::sync::Notify::new(),
            )));
            tray::build(app)?;

            let want = app.state::<state::Shared>().get_settings().autostart;
            let mgr = app.autolaunch();
            if want && !mgr.is_enabled().unwrap_or(false) {
                let _ = mgr.enable();
            }
            if !want && mgr.is_enabled().unwrap_or(false) {
                let _ = mgr.disable();
            }

            if let Some(w) = app.get_webview_window("main") {
                let w2 = w.clone();
                w.on_window_event(move |e| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                        api.prevent_close();
                        let _ = w2.hide();
                    }
                });
            }
            scheduler::spawn(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
