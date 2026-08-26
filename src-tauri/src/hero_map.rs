use std::collections::HashMap;

/// Bundled Dota hero id/name/slug lookup table.
pub struct HeroMap {
    /// normalized(name-or-slug) -> hero id
    by_key: HashMap<String, u32>,
    /// hero id -> slug (hero internal name without the `npc_dota_hero_` prefix)
    slug_by_id: HashMap<u32, String>,
}

/// Lowercases and strips spaces, `-`, and `'` so that "Anti-Mage", "antimage",
/// and "anti mage" all normalize to the same key.
fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, ' ' | '-' | '\''))
        .flat_map(|c| c.to_lowercase())
        .collect()
}

impl HeroMap {
    pub fn bundled() -> HeroMap {
        let raw = include_str!("../resources/heroes.json");
        let value: serde_json::Value =
            serde_json::from_str(raw).expect("bundled heroes.json must be valid JSON");

        let mut by_key = HashMap::new();
        let mut slug_by_id = HashMap::new();

        if let serde_json::Value::Object(heroes) = value {
            for (_, hero) in heroes {
                let id = hero.get("id").and_then(|v| v.as_u64()).map(|v| v as u32);
                let name = hero.get("name").and_then(|v| v.as_str());
                let localized_name = hero.get("localized_name").and_then(|v| v.as_str());

                let (Some(id), Some(name)) = (id, name) else {
                    continue;
                };

                let slug = name.strip_prefix("npc_dota_hero_").unwrap_or(name).to_string();

                by_key.insert(normalize(&slug), id);
                if let Some(localized_name) = localized_name {
                    by_key.insert(normalize(localized_name), id);
                }
                slug_by_id.insert(id, slug);
            }
        }

        HeroMap { by_key, slug_by_id }
    }

    pub fn id_for(&self, name_or_slug: &str) -> Option<u32> {
        self.by_key.get(&normalize(name_or_slug)).copied()
    }

    pub fn slug_for(&self, id: u32) -> Option<&str> {
        self.slug_by_id.get(&id).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolves_names_and_slugs() {
        let m = HeroMap::bundled();
        let am = m.id_for("Anti-Mage").unwrap();
        assert_eq!(m.id_for("antimage"), Some(am));
        assert_eq!(m.id_for("anti mage"), Some(am));
        assert_eq!(m.slug_for(am), Some("antimage"));
        assert!(m.id_for("Not A Hero").is_none());
    }
}
