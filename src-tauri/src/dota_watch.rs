use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

pub struct DotaWatch {
    was_running: bool,
}

impl DotaWatch {
    pub fn new() -> Self {
        DotaWatch { was_running: false }
    }

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

pub fn is_dota_running(sys: &sysinfo::System) -> bool {
    sys.processes()
        .values()
        .any(|p| p.name().to_string_lossy().to_lowercase().contains("dota2"))
}

pub fn spawn(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut watch = DotaWatch::new();
        let mut sys = sysinfo::System::new();
        loop {
            sys.refresh_processes(sysinfo::ProcessesToUpdate::All);
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
        assert!(w.observe(true));
        assert!(!w.observe(true));
        assert!(!w.observe(false));
        assert!(w.observe(true));
    }
}
