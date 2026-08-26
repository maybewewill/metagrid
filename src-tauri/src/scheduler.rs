//! Background refresh scheduler: kicks a refresh once at startup, then on a
//! settings-driven interval, and can be woken early via [`trigger`] (e.g. by
//! the tray or the Dota watcher). [`run_refresh`] is the single source of
//! truth for "what a refresh does" — the `refresh_now` IPC command delegates
//! to it so there is no duplicated logic between the manual and scheduled
//! refresh paths.

use std::sync::Arc;
use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::Notify;

use crate::events;
use crate::model::MetaSnapshot;
use crate::pipeline;
use crate::services::Services;
use crate::state::{Shared, Status};
use crate::steam::SteamLocator;

/// Runs a full refresh cycle: sets status, emits lifecycle events, resolves
/// settings/Steam/provider off the app handle's managed state, and updates
/// the shared snapshot/status on completion. Used by both the `refresh_now`
/// IPC command and the background scheduler.
pub async fn run_refresh(app: &AppHandle) -> Result<MetaSnapshot, String> {
    let state = app.state::<Shared>();
    let services = app.state::<Services>();

    state.set_status(Status::Refreshing);
    crate::tray::set_state(app, &Status::Refreshing);
    let _ = app.emit(events::REFRESH_STARTED, ());

    let settings = state.get_settings();

    let Some(steam) = SteamLocator::detect() else {
        let msg = "could not detect a Steam installation".to_string();
        state.set_status(Status::Error(msg.clone()));
        crate::tray::set_state(app, &Status::Error(msg.clone()));
        let _ = app.emit(events::REFRESH_ERROR, msg.clone());
        return Err(msg);
    };

    match pipeline::refresh_all(
        &*services.provider,
        &services.map,
        &steam,
        settings.top_n,
        settings.account_id.as_deref(),
    )
    .await
    {
        Ok(snap) => {
            state.set_snapshot(snap.clone());
            state.set_status(Status::Ok);
            crate::tray::set_state(app, &Status::Ok);
            let _ = app.emit(events::REFRESH_DONE, snap.clone());
            let _ = app
                .notification()
                .builder()
                .title("MetaGrid")
                .body(format!("Meta updated for Patch {}", snap.patch))
                .show();
            Ok(snap)
        }
        Err(err) => {
            let msg = err.to_string();
            state.set_status(Status::Error(msg.clone()));
            crate::tray::set_state(app, &Status::Error(msg.clone()));
            let _ = app.emit(events::REFRESH_ERROR, msg.clone());
            let _ = app
                .notification()
                .builder()
                .title("MetaGrid: Update Failed")
                .body(format!("{}", msg))
                .show();
            Err(msg)
        }
    }
}

/// Returns `true` when `now` is less than `min` after `last` — i.e. the
/// trigger arrived too soon and should be ignored.
pub fn should_debounce(last: Instant, now: Instant, min: Duration) -> bool {
    now.duration_since(last) < min
}

/// Managed state wrapping the `Notify` used to wake the scheduler loop early.
pub struct Trigger(pub Arc<Notify>);

/// Wakes the background scheduler loop, causing it to (subject to debounce)
/// run a refresh immediately instead of waiting for the next interval tick.
pub fn trigger(app: &AppHandle) -> tauri::Result<()> {
    app.state::<Trigger>().0.notify_one();
    Ok(())
}

const MIN_INTERVAL_HOURS: u64 = 1;
const DEBOUNCE_FLOOR: Duration = Duration::from_secs(30);

/// Spawns the background scheduler task: one refresh at startup, then a loop
/// that wakes on either the settings-driven interval or an explicit
/// [`trigger`], applying a debounce floor so rapid triggers collapse into a
/// single refresh.
pub fn spawn(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut last = Instant::now();

        loop {
            let interval_hours = app
                .state::<Shared>()
                .get_settings()
                .interval_hours
                .max(MIN_INTERVAL_HOURS);
            let dur = Duration::from_secs(interval_hours * 3600);
            let notify = app.state::<Trigger>().0.clone();

            tokio::select! {
                _ = tokio::time::sleep(dur) => {}
                _ = notify.notified() => {}
            }

            let now = Instant::now();
            if should_debounce(last, now, DEBOUNCE_FLOOR) {
                continue;
            }
            last = now;
            let _ = run_refresh(&app).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    #[test]
    fn debounces_rapid_triggers() {
        let t0 = Instant::now();
        assert!(should_debounce(
            t0,
            t0 + Duration::from_secs(1),
            Duration::from_secs(30)
        ));
        assert!(!should_debounce(
            t0,
            t0 + Duration::from_secs(60),
            Duration::from_secs(30)
        ));
    }
}
