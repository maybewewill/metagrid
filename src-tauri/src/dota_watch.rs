use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DotaEvent {
    None,
    Started,
    Closed,
}

pub struct DotaWatch {
    was_running: bool,
}

impl DotaWatch {
    pub fn new() -> Self {
        DotaWatch { was_running: false }
    }

    pub fn observe(&mut self, running: bool) -> DotaEvent {
        let event = match (self.was_running, running) {
            (false, true) => DotaEvent::Started,
            (true, false) => DotaEvent::Closed,
            _ => DotaEvent::None,
        };
        self.was_running = running;
        event
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
            match watch.observe(running) {
                DotaEvent::Started => {
                    let _ = crate::scheduler::trigger(&app);
                    let _ = app
                        .notification()
                        .builder()
                        .title("MetaGrid")
                        .body("Meta refreshed for current game")
                        .show();
                }
                DotaEvent::Closed => {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    let _ = crate::scheduler::trigger(&app);
                    let _ = app
                        .notification()
                        .builder()
                        .title("MetaGrid")
                        .body("Dota 2 closed — Meta refreshed for your next session")
                        .show();
                }
                DotaEvent::None => {}
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fires_started_and_closed_events_correctly() {
        let mut w = DotaWatch::new();
        assert_eq!(w.observe(false), DotaEvent::None);
        assert_eq!(w.observe(true), DotaEvent::Started);
        assert_eq!(w.observe(true), DotaEvent::None);
        assert_eq!(w.observe(false), DotaEvent::Closed);
        assert_eq!(w.observe(false), DotaEvent::None);
        assert_eq!(w.observe(true), DotaEvent::Started);
    }
}
