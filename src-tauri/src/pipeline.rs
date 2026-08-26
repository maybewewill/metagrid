use std::path::Path;

use crate::grid::{build_grid, GridOptions};
use crate::grid_writer::{write_to, GridError, METAGRID_NAME};
use crate::hero_map::HeroMap;
use crate::model::MetaSnapshot;
use crate::provider::{MetaProvider, ProviderError};

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error(transparent)]
    Grid(#[from] GridError),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RunSummary {
    pub source: String,
    pub patch: String,
    pub roles: usize,
    pub total_heroes: usize,
    pub grid_path: String,
    pub preserved_configs: Vec<String>,
    pub wrote_metagrid: bool,
}

/// Read `grid_path` (if present) and collect the `config_name`s of every
/// config in it except `"MetaGrid"`. Missing, empty, or unparseable files
/// are treated as "nothing to preserve" — `write_to` enforces its own
/// safety around the actual write, so this read is best-effort/informational
/// only and must never error the pipeline.
fn read_preserved_config_names(grid_path: &Path) -> Vec<String> {
    let Ok(contents) = std::fs::read_to_string(grid_path) else {
        return Vec::new();
    };
    if contents.trim().is_empty() {
        return Vec::new();
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&contents) else {
        return Vec::new();
    };
    let Some(configs) = value.get("configs").and_then(|c| c.as_array()) else {
        return Vec::new();
    };
    configs
        .iter()
        .filter_map(|c| c.get("config_name").and_then(|n| n.as_str()))
        .filter(|name| *name != METAGRID_NAME)
        .map(|name| name.to_string())
        .collect()
}

/// Fetch the current meta from `provider`, build a MetaGrid layout from it,
/// and write it into `grid_path` — preserving every other config already in
/// that file.
pub async fn run_once(
    provider: &dyn MetaProvider,
    map: &HeroMap,
    grid_path: &Path,
    opts: &GridOptions,
    top_n: usize,
) -> Result<RunSummary, PipelineError> {
    let snap: MetaSnapshot = provider.fetch(map, top_n).await?;

    let preserved_configs = read_preserved_config_names(grid_path);

    let grid = build_grid(&snap, opts);
    write_to(grid_path, &grid)?;

    Ok(RunSummary {
        source: snap.source.clone(),
        patch: snap.patch.clone(),
        roles: snap.roles.len(),
        total_heroes: snap.roles.iter().map(|r| r.heroes.len()).sum(),
        grid_path: grid_path.display().to_string(),
        preserved_configs,
        wrote_metagrid: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{HeroMeta, Position, RoleMeta, SortMetric};
    use crate::provider::ProviderError;

    struct FakeProvider;

    #[async_trait::async_trait]
    impl MetaProvider for FakeProvider {
        fn id(&self) -> &'static str {
            "fake"
        }

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
                        },
                        HeroMeta {
                            hero_id: (i as u32) * 10 + 2,
                            name: format!("Hero{}B", i),
                            slug: format!("hero{}b", i),
                            winrate: 0.45,
                            pickrate: 0.10,
                            matches: 50,
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
    async fn run_once_preserves_and_writes() {
        let dir = tempfile::tempdir().unwrap();
        let grid_path = dir.path().join("grid_config.json");
        let seed = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[{"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":10.0,"height":10.0,"hero_ids":[1,2]}]}
        ]}"#;
        std::fs::write(&grid_path, seed).unwrap();

        let opts = GridOptions {
            sort: SortMetric::Pickrate,
            layout_columns: true,
        };

        let summary = run_once(&FakeProvider, &HeroMap::bundled(), &grid_path, &opts, 10)
            .await
            .unwrap();

        let written = std::fs::read_to_string(&grid_path).unwrap();
        assert!(written.contains("Main Layout"));
        assert!(written.contains("MetaGrid"));
        assert_eq!(summary.preserved_configs, vec!["Main Layout".to_string()]);
        assert_eq!(summary.roles, 5);
        assert!(summary.wrote_metagrid);
    }

    /// Live end-to-end proof: fetches the REAL d2pt site and writes a REAL
    /// hero_grid_config.json format grid — but ONLY into a temp COPY of the
    /// user's real file, never the real file itself. Run explicitly with:
    /// `cargo test --manifest-path src-tauri/Cargo.toml -- --ignored live_end_to_end --nocapture`
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

        let opts = GridOptions {
            sort: SortMetric::Pickrate,
            layout_columns: true,
        };

        let summary = run_once(
            &D2ptProvider::new(),
            &HeroMap::bundled(),
            &copy_path,
            &opts,
            12,
        )
        .await
        .unwrap();

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
        assert!(
            after_names.iter().any(|n| n == "MetaGrid"),
            "MetaGrid config must be present after write"
        );

        println!("=== LIVE END-TO-END PROOF ===");
        println!("real grid file (untouched):   {}", real_path.display());
        println!("temp copy (written):          {}", copy_path.display());
        println!("provider source:              {}", summary.source);
        println!("patch:                        {}", summary.patch);
        println!("roles:                        {}", summary.roles);
        println!("total heroes:                 {}", summary.total_heroes);
        println!();

        let snap = D2ptProvider::new()
            .fetch(&HeroMap::bundled(), 12)
            .await
            .unwrap();
        for role in &snap.roles {
            let top3: Vec<&str> = role.heroes.iter().take(3).map(|h| h.name.as_str()).collect();
            println!(
                "{:<24} heroes={:<3} top3={:?}",
                role.position.label("en"),
                role.heroes.len(),
                top3
            );
        }

        println!();
        println!("preserved foreign configs: {:?}", summary.preserved_configs);
        println!("temp path written:         {}", copy_path.display());
    }
}
