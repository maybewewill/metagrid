use crate::grid::GridConfig;
use std::io;
use std::path::Path;
use thiserror::Error;


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
    n == "MetaGrid" || n.starts_with("MetaGrid") || n.starts_with('⚡') || n.starts_with("⚡ ")
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

pub fn write_configs_to(path: &Path, our_grids: &[GridConfig]) -> Result<(), GridError> {
    let existing = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(GridError::Io(e)),
    };

    let new_json = upsert_configs(&existing, our_grids)?;

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

    std::fs::write(&tmp_path, &new_json)?;
    std::fs::rename(&tmp_path, path)?;

    Ok(())
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
    fn upsert_configs_preserves_user_configs_named_after_roles() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[]},
            {"config_name":"Carry","categories":[]},
            {"config_name":"⚡ Old Carry","categories":[]}
        ]}"#;

        let named_grids: Vec<GridConfig> = ["⚡ Carry", "⚡ Mid", "⚡ Offlane", "⚡ Support", "⚡ Hard Support"]
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
        assert!(names.contains(&"Carry".to_string()));
        assert!(!names.contains(&"⚡ Old Carry".to_string()));
        for name in &["⚡ Carry", "⚡ Mid", "⚡ Offlane", "⚡ Support", "⚡ Hard Support"] {
            assert!(names.contains(&name.to_string()));
        }

        let out2 = upsert_configs(&out, &named_grids).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&out2).unwrap();
        let configs2 = v2["configs"].as_array().unwrap();
        assert_eq!(configs2.len(), 7);
    }
}
