use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::model::SortMetric;

fn default_top_n() -> usize {
    10
}

fn default_sort() -> SortMetric {
    SortMetric::Pickrate
}

fn default_interval_hours() -> f64 {
    6.0
}

fn default_autostart() -> bool {
    true
}

fn default_layout_columns() -> bool {
    false
}

fn default_lang() -> String {
    "en".into()
}

fn default_onboarded() -> bool {
    false
}

fn default_role_labels() -> String {
    "named".into()
}

fn default_grid_mode() -> String {
    "separate".into()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    #[serde(default = "default_top_n")]
    pub top_n: usize,
    #[serde(default = "default_sort")]
    pub sort: SortMetric,
    #[serde(default = "default_interval_hours")]
    pub interval_hours: f64,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default = "default_autostart")]
    pub autostart: bool,
    #[serde(default = "default_layout_columns")]
    pub layout_columns: bool,
    #[serde(default = "default_lang")]
    pub lang: String,
    #[serde(default = "default_onboarded")]
    pub onboarded: bool,
    #[serde(default = "default_role_labels")]
    pub role_labels: String,
    #[serde(default = "default_grid_mode")]
    pub grid_mode: String,
    #[serde(default)]
    pub merge_target: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            top_n: default_top_n(),
            sort: default_sort(),
            interval_hours: default_interval_hours(),
            account_id: None,
            autostart: default_autostart(),
            layout_columns: default_layout_columns(),
            lang: default_lang(),
            onboarded: default_onboarded(),
            role_labels: default_role_labels(),
            grid_mode: default_grid_mode(),
            merge_target: None,
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
            interval_hours: 3.0,
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
    #[test]
    fn partial_json_populates_missing_fields_with_defaults() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("settings.json"), br#"{"top_n": 5}"#).unwrap();
        let s = load_from(tmp.path());
        assert_eq!(s.top_n, 5);
        assert_eq!(s.interval_hours, 6.0);
        assert_eq!(s.role_labels, "named");
        assert!(s.autostart);
    }
}
