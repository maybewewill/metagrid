use crate::model::{MetaSnapshot, Position, RoleMeta, SortMetric};
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

pub struct GridOptions {
    pub sort: SortMetric,
    pub layout_columns: bool,
}

/// Builds a single role's `Category`: heroes sorted by `sort`, geometry
/// pinned at `x_position` (the caller decides column layout, if any).
/// Shared by both `build_grid` (one category per role, in one config) and
/// `build_grid_multi` (one category — and one config — per role).
fn build_category(role: &RoleMeta, sort: SortMetric, x_position: f64) -> Category {
    let mut heroes = role.heroes.clone();
    match sort {
        SortMetric::Winrate => heroes.sort_by(|a, b| b.winrate.partial_cmp(&a.winrate).unwrap()),
        SortMetric::Pickrate => {
            heroes.sort_by(|a, b| b.pickrate.partial_cmp(&a.pickrate).unwrap())
        }
    }
    let hero_ids = heroes.into_iter().map(|h| h.hero_id).collect();
    Category {
        category_name: format!(
            "{} (WR {}%)",
            role.position.label("en"),
            (role.role_winrate * 100.0).round() as i32
        ),
        x_position,
        y_position: 0.0,
        width: 200.0,
        height: 200.0,
        hero_ids,
    }
}

/// Maps a `Position` to its 1-based POS number, used for the
/// `"MetaGrid POS n"` config names in `build_grid_multi`.
fn position_number(p: Position) -> u8 {
    match p {
        Position::Pos1 => 1,
        Position::Pos2 => 2,
        Position::Pos3 => 3,
        Position::Pos4 => 4,
        Position::Pos5 => 5,
    }
}

pub fn build_grid(snap: &MetaSnapshot, opts: &GridOptions) -> GridConfig {
    let categories = snap
        .roles
        .iter()
        .enumerate()
        .map(|(i, role)| {
            let x_position = if opts.layout_columns { i as f64 * 210.0 } else { 0.0 };
            build_category(role, opts.sort, x_position)
        })
        .collect();

    GridConfig {
        config_name: "MetaGrid".into(),
        categories,
    }
}

/// Per-role layout mode: produces one `GridConfig` per role — `"Carry"`, `"Mid"`, `"Offlane"`,
/// `"Support"`, `"Hard Support"` — containing the 7 Top Heroes from D2PT ELO followed by
/// Other Heroes sorted by D2PT rating, with Winrate % and Pickrate % displayed above each portrait.
pub fn build_grid_multi(snap: &MetaSnapshot) -> Vec<GridConfig> {
    snap.roles
        .iter()
        .map(|role| {
            let role_upper = role.position.role_upper();
            
            let mut categories = Vec::new();

            // Header 1: TOP HEROES
            categories.push(Category {
                category_name: format!("TOP {role_upper} HEROES - ORDERED BY D2PT ELO"),
                x_position: 0.0,
                y_position: 0.0,
                width: 1100.0,
                height: 0.0,
                hero_ids: vec![],
            });

            let card_w = 68.0;
            let card_h = 110.0;
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

            // Top heroes boxes in a single row
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

            // Other heroes (excluding top heroes)
            let other_heroes: Vec<_> = role
                .heroes
                .iter()
                .filter(|h| !top_ids.contains(&h.hero_id))
                .cloned()
                .collect();

            if !other_heroes.is_empty() {
                let other_header_y = 30.0 + card_h + 35.0; // 175.0
                categories.push(Category {
                    category_name: format!("OTHER {role_upper} HEROES - ORDERED BY D2PT RATING"),
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
                config_name: role.position.config_name().to_string(),
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
    fn five_columns_and_sort_applies() {
        let g = build_grid(
            &sample_snapshot(),
            &GridOptions {
                sort: SortMetric::Winrate,
                layout_columns: true,
            },
        );
        assert_eq!(g.config_name, "MetaGrid");
        assert_eq!(g.categories.len(), 5);
        assert!(g.categories[1].x_position > g.categories[0].x_position);
        assert_eq!(g.categories[0].hero_ids, vec![10, 20]); // winrate desc → A before B
        let g2 = build_grid(
            &sample_snapshot(),
            &GridOptions {
                sort: SortMetric::Pickrate,
                layout_columns: true,
            },
        );
        assert_eq!(g2.categories[0].hero_ids, vec![20, 10]); // pickrate desc → B before A
    }

    #[test]
    fn multi_layout_makes_five_named_configs() {
        let cfgs = build_grid_multi(&sample_snapshot());
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
    }
}
