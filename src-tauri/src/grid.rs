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

fn extract_tournament_title(source: &str) -> Option<&str> {
    if let Some(pos) = source.find("Tournament: ") {
        let rest = &source[pos + "Tournament: ".len()..];
        let name = rest.strip_suffix(')').unwrap_or(rest);
        Some(name)
    } else {
        None
    }
}

fn format_rate(rate: f32) -> String {
    let val = rate * 100.0;
    if (val - 100.0).abs() < 1e-4 {
        "100%".to_string()
    } else if val.abs() < 1e-4 {
        "0%".to_string()
    } else {
        format!("{:.1}%", val)
    }
}

pub fn build_grid_multi(snap: &MetaSnapshot, role_labels: &str) -> Vec<GridConfig> {
    let tourney_title = extract_tournament_title(&snap.source);
    snap.roles
        .iter()
        .map(|role| {
            let role_upper = role.position.role_upper(role_labels);
            let base_name = role.position.config_name(role_labels);
            let config_name = if let Some(tourney) = tourney_title {
                format!("MetaGrid - {tourney} - {base_name}")
            } else {
                format!("MetaGrid - {base_name}")
            };

            let mut categories = Vec::new();
            let mut cursor_y = 0.0_f64;

            if let Some(tourney) = tourney_title {
                categories.push(Category {
                    category_name: format!("Tournament: {}", tourney),
                    x_position: 0.0,
                    y_position: cursor_y,
                    width: 1100.0,
                    height: 24.0,
                    hero_ids: vec![],
                });
                cursor_y += 24.0;
            }

            let top_header = format!("— TOP {role_upper} HEROES - ORDERED BY D2PT ELO");

            categories.push(Category {
                category_name: top_header,
                x_position: 0.0,
                y_position: cursor_y,
                width: 1100.0,
                height: 24.0,
                hero_ids: vec![],
            });
            cursor_y += 24.0;

            let card_w = 68.0;
            let card_h = 104.0;
            let gap_x = 10.0;
            let gap_y = 10.0;
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

            let top_heroes_y = cursor_y;
            for (idx, h) in top_slice.iter().enumerate() {
                categories.push(Category {
                    category_name: format_rate(h.winrate),
                    x_position: (idx as f64) * (card_w + gap_x),
                    y_position: top_heroes_y,
                    width: card_w,
                    height: card_h,
                    hero_ids: vec![h.hero_id],
                });
            }

            let other_heroes: Vec<_> = role
                .heroes
                .iter()
                .filter(|h| !top_ids.contains(&h.hero_id))
                .take(20)
                .cloned()
                .collect();

            if !other_heroes.is_empty() {
                let other_header_y = top_heroes_y + card_h + 10.0;
                let other_header = format!("— OTHER {role_upper} HEROES - ORDERED BY D2PT RATING (AND PICKRATE)");
                categories.push(Category {
                    category_name: other_header,
                    x_position: 0.0,
                    y_position: other_header_y,
                    width: 1100.0,
                    height: 24.0,
                    hero_ids: vec![],
                });

                let other_heroes_start_y = other_header_y + 24.0;
                for (idx, h) in other_heroes.iter().enumerate() {
                    let col = idx % heroes_per_row;
                    let row = idx / heroes_per_row;
                    categories.push(Category {
                        category_name: format_rate(h.winrate),
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

pub fn build_meta_categories(snap: &MetaSnapshot, role_labels: &str, top_n: usize) -> Vec<Category> {
    const COMPACT_W: f64 = 330.0;
    const COMPACT_H: f64 = 100.0;
    const GAP: f64 = 15.0;

    let mut cats = Vec::new();
    let mut cursor_y = 0.0_f64;

    let is_tournament = extract_tournament_title(&snap.source).is_some();

    if let Some(tourney) = extract_tournament_title(&snap.source) {
        cats.push(Category {
            category_name: format!("Tournament: {}", tourney),
            x_position: 0.0,
            y_position: cursor_y,
            width: COMPACT_W,
            height: 0.0,
            hero_ids: vec![],
        });
        cursor_y += 30.0;
    }

    let take_count = if top_n == 0 { 7 } else { top_n.min(7) };

    for role in &snap.roles {
        let top_heroes: Vec<u32> = role
            .heroes
            .iter()
            .filter(|h| h.is_top)
            .take(take_count)
            .map(|h| h.hero_id)
            .collect();

        let hero_ids: Vec<u32> = if top_heroes.is_empty() {
            role.heroes.iter().take(take_count).map(|h| h.hero_id).collect()
        } else {
            top_heroes
        };

        if hero_ids.is_empty() {
            continue;
        }

        let cat_name = if is_tournament {
            format!("— META {}", role.position.role_upper(role_labels))
        } else {
            format!("META {}", role.position.role_upper(role_labels))
        };

        cats.push(Category {
            category_name: cat_name,
            x_position: 0.0,
            y_position: cursor_y,
            width: COMPACT_W,
            height: COMPACT_H,
            hero_ids,
        });
        cursor_y += COMPACT_H + GAP;
    }
    cats
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
                    heroes: (1..=10)
                        .map(|i| HeroMeta {
                            hero_id: i * 10 + p as u32,
                            name: format!("Hero {i}"),
                            slug: format!("hero-{i}"),
                            winrate: 0.55,
                            pickrate: 0.05,
                            matches: 100,
                            d2pt_rating: 3000,
                            is_top: i <= 3,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
    #[test]
    fn multi_layout_makes_five_named_configs() {
        let cfgs = build_grid_multi(&sample_snapshot(), "named");
        assert_eq!(cfgs.len(), 5);
        assert!(cfgs.iter().any(|c| c.config_name == "MetaGrid - Carry"));
        assert!(cfgs.iter().any(|c| c.config_name == "MetaGrid - Mid"));
        assert!(cfgs.iter().any(|c| c.config_name == "MetaGrid - Offlane"));
        assert!(cfgs.iter().any(|c| c.config_name == "MetaGrid - Support"));
        assert!(cfgs.iter().any(|c| c.config_name == "MetaGrid - Hard Support"));
        for c in &cfgs {
            assert!(c.categories.len() >= 2);
            assert!(c.categories[0].category_name.contains("TOP"));
        }

        let pos_cfgs = build_grid_multi(&sample_snapshot(), "pos");
        assert!(pos_cfgs.iter().any(|c| c.config_name == "MetaGrid - POS 1"));
        assert!(pos_cfgs[0].categories[0].category_name.contains("POS 1"));
    }

    #[test]
    fn multi_layout_with_tournament_prefixes_config_name() {
        let mut snap = sample_snapshot();
        snap.source = "d2pt (Tournament: BLAST Slam I)".into();
        let cfgs = build_grid_multi(&snap, "named");
        assert_eq!(cfgs.len(), 5);
        assert!(cfgs.iter().any(|c| c.config_name == "MetaGrid - BLAST Slam I - Carry"));
        assert_eq!(cfgs[0].categories[0].category_name, "Tournament: BLAST Slam I");
        assert!(cfgs[0].categories[1].category_name.starts_with("— TOP"));
    }

    #[test]
    fn meta_categories_boxes_fit_all_heroes() {
        let cats = build_meta_categories(&sample_snapshot(), "named", 10);
        assert_eq!(cats.len(), 5);
        assert!(cats.iter().all(|c| c.category_name.starts_with("META ")));
        assert!(cats.iter().any(|c| c.category_name == "META CARRY"));
        assert!(cats.iter().all(|c| c.x_position == 0.0));
        assert!(cats.iter().all(|c| c.width == 330.0));
        assert!(cats.iter().all(|c| c.height == 100.0));

        for w in cats.windows(2) {
            assert!(w[1].y_position >= w[0].y_position + w[0].height);
        }
    }
}
