use crate::hero_map::HeroMap;
use crate::model::MetaSnapshot;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("http error: {0}")]
    Http(String),
    #[error("request blocked")]
    Blocked,
    #[error("parse error: {0}")]
    Parse(String),
    #[error("no data returned")]
    Empty,
}

#[async_trait::async_trait]
pub trait MetaProvider: Send + Sync {
    fn id(&self) -> &'static str;
    async fn fetch(&self, map: &HeroMap, top_n: usize) -> Result<MetaSnapshot, ProviderError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{hero_map::HeroMap, model::*};
    struct Fake;
    #[async_trait::async_trait]
    impl MetaProvider for Fake {
        fn id(&self)->&'static str { "fake" }
        async fn fetch(&self, _m:&HeroMap, _n:usize) -> Result<MetaSnapshot, ProviderError> {
            Err(ProviderError::Empty)
        }
    }
    #[tokio::test]
    async fn trait_is_object_safe() {
        let p: Box<dyn MetaProvider> = Box::new(Fake);
        assert_eq!(p.id(), "fake");
        assert!(matches!(p.fetch(&HeroMap::bundled(), 10).await, Err(ProviderError::Empty)));
    }
}
