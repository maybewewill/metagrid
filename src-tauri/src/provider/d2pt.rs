use regex::Regex;

use crate::hero_map::HeroMap;
use crate::model::{HeroMeta, MetaSnapshot, Position, RoleMeta, Tournament};
use crate::provider::{MetaProvider, ProviderError};

const D2PT_URL: &str = "https://dota2protracker.com/";
const D2PT_META_URL: &str = "https://dota2protracker.com/meta?mmr=7000&period=8&position=pos%2B";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const ACCEPT: &str =
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9";

pub struct D2ptProvider;

impl D2ptProvider {
    pub fn new() -> Self {
        D2ptProvider
    }
}

impl Default for D2ptProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl D2ptProvider {
    async fn fetch_http(&self, url: &str) -> Result<String, ProviderError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|_| ProviderError::Blocked)?;

        let response = client
            .get(url)
            .header("User-Agent", USER_AGENT)
            .header("Accept", ACCEPT)
            .header("Accept-Language", ACCEPT_LANGUAGE)
            .send()
            .await
            .map_err(|e| {
                tracing::warn!(error = %e, "d2pt: http request failed");
                ProviderError::Blocked
            })?;

        if !response.status().is_success() {
            tracing::warn!(status = %response.status(), "d2pt: http status non-success");
            return Err(ProviderError::Blocked);
        }

        let body = response.text().await.map_err(|e| {
            tracing::warn!(error = %e, "d2pt: failed reading response body");
            ProviderError::Blocked
        })?;

        if body.is_empty() || body.contains("Just a moment") {
            tracing::warn!("d2pt: response looked blocked (empty or Cloudflare challenge)");
            return Err(ProviderError::Blocked);
        }

        Ok(body)
    }

    pub async fn fetch_raw(&self) -> Result<String, ProviderError> {
        let body = self.fetch_http(D2PT_URL).await?;
        if !body.contains("roles:[") {
            tracing::warn!("d2pt: homepage response missing 'roles:[' marker");
            return Err(ProviderError::Blocked);
        }
        Ok(body)
    }

    pub async fn fetch_pos_raw(&self, pos: u8, meta_source: &str, league_id: i64) -> Result<String, ProviderError> {
        let url = if meta_source == "tournaments" {
            format!("https://dota2protracker.com/meta?position=pos%2B{pos}&league_id={league_id}")
        } else {
            format!("{D2PT_META_URL}{pos}")
        };
        self.fetch_http(&url).await
    }

    pub async fn fetch_meta_all(&self, map: &HeroMap, top_n: usize, meta_source: &str, league_id: i64) -> Result<MetaSnapshot, ProviderError> {
        let (p1, p2, p3, p4, p5) = tokio::join!(
            self.fetch_pos_raw(1, meta_source, league_id),
            self.fetch_pos_raw(2, meta_source, league_id),
            self.fetch_pos_raw(3, meta_source, league_id),
            self.fetch_pos_raw(4, meta_source, league_id),
            self.fetch_pos_raw(5, meta_source, league_id),
        );

        let bodies = [
            (Position::Pos1, p1?),
            (Position::Pos2, p2?),
            (Position::Pos3, p3?),
            (Position::Pos4, p4?),
            (Position::Pos5, p5?),
        ];

        let mut roles = Vec::with_capacity(5);
        let mut patch = "unknown".to_string();

        for (pos, body) in &bodies {
            if patch == "unknown" {
                patch = extract_patch(body);
            }
            let role_meta = parse_pos_meta(body, *pos, map, top_n)?;
            roles.push(role_meta);
        }

        let source = if meta_source == "tournaments" {
            let live_tourneys = parse_tournaments_from_html(&bodies[0].1);
            let tname = live_tourneys
                .iter()
                .find(|t| t.id == league_id)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| tournament_name_for_id(league_id));
            format!("d2pt (Tournament: {tname})")
        } else {
            "d2pt".to_string()
        };

        Ok(MetaSnapshot {
            patch,
            fetched_at: chrono::Utc::now().to_rfc3339(),
            source,
            roles,
        })
    }

    pub async fn fetch_tournaments_live(&self) -> Result<Vec<Tournament>, ProviderError> {
        let body = self.fetch_http("https://dota2protracker.com/meta?position=pos%2B1&league_id=-1").await?;
        let parsed = parse_tournaments_from_html(&body);
        if parsed.len() <= 1 {
            let raw = include_str!("../../resources/tournaments.json");
            if let Ok(fallback) = serde_json::from_str::<Vec<Tournament>>(raw) {
                return Ok(fallback);
            }
        }
        Ok(parsed)
    }
}

pub fn parse_tournaments_from_html(html: &str) -> Vec<Tournament> {
    let re = regex::Regex::new(r#"leagueid:(\d+),name:"([^"]+)"(?:,tier:"[^"]*")?(?:,match_count:(\d+))?"#).unwrap();
    let mut list = Vec::new();
    list.push(Tournament {
        id: -1,
        name: "All Tournaments".to_string(),
        match_count: 0,
    });

    let mut seen = std::collections::HashSet::new();
    seen.insert(-1i64);

    for cap in re.captures_iter(html) {
        if let (Some(id_m), Some(name_m)) = (cap.get(1), cap.get(2)) {
            if let Ok(id) = id_m.as_str().parse::<i64>() {
                if !seen.contains(&id) {
                    seen.insert(id);
                    let count = cap.get(3).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(0);
                    list.push(Tournament {
                        id,
                        name: name_m.as_str().to_string(),
                        match_count: count,
                    });
                }
            }
        }
    }

    list
}

fn tournament_name_for_id(league_id: i64) -> String {
    if league_id == -1 {
        return "All Tournaments".to_string();
    }
    let raw = include_str!("../../resources/tournaments.json");
    if let Ok(serde_json::Value::Array(arr)) = serde_json::from_str::<serde_json::Value>(raw) {
        for item in arr {
            if item.get("id").and_then(|v| v.as_i64()) == Some(league_id) {
                if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                    return name.to_string();
                }
            }
        }
    }
    format!("Tournament {league_id}")
}

#[async_trait::async_trait]
impl MetaProvider for D2ptProvider {
    async fn fetch(&self, map: &HeroMap, top_n: usize, meta_source: &str, league_id: i64) -> Result<MetaSnapshot, ProviderError> {
        match self.fetch_meta_all(map, top_n, meta_source, league_id).await {
            Ok(snap) => Ok(snap),
            Err(e) => {
                if meta_source == "tournaments" {
                    return Err(e);
                }
                tracing::warn!(error = %e, "d2pt: /meta multi-fetch failed, falling back to homepage");
                let raw = self.fetch_raw().await?;
                parse_meta(&raw, map, top_n)
            }
        }
    }
}

struct RawHero {
    hero_id: u32,
    hero_name: String,
    npc: Option<String>,
    period: String,
    matches: u32,
    win_rate: f32,
    d2pt_rating: u32,
}

fn extract_roles_array(html: &str) -> Result<&str, ProviderError> {
    let start = html
        .find("roles:[")
        .ok_or_else(|| ProviderError::Parse("missing 'roles:[' marker in response".into()))?;
    let slice_start = start + "roles:".len();
    let rest = &html[slice_start..];

    let mut depth = 0i32;
    let mut in_str = false;
    let mut str_quote = '"';
    let mut prev = '\0';
    let mut end_byte = None;

    for (idx, ch) in rest.char_indices() {
        if in_str {
            if ch == str_quote && prev != '\\' {
                in_str = false;
            }
        } else if ch == '"' || ch == '\'' {
            in_str = true;
            str_quote = ch;
        } else if ch == '[' {
            depth += 1;
        } else if ch == ']' {
            depth -= 1;
            if depth == 0 {
                end_byte = Some(idx + ch.len_utf8());
                break;
            }
        }
        prev = ch;
    }

    let end_byte = end_byte
        .ok_or_else(|| ProviderError::Parse("unmatched bracket in roles block".into()))?;
    Ok(&rest[..end_byte])
}

fn parse_win_rate(raw: &str) -> Option<f32> {
    let normalized = if let Some(rest) = raw.strip_prefix('.') {
        format!("0.{rest}")
    } else {
        raw.to_string()
    };
    normalized.parse::<f32>().ok()
}

fn derive_slug(hero_name: &str) -> String {
    hero_name
        .chars()
        .filter(|c| !matches!(c, ' ' | '-' | '\''))
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn position_from_d2pt(raw: &str) -> Option<Position> {
    match raw {
        "pos 1" => Some(Position::Pos1),
        "pos 2" => Some(Position::Pos2),
        "pos 3" => Some(Position::Pos3),
        "pos 4" => Some(Position::Pos4),
        "pos 5" => Some(Position::Pos5),
        _ => None,
    }
}

fn extract_patch(html: &str) -> String {
    let markers = [
        r#"href="/patches/([0-9]+\.[0-9]+[a-z]?)""#,
        r#"/patches/([0-9]+\.[0-9]+[a-z]?)"#,
        r#"Current Patch</span>\s*<span[^>]*>([0-9]+\.[0-9]+[a-z]?)</span>"#,
        r#"Patch\s+([0-9]+\.[0-9]+[a-z]?)"#,
        r#"patchVersion:\s*"([^"]+)""#,
        r#"patch:\s*\{\s*version:\s*"([^"]+)""#,
        r#"meta_explorer:\s*\{\s*version:\s*"([^"]+)""#,
        r#"version:\s*"([^"]+)""#,
    ];
    for pattern in markers {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(html) {
                return caps[1].to_string();
            }
        }
    }
    "7.41e".to_string()
}

pub fn parse_pos_meta(
    raw: &str,
    target_pos: Position,
    map: &HeroMap,
    _top_n: usize,
) -> Result<RoleMeta, ProviderError> {
    let patch = extract_patch(raw);

    let mut top_heroes: Vec<HeroMeta> = Vec::new();
    let mut seen_hero_names = std::collections::HashSet::new();

    if let Some(top_idx) = raw.find("Top Heroes") {
        let slice_len = std::cmp::min(6000, raw.len() - top_idx);
        let top_slice = &raw[top_idx..top_idx + slice_len];
        if let Ok(card_re) = Regex::new(
            r#"<a\s+href="/hero/([^"]+)"[^>]*>[\s\S]*?<span[^>]*class="[^"]*truncate"[^>]*>([^<]+)</span>[\s\S]*?<span[^>]*class="text-yellow-300[^"]*"[^>]*>(\d+)</span>[\s\S]*?<span[^>]*class="[^"]*"[^>]*>([0-9.]+)%</span>[\s\S]*?<span[^>]*class="d2pt-rating[^"]*"[^>]*>(\d+)</span>"#,
        ) {
            for cap in card_re.captures_iter(top_slice) {
                let hero_slug = urlencoding::decode(&cap[1]).unwrap_or_default().to_string();
                let hero_name = cap[2].to_string();
                let matches = cap[3].parse::<u32>().unwrap_or(0);
                let winrate = cap[4].parse::<f32>().map(|w| w / 100.0).unwrap_or(0.0);
                let d2pt_rating = cap[5].parse::<u32>().unwrap_or(0);

                let hero_id = map
                    .id_for(&hero_slug)
                    .or_else(|| map.id_for(&hero_name))
                    .unwrap_or(0);

                let display_name = if hero_name.ends_with("...") || hero_name.ends_with('…') {
                    hero_slug.clone()
                } else {
                    hero_name
                };

                if hero_id != 0 && seen_hero_names.insert(display_name.to_lowercase()) {
                    let slug = map
                        .slug_for(hero_id)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| derive_slug(&display_name));

                    top_heroes.push(HeroMeta {
                        hero_id,
                        name: display_name,
                        slug,
                        winrate,
                        pickrate: 0.0,
                        matches,
                        d2pt_rating,
                        is_top: true,
                    });
                    if top_heroes.len() >= 7 {
                        break;
                    }
                }
            }
        }
    }

    let hero_re = Regex::new(
        r#"\{hero_id:(\d+),hero_name:"([^"]+)",npc:"([^"]+)",position:"([^"]+)"[\s\S]*?(?:period:"([^"]+)"[\s\S]*?)?matches:(\d+),wins:(\d+),win_rate:(\.?\d+(?:\.\d+)?)[\s\S]*?d2pt_rating:(\d+)"#,
    )
    .map_err(|e| ProviderError::Parse(format!("bad /meta hero regex: {e}")))?;

    let mut by_hero: std::collections::HashMap<u32, RawHero> = std::collections::HashMap::new();
    let expected_pos_str = match target_pos {
        Position::Pos1 => "pos 1",
        Position::Pos2 => "pos 2",
        Position::Pos3 => "pos 3",
        Position::Pos4 => "pos 4",
        Position::Pos5 => "pos 5",
    };

    for cap in hero_re.captures_iter(raw) {
        let pos_str = &cap[4];
        if pos_str != expected_pos_str {
            continue;
        }

        let Ok(hero_id) = cap[1].parse::<u32>() else { continue; };
        if hero_id == 0 { continue; }
        let hero_name = cap[2].to_string();
        let npc = Some(cap[3].to_string());
        let period = cap.get(5).map(|m| m.as_str().to_string()).unwrap_or_default();
        let Ok(matches) = cap[6].parse::<u32>() else { continue; };
        let Some(win_rate) = parse_win_rate(&cap[8]) else { continue; };
        let d2pt_rating = cap.get(9).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(0);

        let candidate = RawHero {
            hero_id,
            hero_name,
            npc,
            period: period.clone(),
            matches,
            win_rate,
            d2pt_rating,
        };

        if let Some(existing) = by_hero.get_mut(&hero_id) {
            if period == patch || (existing.period != patch && matches > existing.matches) {
                *existing = candidate;
            }
        } else {
            by_hero.insert(hero_id, candidate);
        }
    }

    let mut raw_heroes: Vec<RawHero> = by_hero.into_values().filter(|h| h.matches > 0).collect();
    raw_heroes.sort_by(|a, b| {
        if b.d2pt_rating != a.d2pt_rating {
            b.d2pt_rating.cmp(&a.d2pt_rating)
        } else if b.matches != a.matches {
            b.matches.cmp(&a.matches)
        } else {
            b.win_rate.partial_cmp(&a.win_rate).unwrap_or(std::cmp::Ordering::Equal)
        }
    });

    let top_ids: std::collections::HashSet<u32> = top_heroes.iter().map(|h| h.hero_id).collect();

    while top_heroes.len() < 7 {
        if let Some(idx) = raw_heroes.iter().position(|h| !top_ids.contains(&h.hero_id) && !seen_hero_names.contains(&h.hero_name.to_lowercase())) {
            let h = raw_heroes.remove(idx);
            seen_hero_names.insert(h.hero_name.to_lowercase());
            let slug = h.npc.or_else(|| map.slug_for(h.hero_id).map(|s| s.to_string())).unwrap_or_else(|| derive_slug(&h.hero_name));
            top_heroes.push(HeroMeta {
                hero_id: h.hero_id,
                name: h.hero_name,
                slug,
                winrate: h.win_rate,
                pickrate: 0.0,
                matches: h.matches,
                d2pt_rating: h.d2pt_rating,
                is_top: true,
            });
        } else {
            break;
        }
    }

    let top_ids: std::collections::HashSet<u32> = top_heroes.iter().map(|h| h.hero_id).collect();

    let other_heroes: Vec<HeroMeta> = raw_heroes
        .into_iter()
        .filter(|h| !top_ids.contains(&h.hero_id))
        .map(|h| {
            let slug = h.npc.or_else(|| map.slug_for(h.hero_id).map(|s| s.to_string())).unwrap_or_else(|| derive_slug(&h.hero_name));
            HeroMeta {
                hero_id: h.hero_id,
                name: h.hero_name,
                slug,
                winrate: h.win_rate,
                pickrate: 0.0,
                matches: h.matches,
                d2pt_rating: h.d2pt_rating,
                is_top: false,
            }
        })
        .collect();

    let mut heroes = top_heroes;
    heroes.extend(other_heroes);

    if heroes.is_empty() {
        return Err(ProviderError::Parse(format!(
            "no heroes parsed for {expected_pos_str}"
        )));
    }

    let total_matches: u64 = heroes.iter().map(|h| h.matches as u64).sum();
    for h in &mut heroes {
        if total_matches > 0 {
            h.pickrate = h.matches as f32 / total_matches as f32;
        }
    }

    let sample_count = std::cmp::min(7, heroes.len());
    let role_winrate = if sample_count == 0 {
        0.0
    } else {
        heroes.iter().take(sample_count).map(|h| h.winrate).sum::<f32>() / sample_count as f32
    };

    Ok(RoleMeta {
        position: target_pos,
        role_winrate,
        heroes,
    })
}

pub fn parse_meta(raw: &str, map: &HeroMap, top_n: usize) -> Result<MetaSnapshot, ProviderError> {
    let roles_blob = extract_roles_array(raw)?;

    let role_re = Regex::new(
        r#"position:\s*"([^"]+)",\s*roleName:\s*"([^"]+)".*?heroes:\s*\[([\s\S]*?)\]"#,
    )
    .map_err(|e| ProviderError::Parse(format!("bad role regex: {e}")))?;
    let hero_re = Regex::new(
        r#"hero_id:\s*(\d+).*?hero_name:\s*"([^"]+)".*?position:\s*"([^"]+)".*?matches:\s*(\d+).*?win_rate:\s*(\.?\d+(?:\.\d+)?)"#,
    )
    .map_err(|e| ProviderError::Parse(format!("bad hero regex: {e}")))?;

    let mut roles: Vec<RoleMeta> = Vec::new();

    for role_cap in role_re.captures_iter(roles_blob) {
        let Some(position) = position_from_d2pt(&role_cap[1]) else {
            tracing::warn!(raw_position = %&role_cap[1], "d2pt: unknown position, skipping role");
            continue;
        };
        let heroes_blob = &role_cap[3];

        let mut raw_heroes: Vec<RawHero> = Vec::new();
        for hero_cap in hero_re.captures_iter(heroes_blob) {
            let Ok(hero_id) = hero_cap[1].parse::<u32>() else {
                tracing::warn!(raw = %&hero_cap[1], "d2pt: unparseable hero_id, dropping hero");
                continue;
            };
            if hero_id == 0 {
                tracing::warn!(hero_name = %&hero_cap[2], "d2pt: hero_id 0 (unresolved), dropping hero");
                continue;
            }
            let hero_name = hero_cap[2].to_string();
            let Ok(matches) = hero_cap[4].parse::<u32>() else {
                tracing::warn!(hero_name = %hero_name, "d2pt: unparseable matches, dropping hero");
                continue;
            };
            let Some(win_rate) = parse_win_rate(&hero_cap[5]) else {
                tracing::warn!(hero_name = %hero_name, "d2pt: unparseable win_rate, dropping hero");
                continue;
            };

            raw_heroes.push(RawHero {
                hero_id,
                hero_name,
                npc: None,
                period: String::new(),
                matches,
                win_rate,
                d2pt_rating: 0,
            });
        }

        let total_matches: u64 = raw_heroes.iter().map(|h| h.matches as u64).sum();

        let mut heroes: Vec<HeroMeta> = raw_heroes
            .into_iter()
            .map(|h| {
                let slug = map
                    .slug_for(h.hero_id)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| derive_slug(&h.hero_name));
                let pickrate = if total_matches > 0 {
                    h.matches as f32 / total_matches as f32
                } else {
                    0.0
                };
                HeroMeta {
                    hero_id: h.hero_id,
                    name: h.hero_name,
                    slug,
                    winrate: h.win_rate,
                    pickrate,
                    matches: h.matches,
                    d2pt_rating: 0,
                    is_top: false,
                }
            })
            .collect();

        heroes.sort_by(|a, b| {
            b.pickrate
                .partial_cmp(&a.pickrate)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for (i, h) in heroes.iter_mut().enumerate() {
            if i < 7 {
                h.is_top = true;
            }
        }
        heroes.truncate(top_n);

        let role_winrate = if heroes.is_empty() {
            0.0
        } else {
            heroes.iter().map(|h| h.winrate).sum::<f32>() / heroes.len() as f32
        };

        roles.push(RoleMeta {
            position,
            role_winrate,
            heroes,
        });
    }

    if roles.len() != Position::all().len() {
        return Err(ProviderError::Parse(format!(
            "expected 5 roles, parsed {}",
            roles.len()
        )));
    }

    roles.sort_by_key(|r| Position::all().iter().position(|p| *p == r.position));

    let patch = extract_patch(raw);

    Ok(MetaSnapshot {
        patch,
        fetched_at: chrono::Utc::now().to_rfc3339(),
        source: "d2pt".to_string(),
        roles,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hero_map::HeroMap;

    const FIXTURE_HOME: &str = include_str!("../../tests/fixtures/d2pt_home.html");
    const FIXTURE_POS1: &str = include_str!("../../tests/fixtures/d2pt_pos1.html");

    #[test]
    fn parses_five_roles_top_n_sorted() {
        let map = HeroMap::bundled();
        let snap = parse_meta(FIXTURE_HOME, &map, 10).unwrap();
        assert_eq!(snap.roles.len(), 5);
        for r in &snap.roles {
            assert!(r.heroes.len() <= 10);
            assert!(r.heroes.iter().all(|h| h.hero_id != 0));
            assert!(r.heroes.windows(2).all(|w| w[0].pickrate >= w[1].pickrate));
        }
        assert!(!snap.patch.is_empty());
    }

    #[test]
    fn parses_pos1_meta_fixture_top_and_other_heroes() {
        let map = HeroMap::bundled();
        let role = parse_pos_meta(FIXTURE_POS1, Position::Pos1, &map, 7).unwrap();
        assert_eq!(role.position, Position::Pos1);
        assert_eq!(role.heroes.len(), 66);
        assert_eq!(role.heroes.iter().filter(|h| h.is_top).count(), 7);
        assert!(role.heroes.iter().all(|h| h.hero_id != 0));
        assert!(role.role_winrate > 0.0);
    }

    #[tokio::test]
    #[ignore]
    async fn d2pt_live_fetch() {
        let p = D2ptProvider::new();
        let snap = p.fetch(&HeroMap::bundled(), 15, "pubs", -1).await.unwrap();
        assert_eq!(snap.roles.len(), 5);
        for r in &snap.roles {
            assert_eq!(r.heroes.len(), 15);
        }
    }
}
