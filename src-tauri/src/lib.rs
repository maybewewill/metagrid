mod commands;
mod dota_watch;
mod events;
mod grid;
mod grid_writer;
mod hero_map;
mod model;
mod pipeline;
mod provider;
mod scheduler;
mod services;
mod settings;
mod state;
mod steam;
mod tray;
mod updater;

use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_notification::init());

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            commands::get_snapshot,
            commands::get_status,
            commands::get_settings,
            commands::save_settings,
            commands::list_accounts,
            commands::refresh_now,
            commands::fetch_only,
            commands::get_autostart,
            commands::set_autostart,
            commands::get_portrait_dir,
            commands::check_update,
            commands::list_grid_configs
        ])
        .setup(|app| {
            let data = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data).ok();

            let settings = settings::load_from(&data);
            let state: state::Shared = std::sync::Arc::new(state::AppState::new(settings, data.clone()));
            app.manage(state.clone());
            app.manage(services::Services::new());
            app.manage(services::DataDir(data));

            app.manage(scheduler::Trigger(std::sync::Arc::new(
                tokio::sync::Notify::new(),
            )));
            tray::build(app)?;

            let want = state.get_settings().autostart;
            let mgr = app.autolaunch();
            if want && !mgr.is_enabled().unwrap_or(false) {
                let _ = mgr.enable();
            }
            if !want && mgr.is_enabled().unwrap_or(false) {
                let _ = mgr.disable();
            }

            let is_minimized = std::env::args().any(|arg| arg == "--minimized");

            if let Some(w) = app.get_webview_window("main") {
                let w2 = w.clone();
                w.on_window_event(move |e| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                        api.prevent_close();
                        let _ = w2.hide();
                    }
                });
                if !is_minimized {
                    let _ = w.show();
                }
            }
            scheduler::spawn(app.handle().clone());
            dota_watch::spawn(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
