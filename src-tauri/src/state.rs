use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::model::{MetaSnapshot, Tournament};
use crate::settings::Settings;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "detail")]
pub enum Status {
    Idle,
    Refreshing,
    Ok,
    Stale,
    Error(String),
}

pub struct AppState {
    snapshot: Mutex<Option<MetaSnapshot>>,
    status: Mutex<Status>,
    settings: Mutex<Settings>,
    tournaments: Mutex<Vec<Tournament>>,
    data_dir: PathBuf,
}

impl AppState {
    pub fn new(settings: Settings, data_dir: PathBuf) -> Self {
        let snapshot_file = data_dir.join("snapshot.json");
        let (snap, status) = if let Ok(content) = std::fs::read_to_string(&snapshot_file) {
            if let Ok(s) = serde_json::from_str::<MetaSnapshot>(&content) {
                (Some(s), Status::Ok)
            } else {
                (None, Status::Idle)
            }
        } else {
            (None, Status::Idle)
        };

        let tournaments = {
            let raw = include_str!("../resources/tournaments.json");
            serde_json::from_str::<Vec<Tournament>>(raw).unwrap_or_default()
        };

        AppState {
            snapshot: Mutex::new(snap),
            status: Mutex::new(status),
            settings: Mutex::new(settings),
            tournaments: Mutex::new(tournaments),
            data_dir,
        }
    }

    pub fn set_status(&self, s: Status) {
        *self.status.lock().unwrap() = s;
    }

    pub fn set_snapshot(&self, s: MetaSnapshot) {
        let file = self.data_dir.join("snapshot.json");
        if let Ok(content) = serde_json::to_string_pretty(&s) {
            let _ = std::fs::write(file, content);
        }
        *self.snapshot.lock().unwrap() = Some(s);
    }

    pub fn get_status(&self) -> Status {
        self.status.lock().unwrap().clone()
    }

    pub fn get_snapshot(&self) -> Option<MetaSnapshot> {
        self.snapshot.lock().unwrap().clone()
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    pub fn set_settings(&self, s: Settings) {
        *self.settings.lock().unwrap() = s;
    }

    pub fn get_tournaments(&self) -> Vec<Tournament> {
        self.tournaments.lock().unwrap().clone()
    }

    pub fn set_tournaments(&self, t: Vec<Tournament>) {
        *self.tournaments.lock().unwrap() = t;
    }
}

pub type Shared = Arc<AppState>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_and_snapshot_update() {
        let tmp = std::env::temp_dir().join(format!(
            "metagrid_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).ok();
        let st = AppState::new(Settings::default(), tmp.clone());
        assert_eq!(st.get_status(), Status::Idle);
        st.set_status(Status::Refreshing);
        assert_eq!(st.get_status(), Status::Refreshing);
        let _ = std::fs::remove_dir_all(tmp);
    }
}
