//! Polls the process list for `dota2.exe` and, on a false→true (launch)
//! edge, wakes the scheduler for an immediate refresh and shows a desktop
//! notification. Polling (rather than an OS-level process-start hook) keeps
//! this simple and cross-platform-ish; 5s resolution is plenty for "meta
//! refreshed before you finish loading into a game."

use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

/// Rising-edge detector for "is Dota 2 running". Only the false→true
/// transition should trigger a refresh; repeated `true` observations (the
/// game stays open) or the true→false transition (the game closes) must not.
pub struct DotaWatch {
    was_running: bool,
}

impl DotaWatch {
    pub fn new() -> Self {
        DotaWatch { was_running: false }
    }

    /// Records the latest observed running-state and returns `true` only on
    /// a false→true (launch) edge.
    pub fn observe(&mut self, running: bool) -> bool {
        let rising_edge = running && !self.was_running;
        self.was_running = running;
        rising_edge
    }
}

impl Default for DotaWatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns true if any running process's name contains "dota2" (matches
/// `dota2.exe` on Windows without hard-coding the extension).
pub fn is_dota_running(sys: &sysinfo::System) -> bool {
    sys.processes()
        .values()
        .any(|p| p.name().to_string_lossy().to_lowercase().contains("dota2"))
}

/// Spawns the background polling loop: every [`POLL_INTERVAL`], checks
/// whether Dota 2 just launched and, on that rising edge, wakes the
/// scheduler and shows a notification.
pub fn spawn(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut watch = DotaWatch::new();
        loop {
            let sys = sysinfo::System::new_all();
            let running = is_dota_running(&sys);
            if watch.observe(running) {
                let _ = crate::scheduler::trigger(&app);
                let _ = app
                    .notification()
                    .builder()
                    .title("MetaGrid")
                    .body("Meta refreshed for next game")
                    .show();
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fires_only_on_rising_edge() {
        let mut w = DotaWatch::new();
        assert!(!w.observe(false));
        assert!(w.observe(true)); // launched
        assert!(!w.observe(true)); // still running, no repeat
        assert!(!w.observe(false)); // closed
        assert!(w.observe(true)); // relaunched
    }
}
