//! System tray icon + menu. Left-click (or the "Open MetaGrid" menu item)
//! shows/focuses the main window; "Refresh now" wakes the background
//! scheduler via [`crate::scheduler::trigger`]; "Quit" exits the app.
//!
//! [`set_state`] (Task 3.6) reflects the current [`crate::state::Status`] on
//! the tray icon; called from `scheduler::run_refresh` at each status
//! transition.

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use tauri_plugin_notification::NotificationExt;

pub fn build(app: &tauri::App) -> tauri::Result<()> {
    let refresh = MenuItemBuilder::with_id("refresh", "Fetch & Patch").build(app)?;
    let open = MenuItemBuilder::with_id("open", "Open MetaGrid").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Exit").build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&refresh, &open, &quit])
        .build()?;
    let _tray = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "refresh" => {
                let _ = app
                    .notification()
                    .builder()
                    .title("MetaGrid")
                    .body("Fetching & Patching meta from Dota2ProTracker...")
                    .show();
                let _ = crate::scheduler::trigger(app);
            }
            "open" => show_main(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}

fn show_main(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

/// Reflects `status` on the tray icon. Distinct icon art (tray-idle /
/// tray-busy / tray-error) is not available yet, so every status currently
/// maps to the default window icon — the match stays explicit so wiring in
/// distinct art later is a one-line change per arm.
pub fn set_state(app: &tauri::AppHandle, status: &crate::state::Status) {
    let Some(tray) = app.tray_by_id("main") else {
        return;
    };
    let icon = match status {
        // TODO: distinct tray-idle/busy/error.png art
        crate::state::Status::Idle | crate::state::Status::Ok | crate::state::Status::Stale => {
            app.default_window_icon().cloned()
        }
        // TODO: distinct tray-idle/busy/error.png art
        crate::state::Status::Refreshing => app.default_window_icon().cloned(),
        // TODO: distinct tray-idle/busy/error.png art
        crate::state::Status::Error(_) => app.default_window_icon().cloned(),
    };
    let _ = tray.set_icon(icon);
}
