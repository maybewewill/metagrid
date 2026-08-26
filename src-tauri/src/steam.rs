use std::path::PathBuf;

pub const DEFAULT_STEAM_PATH: &str = r"C:\Program Files (x86)\Steam";
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
    /// Detect the Steam installation root.
    ///
    /// On Windows, reads `HKCU\Software\Valve\Steam\SteamPath` from the
    /// registry. If the key is missing, or on non-Windows targets, falls
    /// back to the default Steam install path.
    pub fn detect() -> Option<SteamLocator> {
        #[cfg(windows)]
        {
            if let Some(root) = Self::detect_from_registry() {
                return Some(SteamLocator { root });
            }
        }

        Some(SteamLocator {
            root: PathBuf::from(DEFAULT_STEAM_PATH),
        })
    }

    #[cfg(windows)]
    fn detect_from_registry() -> Option<PathBuf> {
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let steam_key = hkcu.open_subkey(r"Software\Valve\Steam").ok()?;
        let steam_path: String = steam_key.get_value("SteamPath").ok()?;
        Some(PathBuf::from(steam_path))
    }

    #[cfg(test)]
    pub fn with_root(root: PathBuf) -> Self {
        SteamLocator { root }
    }

    /// Scan `<root>/userdata/<id>/` for accounts that have a Dota 2
    /// (app id 570) grid config directory, returning one `Account` per
    /// matching id.
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
                let cfg_dir = entry.path().join(DOTA2_APP_ID).join("remote").join("cfg");
                if cfg_dir.is_dir() {
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
        // acc A has dota; acc B does not
        std::fs::create_dir_all(root.join("userdata/111/570/remote/cfg")).unwrap();
        std::fs::create_dir_all(root.join("userdata/222/730/remote")).unwrap();
        let loc = SteamLocator::with_root(root);
        let accs = loc.accounts();
        assert_eq!(accs.len(), 1);
        assert_eq!(accs[0].id, "111");
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
        // detect() must never panic and must always yield a locator, even
        // when the registry key is absent (or on non-windows targets).
        let loc = SteamLocator::detect();
        assert!(loc.is_some());
    }
}
