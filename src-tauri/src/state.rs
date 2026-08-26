use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::model::MetaSnapshot;
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
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        AppState {
            snapshot: Mutex::new(None),
            status: Mutex::new(Status::Idle),
            settings: Mutex::new(settings),
        }
    }

    pub fn set_status(&self, s: Status) {
        *self.status.lock().unwrap() = s;
    }

    pub fn set_snapshot(&self, s: MetaSnapshot) {
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
}

pub type Shared = Arc<AppState>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_and_snapshot_update() {
        let st = AppState::new(Settings::default());
        assert_eq!(st.get_status(), Status::Idle);
        st.set_status(Status::Refreshing);
        assert_eq!(st.get_status(), Status::Refreshing);
    }
}
