use std::collections::HashMap;

pub struct HeroMap {
    slug_by_id: HashMap<u32, String>,
}

impl HeroMap {
    pub fn bundled() -> HeroMap {
        let raw = include_str!("../resources/heroes.json");
        let value: serde_json::Value =
            serde_json::from_str(raw).expect("bundled heroes.json must be valid JSON");

        let mut slug_by_id = HashMap::new();

        if let serde_json::Value::Object(heroes) = value {
            for (_, hero) in heroes {
                let id = hero.get("id").and_then(|v| v.as_u64()).map(|v| v as u32);
                let name = hero.get("name").and_then(|v| v.as_str());

                let (Some(id), Some(name)) = (id, name) else {
                    continue;
                };

                let slug = name.strip_prefix("npc_dota_hero_").unwrap_or(name).to_string();
                slug_by_id.insert(id, slug);
            }
        }

        HeroMap { slug_by_id }
    }

    pub fn slug_for(&self, id: u32) -> Option<&str> {
        self.slug_by_id.get(&id).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolves_hero_slugs() {
        let m = HeroMap::bundled();
        assert_eq!(m.slug_for(1), Some("antimage"));
        assert_eq!(m.slug_for(999999), None);
    }
}
