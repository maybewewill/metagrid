use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::model::SortMetric;

fn default_role_labels() -> String {
    "named".into()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub top_n: usize,
    pub sort: SortMetric,
    pub interval_hours: u64,
    pub account_id: Option<String>,
    pub autostart: bool,
    pub layout_columns: bool,
    pub lang: String,
    pub onboarded: bool,
    #[serde(default = "default_role_labels")]
    pub role_labels: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            top_n: 10,
            sort: SortMetric::Pickrate,
            interval_hours: 6,
            account_id: None,
            autostart: true,
            layout_columns: false,
            lang: "en".into(),
            onboarded: false,
            role_labels: default_role_labels(),
        }
    }
}

fn settings_path(dir: &Path) -> PathBuf {
    dir.join("settings.json")
}

pub fn load_from(dir: &Path) -> Settings {
    let path = settings_path(dir);
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Settings::default();
    };
    serde_json::from_str(&contents).unwrap_or_default()
}

pub fn save_to(dir: &Path, s: &Settings) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let json = serde_json::to_string_pretty(s)?;
    std::fs::write(settings_path(dir), json)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_gives_default_then_roundtrips() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(load_from(tmp.path()), Settings::default());
        let s = Settings {
            top_n: 7,
            interval_hours: 3,
            ..Default::default()
        };
        save_to(tmp.path(), &s).unwrap();
        assert_eq!(load_from(tmp.path()), s);
    }
    #[test]
    fn corrupt_file_falls_back_to_default() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("settings.json"), b"{ not json").unwrap();
        assert_eq!(load_from(tmp.path()), Settings::default());
    }
}
