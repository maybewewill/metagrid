use crate::grid::{Category, GridConfig};
use std::io;
use std::path::Path;
use thiserror::Error;

const MERGE_GAP: f64 = 20.0;
const MAX_CANVAS_X: f64 = 1610.0;
const MAX_CANVAS_Y: f64 = 820.0;

#[derive(Debug, Error)]
pub enum GridError {
    #[error("failed to parse existing grid config JSON: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("existing grid config has unexpected shape: {0}")]
    Shape(String),

    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

pub fn is_metagrid_config(name: &str) -> bool {
    let n = name.trim();
    n == "MetaGrid"
        || n.starts_with("MetaGrid")
        || n.starts_with('⚡')
        || n.starts_with("⚡ ")
        || matches!(
            n.to_ascii_uppercase().as_str(),
            "CARRY"
                | "MID"
                | "OFFLANE"
                | "SUPPORT"
                | "HARD SUPPORT"
                | "POS 1"
                | "POS 2"
                | "POS 3"
                | "POS 4"
                | "POS 5"
                | "POS1"
                | "POS2"
                | "POS3"
                | "POS4"
                | "POS5"
        )
        || matches!(
            n,
            "Керри"
                | "Мид"
                | "Оффлейн"
                | "Саппорт"
                | "Хард Саппорт"
                | "ПОЗ 1"
                | "ПОЗ 2"
                | "ПОЗ 3"
                | "ПОЗ 4"
                | "ПОЗ 5"
        )
}

pub fn upsert_configs(existing_json: &str, our_grids: &[GridConfig]) -> Result<String, GridError> {
    let mut root: serde_json::Value = if existing_json.trim().is_empty() {
        serde_json::json!({"version": 3, "configs": []})
    } else {
        serde_json::from_str(existing_json)?
    };

    let obj = root
        .as_object_mut()
        .ok_or_else(|| GridError::Shape("root is not a JSON object".to_string()))?;

    let configs_value = obj
        .entry("configs")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    let configs = configs_value
        .as_array_mut()
        .ok_or_else(|| GridError::Shape("\"configs\" is not an array".to_string()))?;

    configs.retain(|c| {
        let name = c.get("config_name").and_then(|v| v.as_str()).unwrap_or("");
        !is_metagrid_config(name)
    });

    for our in our_grids {
        let our_value = serde_json::to_value(our)?;
        configs.push(our_value);
    }

    Ok(serde_json::to_string_pretty(&root)?)
}

#[allow(dead_code)]
pub fn upsert_config(existing_json: &str, our: &GridConfig) -> Result<String, GridError> {
    upsert_configs(existing_json, std::slice::from_ref(our))
}

fn backup_and_write(path: &Path, existing: &str, new_json: &str) -> Result<(), GridError> {
    if !existing.trim().is_empty() {
        let bak_path = path.with_extension("json.metagrid.bak");
        if !bak_path.exists() {
            std::fs::copy(path, &bak_path)?;
        }
    }

    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let tmp_file_name = format!(
        ".{}.metagrid.tmp",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("grid_config.json")
    );
    let tmp_path = dir.join(tmp_file_name);
    std::fs::write(&tmp_path, new_json)?;
    std::fs::rename(&tmp_path, path)?;
    Ok(())
}

pub fn write_configs_to(path: &Path, our_grids: &[GridConfig]) -> Result<(), GridError> {
    let existing = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(GridError::Io(e)),
    };
    let new_json = upsert_configs(&existing, our_grids)?;
    backup_and_write(path, &existing, &new_json)
}

pub fn merge_meta_into(
    existing_json: &str,
    meta_cats: &[Category],
    target_name: &str,
) -> Result<Option<String>, GridError> {
    let mut root: serde_json::Value = if existing_json.trim().is_empty() {
        serde_json::json!({ "version": 3, "configs": [] })
    } else {
        serde_json::from_str(existing_json)?
    };

    let obj = root
        .as_object_mut()
        .ok_or_else(|| GridError::Shape("root is not a JSON object".to_string()))?;
    let configs = obj
        .entry("configs")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or_else(|| GridError::Shape("\"configs\" is not an array".to_string()))?;

    let Some(target) = configs
        .iter_mut()
        .find(|c| c.get("config_name").and_then(|v| v.as_str()) == Some(target_name))
    else {
        return Ok(None);
    };

    let cats = target
        .get_mut("categories")
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| GridError::Shape("target config \"categories\" is not an array".to_string()))?;

    cats.retain(|c| {
        let name = c.get("category_name").and_then(|v| v.as_str()).unwrap_or("");
        !name.starts_with("META ")
    });

    let wmeta = meta_cats
        .iter()
        .map(|c| c.x_position + c.width)
        .fold(0.0_f64, f64::max);
    let user_left_target = wmeta + MERGE_GAP;
    let user_min_x = cats
        .iter()
        .filter_map(|c| c.get("x_position").and_then(|v| v.as_f64()))
        .fold(f64::INFINITY, f64::min);
    if user_min_x.is_finite() {
        let delta = user_left_target - user_min_x;
        if delta != 0.0 {
            for c in cats.iter_mut() {
                if let Some(x) = c.get("x_position").and_then(|v| v.as_f64()) {
                    c["x_position"] = serde_json::json!(x + delta);
                }
            }
        }

        let user_max_x = cats
            .iter()
            .filter_map(|c| {
                let x = c.get("x_position").and_then(|v| v.as_f64())?;
                let w = c.get("width").and_then(|v| v.as_f64())?;
                Some(x + w)
            })
            .fold(0.0_f64, f64::max);

        if user_max_x > MAX_CANVAS_X && user_max_x > user_left_target {
            let avail_w = MAX_CANVAS_X - user_left_target;
            let span_w = user_max_x - user_left_target;
            let scale_x = avail_w / span_w;
            for c in cats.iter_mut() {
                if let Some(x) = c.get("x_position").and_then(|v| v.as_f64()) {
                    let rel_x = x - user_left_target;
                    c["x_position"] = serde_json::json!(user_left_target + rel_x * scale_x);
                }
                if let Some(w) = c.get("width").and_then(|v| v.as_f64()) {
                    c["width"] = serde_json::json!(w * scale_x);
                }
            }
        }

        let user_min_y = cats
            .iter()
            .filter_map(|c| c.get("y_position").and_then(|v| v.as_f64()))
            .fold(0.0_f64, f64::min);
        let user_max_y = cats
            .iter()
            .filter_map(|c| {
                let y = c.get("y_position").and_then(|v| v.as_f64())?;
                let h = c.get("height").and_then(|v| v.as_f64())?;
                Some(y + h)
            })
            .fold(0.0_f64, f64::max);

        if user_max_y > MAX_CANVAS_Y && user_max_y > user_min_y {
            let avail_h = MAX_CANVAS_Y - user_min_y;
            let span_h = user_max_y - user_min_y;
            let scale_y = avail_h / span_h;
            for c in cats.iter_mut() {
                if let Some(y) = c.get("y_position").and_then(|v| v.as_f64()) {
                    let rel_y = y - user_min_y;
                    c["y_position"] = serde_json::json!(user_min_y + rel_y * scale_y);
                }
                if let Some(h) = c.get("height").and_then(|v| v.as_f64()) {
                    c["height"] = serde_json::json!(h * scale_y);
                }
            }
        }
    }

    let meta_values: Vec<serde_json::Value> = meta_cats
        .iter()
        .map(serde_json::to_value)
        .collect::<Result<_, _>>()?;
    cats.splice(0..0, meta_values);

    Ok(Some(serde_json::to_string_pretty(&root)?))
}

pub fn write_merge_to(
    path: &Path,
    meta_cats: &[Category],
    target_name: &str,
) -> Result<(), GridError> {
    let existing = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(GridError::Io(e)),
    };
    match merge_meta_into(&existing, meta_cats, target_name)? {
        Some(new_json) => backup_and_write(path, &existing, &new_json),
        None => Ok(()),
    }
}

#[allow(dead_code)]
pub fn write_to(path: &Path, our: &GridConfig) -> Result<(), GridError> {
    write_configs_to(path, std::slice::from_ref(our))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::GridConfig;

    #[test]
    fn upsert_preserves_foreign_configs() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[{"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":10.0,"height":10.0,"hero_ids":[1,2]}]}
        ]}"#;
        let ours = GridConfig::sample_metagrid();
        let out = upsert_config(existing, &ours).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let names: Vec<_> = v["configs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["config_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names.contains(&"Main Layout".to_string()));
        assert!(names.contains(&"MetaGrid".to_string()));

        let out2 = upsert_config(&out, &ours).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&out2).unwrap();
        let count = v2["configs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|c| c["config_name"] == "MetaGrid")
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn write_to_preserves_foreign_config_creates_backup_and_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("grid_config.json");
        let original = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[{"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":10.0,"height":10.0,"hero_ids":[1,2]}]}
        ]}"#;
        std::fs::write(&path, original).unwrap();

        let ours = GridConfig::sample_metagrid();

        write_to(&path, &ours).unwrap();

        let bak_path = path.with_extension("json.metagrid.bak");
        assert!(bak_path.exists());
        let bak_contents = std::fs::read_to_string(&bak_path).unwrap();
        assert_eq!(bak_contents, original);

        let written = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&written).unwrap();
        let names: Vec<_> = v["configs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["config_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names.contains(&"Main Layout".to_string()));
        assert!(names.contains(&"MetaGrid".to_string()));

        write_to(&path, &ours).unwrap();

        let bak_contents2 = std::fs::read_to_string(&bak_path).unwrap();
        assert_eq!(bak_contents2, original);

        let written2 = std::fs::read_to_string(&path).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&written2).unwrap();
        let configs2 = v2["configs"].as_array().unwrap();
        let metagrid_count = configs2.iter().filter(|c| c["config_name"] == "MetaGrid").count();
        assert_eq!(metagrid_count, 1);
        let names2: Vec<_> = configs2
            .iter()
            .map(|c| c["config_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names2.contains(&"Main Layout".to_string()));
    }

    #[test]
    fn upsert_configs_preserves_foreign_and_replaces_our_grids() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[]},
            {"config_name":"My Custom","categories":[]},
            {"config_name":"⚡ Old Grid","categories":[]}
        ]}"#;

        let named_grids: Vec<GridConfig> = ["Carry", "Mid", "Offlane", "Support", "Hard Support"]
            .iter()
            .map(|&name| GridConfig {
                config_name: name.to_string(),
                categories: vec![],
            })
            .collect();

        let out = upsert_configs(existing, &named_grids).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let configs = v["configs"].as_array().unwrap();

        assert_eq!(configs.len(), 7);
        let names: Vec<String> = configs
            .iter()
            .map(|c| c["config_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names.contains(&"Main Layout".to_string()));
        assert!(names.contains(&"My Custom".to_string()));
        assert!(!names.contains(&"⚡ Old Grid".to_string()));
        for name in &["Carry", "Mid", "Offlane", "Support", "Hard Support"] {
            assert!(names.contains(&name.to_string()));
        }

        let out2 = upsert_configs(&out, &named_grids).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&out2).unwrap();
        let configs2 = v2["configs"].as_array().unwrap();
        assert_eq!(configs2.len(), 7);
    }

    fn meta_cats_fixture() -> Vec<Category> {
        use crate::model::{HeroMeta, MetaSnapshot, Position, RoleMeta};
        let roles = Position::all()
            .iter()
            .enumerate()
            .map(|(p_idx, &position)| RoleMeta {
                position,
                role_winrate: 0.5,
                heroes: (0..10)
                    .map(|i| HeroMeta {
                        hero_id: (p_idx as u32 + 1) * 100 + i,
                        name: format!("H{i}"),
                        slug: format!("h{i}"),
                        winrate: 0.5,
                        pickrate: 0.1,
                        matches: 10,
                        d2pt_rating: 0,
                        is_top: i < 3,
                    })
                    .collect(),
            })
            .collect();
        let snap = MetaSnapshot {
            patch: "x".into(),
            fetched_at: "t".into(),
            source: "test".into(),
            roles,
        };
        crate::grid::build_meta_categories(&snap, "named", 10)
    }

    #[test]
    fn merge_injects_and_preserves_user_categories() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[
                {"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":687.0,"height":293.0,"hero_ids":[1,2]}
            ]}
        ]}"#;
        let cats = meta_cats_fixture();
        let out = merge_meta_into(existing, &cats, "Main Layout").unwrap().unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let cfg0 = &v["configs"][0];
        let target_cats = cfg0["categories"].as_array().unwrap();
        assert_eq!(target_cats.len(), 6);
        assert!(target_cats.iter().any(|c| c["category_name"] == "META CARRY"));
        let user = target_cats
            .iter()
            .find(|c| c["category_name"] == "2pos")
            .unwrap();
        assert_eq!(user["x_position"].as_f64().unwrap(), 510.0);
    }

    #[test]
    fn merge_is_idempotent_no_drift() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[
                {"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":687.0,"height":293.0,"hero_ids":[1,2]}
            ]}
        ]}"#;
        let cats = meta_cats_fixture();
        let out1 = merge_meta_into(existing, &cats, "Main Layout").unwrap().unwrap();
        let out2 = merge_meta_into(&out1, &cats, "Main Layout").unwrap().unwrap();

        let v1: serde_json::Value = serde_json::from_str(&out1).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&out2).unwrap();
        let c1 = v1["configs"][0]["categories"].as_array().unwrap();
        let c2 = v2["configs"][0]["categories"].as_array().unwrap();

        assert_eq!(c1.len(), c2.len());
        let meta_count1 = c1.iter().filter(|c| c["category_name"].as_str().unwrap().starts_with("META ")).count();
        let meta_count2 = c2.iter().filter(|c| c["category_name"].as_str().unwrap().starts_with("META ")).count();
        assert_eq!(meta_count1, 5);
        assert_eq!(meta_count2, 5);

        let user1 = c1.iter().find(|c| c["category_name"] == "2pos").unwrap();
        let user2 = c2.iter().find(|c| c["category_name"] == "2pos").unwrap();
        assert_eq!(user1["x_position"].as_f64().unwrap(), 510.0);
        assert_eq!(user2["x_position"].as_f64().unwrap(), 510.0);
    }

    #[test]
    fn merge_scales_oversized_user_categories_within_canvas_bounds() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[
                {"category_name":"Gigantic","x_position":0.0,"y_position":0.0,"width":1800.0,"height":1000.0,"hero_ids":[34]}
            ]}
        ]}"#;
        let cats = meta_cats_fixture();
        let out = merge_meta_into(existing, &cats, "Main Layout").unwrap().unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let target_cats = v["configs"][0]["categories"].as_array().unwrap();
        let user = target_cats.iter().find(|c| c["category_name"] == "Gigantic").unwrap();
        let x = user["x_position"].as_f64().unwrap();
        let w = user["width"].as_f64().unwrap();
        let y = user["y_position"].as_f64().unwrap();
        let h = user["height"].as_f64().unwrap();
        assert_eq!(x, 510.0);
        assert!(x + w <= 1610.0 + 1e-6);
        assert!(y + h <= 820.0 + 1e-6);
    }

    #[test]
    fn merge_target_not_found_is_noop() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Other Layout","categories":[]}
        ]}"#;
        let cats = meta_cats_fixture();
        let res = merge_meta_into(existing, &cats, "Main Layout").unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn merge_preserves_foreign_configs() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[{"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":10.0,"height":10.0,"hero_ids":[1]}]},
            {"config_name":"Untouched","categories":[{"category_name":"mine","x_position":12.0,"y_position":34.0,"width":50.0,"height":60.0,"hero_ids":[99]}]}
        ]}"#;
        let cats = meta_cats_fixture();
        let out = merge_meta_into(existing, &cats, "Main Layout").unwrap().unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let untouched = &v["configs"][1];
        assert_eq!(untouched["config_name"], "Untouched");
        assert_eq!(untouched["categories"][0]["category_name"], "mine");
        assert_eq!(untouched["categories"][0]["x_position"].as_f64().unwrap(), 12.0);
    }

    #[test]
    fn merge_on_empty_target_categories() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[]}
        ]}"#;
        let cats = meta_cats_fixture();
        let out = merge_meta_into(existing, &cats, "Main Layout").unwrap().unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let target_cats = v["configs"][0]["categories"].as_array().unwrap();
        assert_eq!(target_cats.len(), 5);
        assert!(target_cats.iter().all(|c| c["category_name"].as_str().unwrap().starts_with("META ")));
    }
}
