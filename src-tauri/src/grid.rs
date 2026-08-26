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
/// `"Support"`, `"Hard Support"` — each containing two categories:
/// 1. "TOP {ROLE} HEROES - ORDERED BY D2PT ELO" (top rating heroes)
/// 2. "OTHER {ROLE} HEROES - ORDERED BY PICK RATE" (remaining heroes)
pub fn build_grid_multi(snap: &MetaSnapshot) -> Vec<GridConfig> {
    snap.roles
        .iter()
        .map(|role| {
            let role_upper = role.position.role_upper();
            
            // Top heroes sorted by d2pt_rating desc (or winrate if rating is 0)
            let mut by_rating = role.heroes.clone();
            by_rating.sort_by(|a, b| {
                if b.d2pt_rating != a.d2pt_rating {
                    b.d2pt_rating.cmp(&a.d2pt_rating)
                } else {
                    b.winrate.partial_cmp(&a.winrate).unwrap_or(std::cmp::Ordering::Equal)
                }
            });

            let top_count = std::cmp::min(7, by_rating.len());
            let top_heroes = &by_rating[..top_count];
            let top_ids: std::collections::HashSet<u32> = top_heroes.iter().map(|h| h.hero_id).collect();

            // Other heroes sorted by pickrate/matches desc
            let mut other_heroes: Vec<_> = role
                .heroes
                .iter()
                .filter(|h| !top_ids.contains(&h.hero_id))
                .cloned()
                .collect();
            other_heroes.sort_by(|a, b| {
                b.pickrate
                    .partial_cmp(&a.pickrate)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let cat_top = Category {
                category_name: format!("TOP {role_upper} HEROES - ORDERED BY D2PT ELO"),
                x_position: 0.0,
                y_position: 0.0,
                width: 1100.0,
                height: 160.0,
                hero_ids: top_heroes.iter().map(|h| h.hero_id).collect(),
            };

            let cat_other = Category {
                category_name: format!("OTHER {role_upper} HEROES - ORDERED BY PICK RATE"),
                x_position: 0.0,
                y_position: 190.0,
                width: 1100.0,
                height: 380.0,
                hero_ids: other_heroes.iter().map(|h| h.hero_id).collect(),
            };

            GridConfig {
                config_name: role.position.config_name().to_string(),
                categories: vec![cat_top, cat_other],
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
                        },
                        HeroMeta {
                            hero_id: 20,
                            name: "B".into(),
                            slug: "b".into(),
                            winrate: 0.40,
                            pickrate: 0.30,
                            matches: 300,
                            d2pt_rating: 2900,
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
            assert_eq!(c.categories.len(), 2);
            assert!(c.categories[0].category_name.contains("TOP"));
            assert!(c.categories[1].category_name.contains("OTHER"));
        }
    }
}
