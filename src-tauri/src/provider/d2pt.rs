use serde::Deserialize;

use crate::grid::GridConfig;
use crate::hero_map::HeroMap;
use crate::model::{HeroMeta, MetaSnapshot, Position, RoleMeta};
use crate::provider::{MetaProvider, ProviderError};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36";
const ACCEPT: &str = "application/json, text/plain, */*";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9";

#[derive(Debug, Deserialize)]
struct D2ptDownloadRoot {
    configs: Vec<GridConfig>,
}

pub struct D2ptProvider;

impl D2ptProvider {
    pub fn new() -> Self {
        D2ptProvider
    }
}

impl Default for D2ptProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl D2ptProvider {
    async fn fetch_http(&self, url: &str) -> Result<String, ProviderError> {
        let program = if cfg!(target_os = "windows") { "curl.exe" } else { "curl" };
        let mut cmd = tokio::process::Command::new(program);
        cmd.arg("-s")
            .arg("--connect-timeout")
            .arg("10")
            .arg("--max-time")
            .arg("25")
            .arg("-H")
            .arg(format!("User-Agent: {USER_AGENT}"))
            .arg("-H")
            .arg(format!("Accept: {ACCEPT}"))
            .arg("-H")
            .arg(format!("Accept-Language: {ACCEPT_LANGUAGE}"))
            .arg(url);

        #[cfg(target_os = "windows")]
        {
            cmd.creation_flags(0x0800_0000);
        }

        if let Ok(output) = cmd.output().await {
            if output.status.success() {
                let body = String::from_utf8_lossy(&output.stdout).to_string();
                if !body.is_empty() && !body.contains("Just a moment") {
                    return Ok(body);
                }
            }
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| ProviderError::Parse(e.to_string()))?;

        let res = client
            .get(url)
            .header("Accept", ACCEPT)
            .header("Accept-Language", ACCEPT_LANGUAGE)
            .send()
            .await
            .map_err(|_| ProviderError::Blocked)?;

        let body = res.text().await.map_err(|_| ProviderError::Blocked)?;
        if body.is_empty() || body.contains("Just a moment") {
            return Err(ProviderError::Blocked);
        }

        Ok(body)
    }

    pub fn extract_patch(configs: &[GridConfig]) -> String {
        for cfg in configs {
            let name = &cfg.config_name;
            if let Some(pos) = name.find("Dota2ProTracker ") {
                let rest = &name[pos + "Dota2ProTracker ".len()..];
                if let Some(space_idx) = rest.find(' ') {
                    let patch = &rest[..space_idx];
                    if !patch.is_empty() {
                        return patch.to_string();
                    }
                }
            }
        }
        "Latest".to_string()
    }

    pub fn build_snapshot(&self, map: &HeroMap, patch: &str, mode: &str, configs: &[GridConfig]) -> MetaSnapshot {
        let mut pos1_heroes = Vec::new();
        let mut pos2_heroes = Vec::new();
        let mut pos3_heroes = Vec::new();
        let mut pos4_heroes = Vec::new();
        let mut pos5_heroes = Vec::new();

        if let Some(all_roles) = configs.iter().find(|c| c.config_name.contains("All Roles")) {
            for cat in &all_roles.categories {
                let name = cat.category_name.trim().to_lowercase();
                let list = match name.as_str() {
                    "carry" | "pos 1" => &mut pos1_heroes,
                    "mid" | "pos 2" => &mut pos2_heroes,
                    "offlane" | "pos 3" => &mut pos3_heroes,
                    "support" | "pos 4" => &mut pos4_heroes,
                    "hard support" | "pos 5" => &mut pos5_heroes,
                    _ => continue,
                };
                for &id in &cat.hero_ids {
                    list.push(HeroMeta {
                        hero_id: id,
                        name: map.name_for(id).unwrap_or("Hero").to_string(),
                        slug: map.slug_for(id).unwrap_or("").to_string(),
                        winrate: 0.50,
                        pickrate: 0.0,
                        matches: 0,
                        d2pt_rating: 0,
                        is_top: true,
                    });
                }
            }
        } else {
            for cfg in configs {
                for cat in &cfg.categories {
                    let name = cat.category_name.trim();
                    let list = if name.contains("Pos 1") || cfg.config_name.contains("Carry") {
                        &mut pos1_heroes
                    } else if name.contains("Pos 2") || cfg.config_name.contains("Mid") {
                        &mut pos2_heroes
                    } else if name.contains("Pos 3") || cfg.config_name.contains("Offlane") {
                        &mut pos3_heroes
                    } else if name.contains("Pos 4") || (cfg.config_name.contains("Support") && !cfg.config_name.contains("Hard Support")) {
                        &mut pos4_heroes
                    } else if name.contains("Pos 5") || cfg.config_name.contains("Hard Support") {
                        &mut pos5_heroes
                    } else {
                        continue;
                    };
                    if name.starts_with("Top Heroes") {
                        for &id in &cat.hero_ids {
                            if !list.iter().any(|h| h.hero_id == id) {
                                list.push(HeroMeta {
                                    hero_id: id,
                                    name: map.name_for(id).unwrap_or("Hero").to_string(),
                                    slug: map.slug_for(id).unwrap_or("").to_string(),
                                    winrate: 0.50,
                                    pickrate: 0.0,
                                    matches: 0,
                                    d2pt_rating: 0,
                                    is_top: true,
                                });
                            }
                        }
                    }
                }
            }
        }

        let roles = vec![
            RoleMeta { position: Position::Pos1, role_winrate: 0.50, heroes: pos1_heroes },
            RoleMeta { position: Position::Pos2, role_winrate: 0.50, heroes: pos2_heroes },
            RoleMeta { position: Position::Pos3, role_winrate: 0.50, heroes: pos3_heroes },
            RoleMeta { position: Position::Pos4, role_winrate: 0.50, heroes: pos4_heroes },
            RoleMeta { position: Position::Pos5, role_winrate: 0.50, heroes: pos5_heroes },
        ];

        let mode_label = match mode {
            "matches_wr" => "High Winrate",
            "d2ptrating" => "D2PT Rating",
            _ => "Most Played",
        };

        MetaSnapshot {
            patch: patch.to_string(),
            fetched_at: chrono::Utc::now().to_rfc3339(),
            source: format!("Dota2ProTracker ({mode_label})"),
            roles,
            configs: configs.to_vec(),
        }
    }
}

#[async_trait::async_trait]
impl MetaProvider for D2ptProvider {
    async fn fetch(&self, map: &HeroMap, meta_mode: &str) -> Result<(MetaSnapshot, Vec<GridConfig>), ProviderError> {
        let valid_mode = match meta_mode {
            "matches_wr" => "matches_wr",
            "d2ptrating" => "d2ptrating",
            _ => "matches",
        };

        let url = format!("https://dota2protracker.com/meta-hero-grids/download?mode={valid_mode}&patch=latest");
        let body = self.fetch_http(&url).await?;

        let parsed: D2ptDownloadRoot = serde_json::from_str(&body)
            .map_err(|e| ProviderError::Parse(format!("Failed to parse D2PT hero grid JSON: {e}")))?;

        let patch = Self::extract_patch(&parsed.configs);
        let snap = self.build_snapshot(map, &patch, valid_mode, &parsed.configs);

        Ok((snap, parsed.configs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_d2pt_downloaded_json_fixture() {
        let fixture = r#"{
            "configs": [
                {
                    "config_name": "Dota2ProTracker 7.41e - All Roles",
                    "categories": [
                        { "category_name": "Carry", "x_position": 0, "y_position": 0, "width": 455, "height": 75, "hero_ids": [11, 48] },
                        { "category_name": "Mid", "x_position": 0, "y_position": 95, "width": 455, "height": 75, "hero_ids": [25, 74] },
                        { "category_name": "Offlane", "x_position": 0, "y_position": 190, "width": 455, "height": 75, "hero_ids": [135, 2] },
                        { "category_name": "Support", "x_position": 0, "y_position": 285, "width": 455, "height": 75, "hero_ids": [86, 123] },
                        { "category_name": "Hard Support", "x_position": 0, "y_position": 380, "width": 455, "height": 75, "hero_ids": [83, 85] },
                        { "category_name": "All Heroes", "x_position": 500, "y_position": 0, "width": 600, "height": 600, "hero_ids": [1, 2] }
                    ]
                }
            ]
        }"#;

        let parsed: D2ptDownloadRoot = serde_json::from_str(fixture).unwrap();
        assert_eq!(parsed.configs.len(), 1);
        let patch = D2ptProvider::extract_patch(&parsed.configs);
        assert_eq!(patch, "7.41e");

        let map = HeroMap::bundled();
        let provider = D2ptProvider::new();
        let snap = provider.build_snapshot(&map, &patch, "matches", &parsed.configs);

        assert_eq!(snap.patch, "7.41e");
        assert_eq!(snap.roles.len(), 5);
        assert_eq!(snap.roles[0].position, Position::Pos1);
        assert_eq!(snap.roles[0].heroes.len(), 2);
        assert_eq!(snap.roles[0].heroes[0].name, "Shadow Fiend");
    }

    #[tokio::test]
    #[ignore]
    async fn d2pt_live_fetch() {
        let provider = D2ptProvider::new();
        let map = HeroMap::bundled();
        let (snap, configs) = provider.fetch(&map, "matches").await.unwrap();
        assert!(!snap.patch.is_empty());
        assert_eq!(snap.roles.len(), 5);
        assert_eq!(configs.len(), 6);
    }
}
