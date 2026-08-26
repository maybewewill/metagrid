use crate::model::{MetaSnapshot, Position, SortMetric};
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

pub fn build_grid(snap: &MetaSnapshot, opts: &GridOptions) -> GridConfig {
    let categories = snap
        .roles
        .iter()
        .enumerate()
        .map(|(i, role)| {
            let mut heroes = role.heroes.clone();
            match opts.sort {
                SortMetric::Winrate => {
                    heroes.sort_by(|a, b| b.winrate.partial_cmp(&a.winrate).unwrap())
                }
                SortMetric::Pickrate => {
                    heroes.sort_by(|a, b| b.pickrate.partial_cmp(&a.pickrate).unwrap())
                }
            }
            let hero_ids = heroes.into_iter().map(|h| h.hero_id).collect();
            let x_position = if opts.layout_columns { i as f64 * 210.0 } else { 0.0 };
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
        })
        .collect();

    GridConfig {
        config_name: "MetaGrid".into(),
        categories,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    fn snap() -> MetaSnapshot {
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
                            matches: 1,
                        },
                        HeroMeta {
                            hero_id: 20,
                            name: "B".into(),
                            slug: "b".into(),
                            winrate: 0.40,
                            pickrate: 0.30,
                            matches: 1,
                        },
                    ],
                })
                .collect(),
        }
    }
    #[test]
    fn five_columns_and_sort_applies() {
        let g = build_grid(
            &snap(),
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
            &snap(),
            &GridOptions {
                sort: SortMetric::Pickrate,
                layout_columns: true,
            },
        );
        assert_eq!(g2.categories[0].hero_ids, vec![20, 10]); // pickrate desc → B before A
    }
}
