use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub const DEFAULT_STEAM_PATH: &str = r"C:\Program Files (x86)\Steam";

#[cfg(target_os = "macos")]
pub const DEFAULT_STEAM_PATH: &str = "Library/Application Support/Steam";

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
pub const DEFAULT_STEAM_PATH: &str = ".local/share/Steam";

const DOTA2_APP_ID: &str = "570";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: String,
    pub grid_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamLocator {
    root: PathBuf,
}

impl SteamLocator {
    pub fn detect() -> Option<SteamLocator> {
        #[cfg(target_os = "windows")]
        {
            if let Some(root) = Self::detect_from_registry() {
                return Some(SteamLocator { root });
            }
            Some(SteamLocator {
                root: PathBuf::from(DEFAULT_STEAM_PATH),
            })
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(root) = Self::detect_macos() {
                return Some(SteamLocator { root });
            }
            let home = std::env::var("HOME")
                .ok()
                .map(PathBuf::from)
                .or_else(|| directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."));
            Some(SteamLocator {
                root: home.join(DEFAULT_STEAM_PATH),
            })
        }

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            if let Some(root) = Self::detect_linux() {
                return Some(SteamLocator { root });
            }
            let home = std::env::var("HOME")
                .ok()
                .map(PathBuf::from)
                .or_else(|| directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."));
            Some(SteamLocator {
                root: home.join(DEFAULT_STEAM_PATH),
            })
        }
    }

    #[cfg(target_os = "windows")]
    fn detect_from_registry() -> Option<PathBuf> {
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let steam_key = hkcu.open_subkey(r"Software\Valve\Steam").ok()?;
        let steam_path: String = steam_key.get_value("SteamPath").ok()?;
        Some(PathBuf::from(steam_path))
    }

    #[cfg(target_os = "macos")]
    fn detect_macos() -> Option<PathBuf> {
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()))?;
        let steam = home.join(DEFAULT_STEAM_PATH);
        if steam.join("userdata").is_dir() {
            Some(steam)
        } else {
            None
        }
    }

    #[cfg(not(windows))]
    fn detect_linux() -> Option<PathBuf> {
        if let Ok(steam_dir) = std::env::var("STEAM_DIR").or_else(|_| std::env::var("STEAM_PATH")) {
            let p = PathBuf::from(steam_dir);
            if p.join("userdata").is_dir() {
                return Some(p);
            }
        }

        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()))?;

        for path in Self::find_linux_candidates(&home) {
            if path.join("userdata").is_dir() {
                return Some(path);
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn find_linux_candidates(home: &std::path::Path) -> Vec<PathBuf> {
        vec![
            home.join(".local/share/Steam"),
            home.join(".steam/steam"),
            home.join(".steam/root"),
            home.join(".steam/debian-installation"),
            home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
            home.join(".var/app/com.valvesoftware.Steam/.steam/steam"),
            home.join("snap/steam/common/.local/share/Steam"),
            home.join("snap/steam/common/.steam/steam"),
        ]
    }

    #[cfg(test)]
    pub fn with_root(root: PathBuf) -> Self {
        SteamLocator { root }
    }

    pub fn accounts(&self) -> Vec<Account> {
        let userdata = self.root.join("userdata");

        let entries = match std::fs::read_dir(&userdata) {
            Ok(entries) => entries,
            Err(_) => return Vec::new(),
        };

        let mut accounts: Vec<Account> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .filter_map(|entry| {
                let id = entry.file_name().to_string_lossy().into_owned();
                if id == "0" || id == "anonymous" {
                    return None;
                }
                let user_path = entry.path();
                let dota_dir = user_path.join(DOTA2_APP_ID);
                let cfg_dir = dota_dir.join("remote").join("cfg");
                if cfg_dir.is_dir() || dota_dir.is_dir() {
                    let grid_path = cfg_dir.join("hero_grid_config.json");
                    Some(Account { id, grid_path })
                } else {
                    None
                }
            })
            .collect();

        accounts.sort_by(|a, b| a.id.cmp(&b.id));
        accounts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_only_dota_accounts() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir_all(root.join("userdata/111/570/remote/cfg")).unwrap();
        std::fs::create_dir_all(root.join("userdata/222/730/remote")).unwrap();
        let loc = SteamLocator::with_root(root);
        let accs = loc.accounts();
        assert_eq!(accs.len(), 1);
        assert_eq!(accs[0].id, "111");
        assert!(accs[0].grid_path.ends_with("cfg/hero_grid_config.json"));
    }

    #[test]
    fn finds_dota_account_even_if_cfg_dir_not_yet_created() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir_all(root.join("userdata/333/570")).unwrap();
        let loc = SteamLocator::with_root(root);
        let accs = loc.accounts();
        assert_eq!(accs.len(), 1);
        assert_eq!(accs[0].id, "333");
        assert!(accs[0].grid_path.ends_with("cfg/hero_grid_config.json"));
    }

    #[test]
    fn missing_userdata_dir_returns_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let loc = SteamLocator::with_root(tmp.path().to_path_buf());
        assert!(loc.accounts().is_empty());
    }

    #[test]
    fn detect_falls_back_when_no_registry_entry() {
        let loc = SteamLocator::detect();
        assert!(loc.is_some());
    }

    #[test]
    fn linux_candidates_include_flatpak_and_native() {
        let home = PathBuf::from("/home/user");
        let candidates = SteamLocator::find_linux_candidates(&home);
        assert!(candidates.iter().any(|p| p.ends_with(".local/share/Steam")));
        assert!(candidates.iter().any(|p| p.to_string_lossy().contains("com.valvesoftware.Steam")));
        assert!(candidates.iter().any(|p| p.to_string_lossy().contains("snap/steam")));
    }
}
