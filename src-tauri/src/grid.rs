use crate::model::MetaSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub category_name: String,
    pub x_position: f64,
    pub y_position: f64,
    pub width: f64,
    pub height: f64,
    pub hero_ids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridConfig {
    pub config_name: String,
    pub categories: Vec<Category>,
}

impl GridConfig {
    #[cfg(test)]
    pub fn sample_metagrid() -> GridConfig {
        GridConfig {
            config_name: "MetaGrid".into(),
            categories: vec![Category {
                category_name: "POS 1 — Carry (WR 52%)".into(),
                x_position: 0.0,
                y_position: 0.0,
                width: 200.0,
                height: 200.0,
                hero_ids: vec![10, 20],
            }],
        }
    }
}

pub fn build_grid_multi(snap: &MetaSnapshot, role_labels: &str) -> Vec<GridConfig> {
    snap.roles
        .iter()
        .map(|role| {
            let role_upper = role.position.role_upper(role_labels);
            let config_name = role.position.config_name(role_labels).to_string();

            let mut categories = Vec::new();

            categories.push(Category {
                category_name: format!("TOP {role_upper} HEROES - ORDERED BY D2PT ELO"),
                x_position: 0.0,
                y_position: 0.0,
                width: 1100.0,
                height: 0.0,
                hero_ids: vec![],
            });

            let card_w = 68.0;
            let card_h = 104.0;
            let gap_x = 12.0;
            let gap_y = 24.0;
            let heroes_per_row = 14;

            let top_heroes: Vec<_> = role.heroes.iter().filter(|h| h.is_top).cloned().collect();
            let top_count = if top_heroes.is_empty() {
                std::cmp::min(7, role.heroes.len())
            } else {
                top_heroes.len()
            };

            let top_slice = if top_heroes.is_empty() {
                &role.heroes[..top_count]
            } else {
                &top_heroes[..]
            };

            let top_ids: std::collections::HashSet<u32> = top_slice.iter().map(|h| h.hero_id).collect();

            for (idx, h) in top_slice.iter().enumerate() {
                categories.push(Category {
                    category_name: format!("{:.2}%\n{:.2}%", h.winrate * 100.0, h.pickrate * 100.0),
                    x_position: (idx as f64) * (card_w + gap_x),
                    y_position: 30.0,
                    width: card_w,
                    height: card_h,
                    hero_ids: vec![h.hero_id],
                });
            }

            let other_heroes: Vec<_> = role
                .heroes
                .iter()
                .filter(|h| !top_ids.contains(&h.hero_id))
                .cloned()
                .collect();

            if !other_heroes.is_empty() {
                let other_header_y = 30.0 + card_h + 35.0;
                categories.push(Category {
                    category_name: format!("OTHER {role_upper} HEROES - ORDERED BY D2PT RATING (AND PICKRATE)"),
                    x_position: 0.0,
                    y_position: other_header_y,
                    width: 1100.0,
                    height: 0.0,
                    hero_ids: vec![],
                });

                let other_heroes_start_y = other_header_y + 30.0;
                for (idx, h) in other_heroes.iter().enumerate() {
                    let col = idx % heroes_per_row;
                    let row = idx / heroes_per_row;
                    categories.push(Category {
                        category_name: format!("{:.2}%\n{:.2}%", h.winrate * 100.0, h.pickrate * 100.0),
                        x_position: (col as f64) * (card_w + gap_x),
                        y_position: other_heroes_start_y + (row as f64) * (card_h + gap_y),
                        width: card_w,
                        height: card_h,
                        hero_ids: vec![h.hero_id],
                    });
                }
            }

            GridConfig {
                config_name,
                categories,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    fn sample_snapshot() -> MetaSnapshot {
        MetaSnapshot {
            patch: "7.41e".into(),
            fetched_at: "t".into(),
            source: "d2pt".into(),
            roles: Position::all()
                .iter()
                .map(|&p| RoleMeta {
                    position: p,
                    role_winrate: 0.5,
                    heroes: vec![
                        HeroMeta {
                            hero_id: 10,
                            name: "A".into(),
                            slug: "a".into(),
                            winrate: 0.60,
                            pickrate: 0.10,
                            matches: 100,
                            d2pt_rating: 3200,
                            is_top: true,
                        },
                        HeroMeta {
                            hero_id: 20,
                            name: "B".into(),
                            slug: "b".into(),
                            winrate: 0.40,
                            pickrate: 0.30,
                            matches: 300,
                            d2pt_rating: 2900,
                            is_top: false,
                        },
                    ],
                })
                .collect(),
        }
    }
    #[test]
    fn multi_layout_makes_five_named_configs() {
        let cfgs = build_grid_multi(&sample_snapshot(), "named");
        assert_eq!(cfgs.len(), 5);
        assert!(cfgs.iter().any(|c| c.config_name == "Carry"));
        assert!(cfgs.iter().any(|c| c.config_name == "Mid"));
        assert!(cfgs.iter().any(|c| c.config_name == "Offlane"));
        assert!(cfgs.iter().any(|c| c.config_name == "Support"));
        assert!(cfgs.iter().any(|c| c.config_name == "Hard Support"));
        for c in &cfgs {
            assert!(c.categories.len() >= 2);
            assert!(c.categories[0].category_name.contains("TOP"));
        }

        let pos_cfgs = build_grid_multi(&sample_snapshot(), "pos");
        assert!(pos_cfgs.iter().any(|c| c.config_name == "POS 1"));
        assert!(pos_cfgs[0].categories[0].category_name.contains("POS 1"));
    }
}
