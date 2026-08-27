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
                category_name: "Carry".into(),
                x_position: 0.0,
                y_position: 0.0,
                width: 200.0,
                height: 200.0,
                hero_ids: vec![10, 20],
            }],
        }
    }
}

pub fn extract_role_categories_for_merge(d2pt_grids: &[GridConfig]) -> Vec<Category> {
    const COMPACT_W: f64 = 455.0;
    const COMPACT_H: f64 = 75.0;
    const GAP: f64 = 20.0;

    let mut result = Vec::new();
    let mut cursor_y = 0.0;

    if let Some(all_roles) = d2pt_grids.iter().find(|g| g.config_name.contains("All Roles")) {
        for cat in &all_roles.categories {
            let name_lower = cat.category_name.trim().to_lowercase();
            if name_lower == "all heroes" || name_lower.contains("best") || name_lower.contains("worst") {
                continue;
            }
            if matches!(name_lower.as_str(), "carry" | "mid" | "offlane" | "support" | "hard support" | "pos 1" | "pos 2" | "pos 3" | "pos 4" | "pos 5") {
                let role_title = match name_lower.as_str() {
                    "carry" | "pos 1" => "META CARRY",
                    "mid" | "pos 2" => "META MID",
                    "offlane" | "pos 3" => "META OFFLANE",
                    "support" | "pos 4" => "META SUPPORT",
                    "hard support" | "pos 5" => "META HARD SUPPORT",
                    _ => &cat.category_name,
                };
                result.push(Category {
                    category_name: role_title.to_string(),
                    x_position: 0.0,
                    y_position: cursor_y,
                    width: COMPACT_W,
                    height: COMPACT_H,
                    hero_ids: cat.hero_ids.clone(),
                });
                cursor_y += COMPACT_H + GAP;
            }
        }
    } else {
        for grid in d2pt_grids {
            for cat in &grid.categories {
                if cat.category_name.starts_with("Top Heroes") {
                    let role_title = if cat.category_name.contains("Pos 1") {
                        "META CARRY"
                    } else if cat.category_name.contains("Pos 2") {
                        "META MID"
                    } else if cat.category_name.contains("Pos 3") {
                        "META OFFLANE"
                    } else if cat.category_name.contains("Pos 4") {
                        "META SUPPORT"
                    } else if cat.category_name.contains("Pos 5") {
                        "META HARD SUPPORT"
                    } else {
                        "META HEROES"
                    };
                    result.push(Category {
                        category_name: role_title.to_string(),
                        x_position: 0.0,
                        y_position: cursor_y,
                        width: COMPACT_W,
                        height: COMPACT_H,
                        hero_ids: cat.hero_ids.clone(),
                    });
                    cursor_y += COMPACT_H + GAP;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_role_categories_skipping_all_heroes_and_synergies() {
        let sample_d2pt = vec![
            GridConfig {
                config_name: "Dota2ProTracker 7.41e - All Roles".into(),
                categories: vec![
                    Category {
                        category_name: "Carry".into(),
                        x_position: 0.0,
                        y_position: 0.0,
                        width: 455.0,
                        height: 75.0,
                        hero_ids: vec![1, 2, 3],
                    },
                    Category {
                        category_name: "Mid".into(),
                        x_position: 0.0,
                        y_position: 95.0,
                        width: 455.0,
                        height: 75.0,
                        hero_ids: vec![4, 5, 6],
                    },
                    Category {
                        category_name: "Offlane".into(),
                        x_position: 0.0,
                        y_position: 190.0,
                        width: 455.0,
                        height: 75.0,
                        hero_ids: vec![7, 8, 9],
                    },
                    Category {
                        category_name: "Support".into(),
                        x_position: 0.0,
                        y_position: 285.0,
                        width: 455.0,
                        height: 75.0,
                        hero_ids: vec![10, 11, 12],
                    },
                    Category {
                        category_name: "Hard Support".into(),
                        x_position: 0.0,
                        y_position: 380.0,
                        width: 455.0,
                        height: 75.0,
                        hero_ids: vec![13, 14, 15],
                    },
                    Category {
                        category_name: "All Heroes".into(),
                        x_position: 500.0,
                        y_position: 0.0,
                        width: 600.0,
                        height: 600.0,
                        hero_ids: (1..100).collect(),
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
                        hero_ids: vec![1, 2, 3],
                    },
                    Category {
                        category_name: "Best with".into(),
                        x_position: 75.0,
                        y_position: 20.0,
                        width: 280.0,
                        height: 55.0,
                        hero_ids: vec![10, 11],
                    },
                ],
            },
        ];

        let cats = extract_role_categories_for_merge(&sample_d2pt);
        assert_eq!(cats.len(), 5);
        assert_eq!(cats[0].category_name, "META CARRY");
        assert_eq!(cats[1].category_name, "META MID");
        assert_eq!(cats[2].category_name, "META OFFLANE");
        assert_eq!(cats[3].category_name, "META SUPPORT");
        assert_eq!(cats[4].category_name, "META HARD SUPPORT");
        assert_eq!(cats[0].hero_ids, vec![1, 2, 3]);
        assert_eq!(cats[4].hero_ids, vec![13, 14, 15]);
    }
}
