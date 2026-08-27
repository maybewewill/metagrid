pub mod d2pt;

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
    async fn fetch(&self, map: &HeroMap, top_n: usize, meta_source: &str, league_id: i64) -> Result<MetaSnapshot, ProviderError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hero_map::HeroMap;
    struct Fake;
    #[async_trait::async_trait]
    impl MetaProvider for Fake {
        async fn fetch(&self, _m: &HeroMap, _n: usize, _s: &str, _l: i64) -> Result<MetaSnapshot, ProviderError> {
            Err(ProviderError::Blocked)
        }
    }
    #[tokio::test]
    async fn trait_is_object_safe() {
        let p: Box<dyn MetaProvider> = Box::new(Fake);
        assert!(matches!(
            p.fetch(&HeroMap::bundled(), 10, "pubs", -1).await,
            Err(ProviderError::Blocked)
        ));
    }
}
