use crate::grid::extract_role_categories_for_merge;
use crate::grid_writer::{write_configs_to, write_merge_to, GridError};
use crate::hero_map::HeroMap;
use crate::model::MetaSnapshot;
use crate::provider::{MetaProvider, ProviderError};
use crate::steam::SteamLocator;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error(transparent)]
    Grid(#[from] GridError),
}

pub async fn refresh_all(
    provider: &dyn MetaProvider,
    map: &HeroMap,
    steam: &SteamLocator,
    account_filter: Option<&str>,
    grid_mode: &str,
    merge_target: Option<&str>,
    meta_mode: &str,
) -> Result<MetaSnapshot, PipelineError> {
    let (snap, d2pt_grids) = provider.fetch(map, meta_mode).await?;

    let merge = grid_mode == "merge" && merge_target.is_some();
    let meta_cats = if merge {
        extract_role_categories_for_merge(&d2pt_grids)
    } else {
        Vec::new()
    };

    for account in steam.accounts() {
        if let Some(filter_id) = account_filter {
            if account.id != filter_id {
                continue;
            }
        }
        if merge {
            write_merge_to(&account.grid_path, &meta_cats, merge_target.unwrap())?;
        } else {
            write_configs_to(&account.grid_path, &d2pt_grids)?;
        }
    }

    Ok(snap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{Category, GridConfig};
    use crate::model::{HeroMeta, Position, RoleMeta};
    use crate::provider::ProviderError;

    struct FakeProvider;

    #[async_trait::async_trait]
    impl MetaProvider for FakeProvider {
        async fn fetch(&self, _map: &HeroMap, _mode: &str) -> Result<(MetaSnapshot, Vec<GridConfig>), ProviderError> {
            let roles = Position::all()
                .iter()
                .enumerate()
                .map(|(i, &position)| RoleMeta {
                    position,
                    role_winrate: 0.5,
                    heroes: vec![
                        HeroMeta {
                            hero_id: (i as u32) * 10 + 1,
                            name: format!("Hero{}A", i),
                            slug: format!("hero{}a", i),
                            winrate: 0.55,
                            pickrate: 0.20,
                            matches: 100,
                            d2pt_rating: 3200,
                            is_top: true,
                        },
                        HeroMeta {
                            hero_id: (i as u32) * 10 + 2,
                            name: format!("Hero{}B", i),
                            slug: format!("hero{}b", i),
                            winrate: 0.45,
                            pickrate: 0.10,
                            matches: 50,
                            d2pt_rating: 2900,
                            is_top: false,
                        },
                    ],
                })
                .collect();

            let snap = MetaSnapshot {
                patch: "7.41e".into(),
                fetched_at: "2026-08-26T10:00:00Z".into(),
                source: "fake".into(),
                roles,
                configs: vec![],
            };

            let configs = vec![
                GridConfig {
                    config_name: "Dota2ProTracker 7.41e - All Roles".into(),
                    categories: vec![
                        Category {
                            category_name: "Carry".into(),
                            x_position: 0.0,
                            y_position: 0.0,
                            width: 455.0,
                            height: 75.0,
                            hero_ids: vec![1, 2],
                        },
                        Category {
                            category_name: "Mid".into(),
                            x_position: 0.0,
                            y_position: 95.0,
                            width: 455.0,
                            height: 75.0,
                            hero_ids: vec![3, 4],
                        },
                        Category {
                            category_name: "Offlane".into(),
                            x_position: 0.0,
                            y_position: 190.0,
                            width: 455.0,
                            height: 75.0,
                            hero_ids: vec![5, 6],
                        },
                        Category {
                            category_name: "Support".into(),
                            x_position: 0.0,
                            y_position: 285.0,
                            width: 455.0,
                            height: 75.0,
                            hero_ids: vec![7, 8],
                        },
                        Category {
                            category_name: "Hard Support".into(),
                            x_position: 0.0,
                            y_position: 380.0,
                            width: 455.0,
                            height: 75.0,
                            hero_ids: vec![9, 10],
                        },
                    ],
                },
                GridConfig {
                    config_name: "Dota2ProTracker 7.41e - Carry".into(),
                    categories: vec![
                        Category {
                            category_name: "Top Heroes Pos 1".into(),
                            x_position: 0.0,
                            y_position: 0.0,
                            width: 65.0,
                            height: 525.0,
                            hero_ids: vec![1, 2],
                        },
                    ],
                },
            ];

            Ok((snap, configs))
        }
    }

    #[tokio::test]
    async fn refresh_all_writes_to_each_account() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir_all(root.join("userdata/111/570/remote/cfg")).unwrap();
        std::fs::create_dir_all(root.join("userdata/222/570/remote/cfg")).unwrap();

        let steam = SteamLocator::with_root(root);

        let snap = refresh_all(&FakeProvider, &HeroMap::bundled(), &steam, None, "separate", None, "matches")
            .await
            .unwrap();

        assert_eq!(snap.roles.len(), 5);

        for account in steam.accounts() {
            let written = std::fs::read_to_string(&account.grid_path).unwrap();
            assert!(written.contains("Dota2ProTracker"));
        }
    }

    #[tokio::test]
    async fn refresh_all_separate_mode_writes_configs_intact() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir_all(root.join("userdata/111/570/remote/cfg")).unwrap();

        let steam = SteamLocator::with_root(root);

        refresh_all(&FakeProvider, &HeroMap::bundled(), &steam, None, "separate", None, "matches")
            .await
            .unwrap();

        let account = steam.accounts().into_iter().next().unwrap();
        let written = std::fs::read_to_string(&account.grid_path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&written).unwrap();
        let names: Vec<String> = v["configs"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|c| c["config_name"].as_str().map(|s| s.to_string()))
            .collect();

        assert!(names.contains(&"Dota2ProTracker 7.41e - All Roles".to_string()));
        assert!(names.contains(&"Dota2ProTracker 7.41e - Carry".to_string()));
    }

    #[tokio::test]
    async fn refresh_all_merge_injects_meta_and_preserves_user_config() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        let cfg = root.join("userdata/111/570/remote/cfg");
        std::fs::create_dir_all(&cfg).unwrap();
        let grid_path = cfg.join("hero_grid_config.json");
        std::fs::write(
            &grid_path,
            r#"{"version":3,"configs":[
                {"config_name":"Main Layout","categories":[
                    {"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":687.0,"height":293.0,"hero_ids":[1,2]}
                ]}
            ]}"#,
        )
        .unwrap();

        let steam = SteamLocator::with_root(root);
        refresh_all(&FakeProvider, &HeroMap::bundled(), &steam, None, "merge", Some("Main Layout"), "matches")
            .await
            .unwrap();

        let written = std::fs::read_to_string(&grid_path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&written).unwrap();
        let cfg0 = &v["configs"][0];
        assert_eq!(cfg0["config_name"], "Main Layout");
        let names: Vec<String> = cfg0["categories"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["category_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names.iter().any(|n| n == "META CARRY"));
        assert!(names.iter().any(|n| n == "2pos"));
        let user = cfg0["categories"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["category_name"] == "2pos")
            .unwrap();
        assert_eq!(user["x_position"].as_f64().unwrap(), 475.0);
    }
}
