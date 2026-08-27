use std::collections::HashMap;

pub struct HeroMap {
    slug_by_id: HashMap<u32, String>,
    name_by_id: HashMap<u32, String>,
    id_by_query: HashMap<String, u32>,
}

fn normalize_query(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, ' ' | '-' | '\'' | '_'))
        .flat_map(|c| c.to_lowercase())
        .collect()
}

impl HeroMap {
    pub fn bundled() -> HeroMap {
        let raw = include_str!("../resources/heroes.json");
        let value: serde_json::Value =
            serde_json::from_str(raw).expect("bundled heroes.json must be valid JSON");

        let mut slug_by_id = HashMap::new();
        let mut name_by_id = HashMap::new();
        let mut id_by_query = HashMap::new();

        if let serde_json::Value::Object(heroes) = value {
            for (_, hero) in heroes {
                let id = hero.get("id").and_then(|v| v.as_u64()).map(|v| v as u32);
                let name = hero.get("name").and_then(|v| v.as_str());
                let localized_name = hero.get("localized_name").and_then(|v| v.as_str());

                let (Some(id), Some(name)) = (id, name) else {
                    continue;
                };

                let slug = name.strip_prefix("npc_dota_hero_").unwrap_or(name).to_string();
                slug_by_id.insert(id, slug.clone());
                if let Some(loc) = localized_name {
                    name_by_id.insert(id, loc.to_string());
                    id_by_query.insert(loc.to_lowercase(), id);
                    id_by_query.insert(normalize_query(loc), id);
                } else {
                    name_by_id.insert(id, slug.clone());
                }
                id_by_query.insert(slug.to_lowercase(), id);
                id_by_query.insert(normalize_query(&slug), id);
            }
        }

        let aliases: &[(&str, u32)] = &[
            ("outworld destroyer", 76),
            ("outworld devourer", 76),
            ("od", 76),
            ("natures prophet", 53),
            ("nature's prophet", 53),
            ("furion", 53),
            ("necrophos", 36),
            ("necrolyte", 36),
            ("windranger", 21),
            ("windrunner", 21),
            ("shadow fiend", 11),
            ("nevermore", 11),
            ("zeus", 22),
            ("zuus", 22),
            ("timbersaw", 98),
            ("shredder", 98),
            ("treant protector", 83),
            ("treant", 83),
            ("centaur warrunner", 96),
            ("centaur", 96),
            ("magnus", 97),
            ("magnataur", 97),
            ("wraith king", 42),
            ("skeleton king", 42),
            ("queen of pain", 39),
            ("queenofpain", 39),
            ("anti-mage", 1),
            ("antimage", 1),
            ("shadow shaman", 27),
            ("rhasta", 27),
            ("clockwerk", 51),
            ("rattletrap", 51),
            ("lifestealer", 54),
            ("underlord", 108),
            ("abyssal underlord", 108),
            ("vengeful spirit", 20),
            ("vengefulspirit", 20),
            ("io", 91),
            ("wisp", 91),
            ("doom", 69),
            ("doom bringer", 69),
        ];
        for (alias, id) in aliases {
            id_by_query.insert(alias.to_lowercase(), *id);
            id_by_query.insert(normalize_query(alias), *id);
        }

        HeroMap {
            slug_by_id,
            name_by_id,
            id_by_query,
        }
    }

    pub fn slug_for(&self, id: u32) -> Option<&str> {
        self.slug_by_id.get(&id).map(|s| s.as_str())
    }

    pub fn name_for(&self, id: u32) -> Option<&str> {
        self.name_by_id.get(&id).map(|s| s.as_str())
    }

    pub fn id_for(&self, query: &str) -> Option<u32> {
        self.id_by_query
            .get(&query.to_lowercase())
            .copied()
            .or_else(|| self.id_by_query.get(&normalize_query(query)).copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolves_hero_slugs() {
        let m = HeroMap::bundled();
        assert_eq!(m.slug_for(1), Some("antimage"));
        assert_eq!(m.name_for(1), Some("Anti-Mage"));
        assert_eq!(m.slug_for(999999), None);
        assert_eq!(m.name_for(999999), None);
        assert_eq!(m.id_for("Shadow Fiend"), Some(11));
        assert_eq!(m.id_for("nevermore"), Some(11));
        assert_eq!(m.id_for("Anti-Mage"), Some(1));
    }
}
