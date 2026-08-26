use crate::grid::{build_grid_multi, build_meta_categories};
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

#[allow(clippy::too_many_arguments)]
pub async fn refresh_all(
    provider: &dyn MetaProvider,
    map: &HeroMap,
    steam: &SteamLocator,
    top_n: usize,
    account_filter: Option<&str>,
    role_labels: &str,
    grid_mode: &str,
    merge_target: Option<&str>,
) -> Result<MetaSnapshot, PipelineError> {
    let snap: MetaSnapshot = provider.fetch(map, top_n).await?;

    let merge = grid_mode == "merge" && merge_target.is_some();
    let grids = if merge { Vec::new() } else { build_grid_multi(&snap, role_labels) };
    let meta_cats = if merge { build_meta_categories(&snap, role_labels, top_n) } else { Vec::new() };

    for account in steam.accounts() {
        if let Some(filter_id) = account_filter {
            if account.id != filter_id {
                continue;
            }
        }
        if merge {
            write_merge_to(&account.grid_path, &meta_cats, merge_target.unwrap())?;
        } else {
            write_configs_to(&account.grid_path, &grids)?;
        }
    }

    Ok(snap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{HeroMeta, Position, RoleMeta};
    use crate::provider::ProviderError;

    struct FakeProvider;

    #[async_trait::async_trait]
    impl MetaProvider for FakeProvider {
        async fn fetch(&self, _map: &HeroMap, _top_n: usize) -> Result<MetaSnapshot, ProviderError> {
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

            Ok(MetaSnapshot {
                patch: "7.41e".into(),
                fetched_at: "2026-08-26T10:00:00Z".into(),
                source: "fake".into(),
                roles,
            })
        }
    }

    #[tokio::test]
    async fn refresh_all_writes_to_each_account() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir_all(root.join("userdata/111/570/remote/cfg")).unwrap();
        std::fs::create_dir_all(root.join("userdata/222/570/remote/cfg")).unwrap();

        let steam = SteamLocator::with_root(root);

        let snap = refresh_all(&FakeProvider, &HeroMap::bundled(), &steam, 10, None, "named", "separate", None)
            .await
            .unwrap();

        assert_eq!(snap.roles.len(), 5);

        for account in steam.accounts() {
            let written = std::fs::read_to_string(&account.grid_path).unwrap();
            assert!(written.contains("Carry"));
        }
    }

    #[tokio::test]
    async fn refresh_all_multi_mode_writes_five_named_configs() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir_all(root.join("userdata/111/570/remote/cfg")).unwrap();

        let steam = SteamLocator::with_root(root);

        refresh_all(&FakeProvider, &HeroMap::bundled(), &steam, 10, None, "named", "separate", None)
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

        for role_name in ["Carry", "Mid", "Offlane", "Support", "Hard Support"] {
            assert!(
                names.contains(&role_name.to_string()),
                "missing {role_name}, got {names:?}"
            );
        }
        assert!(!names.contains(&"MetaGrid".to_string()));
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
        refresh_all(&FakeProvider, &HeroMap::bundled(), &steam, 10, None, "named", "merge", Some("Main Layout"))
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
        assert_eq!(user["x_position"].as_f64().unwrap(), 572.0);
    }

    #[tokio::test]
    #[ignore]
    async fn live_end_to_end() {
        use crate::provider::d2pt::D2ptProvider;
        use crate::steam::SteamLocator;

        let Some(loc) = SteamLocator::detect() else {
            eprintln!("SKIP: no steam");
            return;
        };

        let Some(account) = loc.accounts().into_iter().find(|a| a.grid_path.exists()) else {
            eprintln!("SKIP: no dota grid on this machine");
            return;
        };

        let real_path = account.grid_path.clone();

        let tmp_dir = tempfile::tempdir().unwrap();
        let copy_path = tmp_dir.path().join("hero_grid_config.json");
        std::fs::copy(&real_path, &copy_path).unwrap();

        let original_contents = std::fs::read_to_string(&copy_path).unwrap();
        let original_value: serde_json::Value =
            serde_json::from_str(&original_contents).unwrap();
        let original_names: Vec<String> = original_value["configs"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|c| c.get("config_name").and_then(|n| n.as_str()))
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let snap = D2ptProvider::new()
            .fetch(&HeroMap::bundled(), 12)
            .await
            .unwrap();

        let grids = build_grid_multi(&snap, "named");
        write_configs_to(&copy_path, &grids).unwrap();

        let after_contents = std::fs::read_to_string(&copy_path).unwrap();
        let after_value: serde_json::Value = serde_json::from_str(&after_contents).unwrap();
        let after_names: Vec<String> = after_value["configs"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|c| c.get("config_name").and_then(|n| n.as_str()))
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        for name in &original_names {
            assert!(
                after_names.contains(name),
                "original config {name:?} must survive the write"
            );
        }
        for role_name in ["Carry", "Mid", "Offlane", "Support", "Hard Support"] {
            assert!(
                after_names.contains(&role_name.to_string()),
                "missing config {role_name}"
            );
        }

        println!("=== LIVE END-TO-END PROOF ===");
        println!("real grid file (untouched):   {}", real_path.display());
        println!("temp copy (written):          {}", copy_path.display());
        println!("provider source:              {}", snap.source);
        println!("patch:                        {}", snap.patch);
        println!("roles:                        {}", snap.roles.len());
        println!();

        for role in &snap.roles {
            let top3: Vec<&str> = role.heroes.iter().take(3).map(|h| h.name.as_str()).collect();
            println!(
                "{:<24} heroes={:<3} top3={:?}",
                role.position.config_name("named"),
                role.heroes.len(),
                top3
            );
        }
    }
}
