pub mod d2pt;

use crate::grid::GridConfig;
use crate::hero_map::HeroMap;
use crate::model::MetaSnapshot;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Could not connect to Dota2ProTracker (request blocked or timeout)")]
    Blocked,
    #[error("Failed to parse Dota2ProTracker meta: {0}")]
    Parse(String),
}

#[async_trait::async_trait]
pub trait MetaProvider: Send + Sync {
    async fn fetch(&self, map: &HeroMap, meta_mode: &str) -> Result<(MetaSnapshot, Vec<GridConfig>), ProviderError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hero_map::HeroMap;
    struct Fake;
    #[async_trait::async_trait]
    impl MetaProvider for Fake {
        async fn fetch(&self, _m: &HeroMap, _mode: &str) -> Result<(MetaSnapshot, Vec<GridConfig>), ProviderError> {
            Err(ProviderError::Blocked)
        }
    }
    #[tokio::test]
    async fn trait_is_object_safe() {
        let p: Box<dyn MetaProvider> = Box::new(Fake);
        assert!(matches!(
            p.fetch(&HeroMap::bundled(), "matches").await,
            Err(ProviderError::Blocked)
        ));
    }
}
