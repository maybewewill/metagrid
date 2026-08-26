use crate::grid::GridConfig;
use std::io;
use std::path::Path;
use thiserror::Error;

/// The one config name MetaGrid is allowed to create or replace in a user's
/// grid_config.json. Every other `config_name` present in the file is
/// foreign data and must be preserved byte-for-byte.
pub const METAGRID_NAME: &str = "MetaGrid";

#[derive(Debug, Error)]
pub enum GridError {
    /// The existing file was non-empty but failed to parse as JSON. We
    /// deliberately do NOT fall back to an empty config here: a file we
    /// failed to read might still be a valid (if unusual) config, and
    /// silently replacing it would destroy user data. Never trade a parse
    /// failure for data loss.
    #[error("failed to parse existing grid config JSON: {0}")]
    Parse(#[from] serde_json::Error),

    /// The existing JSON parsed but doesn't have the shape we expect
    /// (root not an object, or `configs` present but not an array). Same
    /// rationale as `Parse`: refuse rather than guess and clobber.
    #[error("existing grid config has unexpected shape: {0}")]
    Shape(String),

    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

/// Merge `our` (the MetaGrid config) into `existing_json`, preserving every
/// other config and every field we don't model.
///
/// Starting-from-empty rule: if `existing_json` is empty or only whitespace,
/// start from `{"version":3,"configs":[]}`. If it is non-empty but fails to
/// parse, this returns `Err` and does NOT synthesize an empty document —
/// see `GridError::Parse` for why.
pub fn upsert_config(existing_json: &str, our: &GridConfig) -> Result<String, GridError> {
    let mut root: serde_json::Value = if existing_json.trim().is_empty() {
        serde_json::json!({"version": 3, "configs": []})
    } else {
        serde_json::from_str(existing_json)?
    };

    let our_value = serde_json::to_value(our)?;

    let obj = root
        .as_object_mut()
        .ok_or_else(|| GridError::Shape("root is not a JSON object".to_string()))?;

    let configs_value = obj
        .entry("configs")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    let configs = configs_value
        .as_array_mut()
        .ok_or_else(|| GridError::Shape("\"configs\" is not an array".to_string()))?;

    let mut replaced = false;
    for c in configs.iter_mut() {
        if c.get("config_name").and_then(|v| v.as_str()) == Some(METAGRID_NAME) {
            *c = our_value.clone();
            replaced = true;
            break;
        }
    }
    if !replaced {
        configs.push(our_value);
    }

    Ok(serde_json::to_string_pretty(&root)?)
}

/// Write `our` into the grid config file at `path`, preserving every other
/// config already there.
///
/// Safety properties:
/// - `upsert_config` runs first, against the file's current contents. If it
///   errors, `path` is never touched.
/// - Before the first modification, if `path.with_extension("json.metagrid.bak")`
///   doesn't already exist, the original file bytes are copied there (only
///   when the original file exists and is non-empty).
/// - The new contents are written to a temp file in the same directory, then
///   `std::fs::rename`d into place — atomic on Windows (MOVEFILE_REPLACE_EXISTING)
///   and POSIX, so `path` never observes a truncated/partial write.
pub fn write_to(path: &Path, our: &GridConfig) -> Result<(), GridError> {
    let existing = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(GridError::Io(e)),
    };

    // Compute the new JSON FIRST — if this fails, `path` is never touched.
    let new_json = upsert_config(&existing, our)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::GridConfig;

    #[test]
    fn upsert_preserves_foreign_configs() {
        let existing = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[{"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":10.0,"height":10.0,"hero_ids":[1,2]}]}
        ]}"#;
        let ours = GridConfig::sample_metagrid(); // config_name == "MetaGrid"
        let out = upsert_config(existing, &ours).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let names: Vec<_> = v["configs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["config_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names.contains(&"Main Layout".to_string())); // foreign kept
        assert!(names.contains(&"MetaGrid".to_string())); // ours added
                                                            // idempotent: second upsert doesn't duplicate
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

    /// Proves the on-disk safety properties of `write_to` end to end using a
    /// real temp file: foreign configs survive, a `.bak` of the original
    /// bytes is created exactly once, and a second write is idempotent
    /// (no duplicate MetaGrid entry, `.bak` untouched by the second call).
    #[test]
    fn write_to_preserves_foreign_config_creates_backup_and_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("grid_config.json");
        let original = r#"{"version":3,"configs":[
            {"config_name":"Main Layout","categories":[{"category_name":"2pos","x_position":0.0,"y_position":0.0,"width":10.0,"height":10.0,"hero_ids":[1,2]}]}
        ]}"#;
        std::fs::write(&path, original).unwrap();

        let ours = GridConfig::sample_metagrid();

        // First write.
        write_to(&path, &ours).unwrap();

        let bak_path = path.with_extension("json.metagrid.bak");
        assert!(bak_path.exists(), ".bak must be created on first write");
        let bak_contents = std::fs::read_to_string(&bak_path).unwrap();
        assert_eq!(bak_contents, original, ".bak must hold the original bytes");

        let written = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&written).unwrap();
        let names: Vec<_> = v["configs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["config_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names.contains(&"Main Layout".to_string()), "foreign config preserved");
        assert!(names.contains(&"MetaGrid".to_string()), "our config written");

        // Second write: idempotent.
        write_to(&path, &ours).unwrap();

        let bak_contents2 = std::fs::read_to_string(&bak_path).unwrap();
        assert_eq!(bak_contents2, original, ".bak must remain the ORIGINAL bytes, not re-copied");

        let written2 = std::fs::read_to_string(&path).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&written2).unwrap();
        let configs2 = v2["configs"].as_array().unwrap();
        let metagrid_count = configs2.iter().filter(|c| c["config_name"] == "MetaGrid").count();
        assert_eq!(metagrid_count, 1, "no duplicate MetaGrid on second write");
        let names2: Vec<_> = configs2
            .iter()
            .map(|c| c["config_name"].as_str().unwrap().to_string())
            .collect();
        assert!(names2.contains(&"Main Layout".to_string()), "foreign config still preserved");
    }
}
