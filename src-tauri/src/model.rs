use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Position {
    Pos1,
    Pos2,
    Pos3,
    Pos4,
    Pos5,
}

impl Position {
    pub fn config_name(&self, role_labels: &str) -> &'static str {
        if role_labels == "pos" {
            match self {
                Position::Pos1 => "POS 1",
                Position::Pos2 => "POS 2",
                Position::Pos3 => "POS 3",
                Position::Pos4 => "POS 4",
                Position::Pos5 => "POS 5",
            }
        } else {
            match self {
                Position::Pos1 => "Carry",
                Position::Pos2 => "Mid",
                Position::Pos3 => "Offlane",
                Position::Pos4 => "Support",
                Position::Pos5 => "Hard Support",
            }
        }
    }

    pub fn role_upper(&self, role_labels: &str) -> &'static str {
        if role_labels == "pos" {
            match self {
                Position::Pos1 => "POS 1",
                Position::Pos2 => "POS 2",
                Position::Pos3 => "POS 3",
                Position::Pos4 => "POS 4",
                Position::Pos5 => "POS 5",
            }
        } else {
            match self {
                Position::Pos1 => "CARRY",
                Position::Pos2 => "MID",
                Position::Pos3 => "OFFLANE",
                Position::Pos4 => "SUPPORT",
                Position::Pos5 => "HARD SUPPORT",
            }
        }
    }

    pub fn all() -> [Position; 5] {
        [
            Position::Pos1,
            Position::Pos2,
            Position::Pos3,
            Position::Pos4,
            Position::Pos5,
        ]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SortMetric {
    Pickrate,
    Winrate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeroMeta {
    pub hero_id: u32,
    pub name: String,
    pub slug: String,
    pub winrate: f32,
    pub pickrate: f32,
    pub matches: u32,
    #[serde(default)]
    pub d2pt_rating: u32,
    #[serde(default)]
    pub is_top: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMeta {
    pub position: Position,
    pub role_winrate: f32,
    pub heroes: Vec<HeroMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaSnapshot {
    pub patch: String,
    pub fetched_at: String,
    pub source: String,
    pub roles: Vec<RoleMeta>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snapshot_serde_roundtrips_and_labels() {
        assert_eq!(Position::all().len(), 5);
        assert_eq!(Position::Pos1.config_name("named"), "Carry");
        assert_eq!(Position::Pos1.config_name("pos"), "POS 1");
        let snap = MetaSnapshot {
            patch: "7.41e".into(), fetched_at: "2026-08-26T10:00:00Z".into(),
            source: "d2pt".into(),
            roles: vec![RoleMeta { position: Position::Pos1, role_winrate: 0.52,
                heroes: vec![HeroMeta{hero_id:1,name:"Anti-Mage".into(),slug:"antimage".into(),winrate:0.53,pickrate:0.12,matches:900,d2pt_rating:3000,is_top:false}] }],
        };
        let json = serde_json::to_string(&snap).unwrap();
        let back: MetaSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(snap, back);
    }
}
