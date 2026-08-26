use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::hero_map::HeroMap;
use crate::provider::d2pt::D2ptProvider;
use crate::provider::MetaProvider;
use crate::steam::SteamLocator;

/// Shared dependencies handed to Tauri commands via `.manage(...)`.
pub struct Services {
    pub provider: Arc<dyn MetaProvider>,
    pub map: HeroMap,
}

impl Services {
    pub fn new() -> Self {
        Services {
            provider: Arc::new(D2ptProvider::new()),
            map: HeroMap::bundled(),
        }
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}

/// The Tauri-resolved app data directory (`$APPDATA/com.metagrid.app`),
/// managed so commands and the scheduler can read/write settings and
/// portraits from the same place the `assetProtocol` scope points at.
pub struct DataDir(pub PathBuf);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountDto {
    pub id: String,
}

pub fn account_dtos(loc: &SteamLocator) -> Vec<AccountDto> {
    loc.accounts()
        .into_iter()
        .map(|a| AccountDto { id: a.id })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_accounts_to_dtos() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("userdata/111/570/remote/cfg")).unwrap();
        let loc = crate::steam::SteamLocator::with_root(tmp.path().to_path_buf());
        let dtos = account_dtos(&loc);
        assert_eq!(dtos, vec![AccountDto { id: "111".into() }]);
    }

    #[test]
    fn services_initializes_with_d2pt() {
        let s = Services::new();
        assert_eq!(s.map.slug_for(1), Some("antimage"));
    }
}
