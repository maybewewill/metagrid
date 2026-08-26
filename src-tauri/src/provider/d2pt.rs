use regex::Regex;
use tokio::process::Command;

use crate::hero_map::HeroMap;
use crate::model::{HeroMeta, MetaSnapshot, Position, RoleMeta};
use crate::provider::{MetaProvider, ProviderError};

const D2PT_URL: &str = "https://dota2protracker.com/";
const D2PT_META_URL: &str = "https://dota2protracker.com/meta?mmr=7000&period=8&position=pos%2B";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const ACCEPT: &str =
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9";

/// Provider for https://dota2protracker.com/ meta stats.
///
/// Shells out to `curl.exe` (Windows' built-in Schannel-backed curl)
/// rather than using `reqwest`: dota2protracker.com sits behind
/// Cloudflare and rejects reqwest/rustls TLS handshakes with a 403,
/// while curl.exe's native Windows TLS stack passes cleanly.
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
    async fn run_curl(&self, url: &str) -> Result<String, ProviderError> {
        let mut cmd = Command::new("curl.exe");
        cmd.arg("-s")
            .arg("-H")
            .arg(format!("User-Agent: {USER_AGENT}"))
            .arg("-H")
            .arg(format!("Accept: {ACCEPT}"))
            .arg("-H")
            .arg(format!("Accept-Language: {ACCEPT_LANGUAGE}"))
            .arg(url);

        #[cfg(target_os = "windows")]
        {
            // CREATE_NO_WINDOW = 0x0800_0000 ensures no visible cmd window flashes.
            cmd.creation_flags(0x0800_0000);
        }

        let output = cmd.output().await.map_err(|e| {
            tracing::warn!(error = %e, "d2pt: curl execution failed");
            ProviderError::Blocked
        })?;

        if !output.status.success() {
            tracing::warn!(code = ?output.status.code(), "d2pt: curl exited non-zero");
            return Err(ProviderError::Blocked);
        }

        let body = String::from_utf8_lossy(&output.stdout).to_string();
        if body.is_empty() || body.contains("Just a moment") {
            tracing::warn!("d2pt: response looked blocked (empty or Cloudflare challenge)");
            return Err(ProviderError::Blocked);
        }

        Ok(body)
    }

    /// Fetch the raw d2pt homepage payload.
    pub async fn fetch_raw(&self) -> Result<String, ProviderError> {
        let body = self.run_curl(D2PT_URL).await?;
        if !body.contains("roles:[") {
            tracing::warn!("d2pt: homepage response missing 'roles:[' marker");
            return Err(ProviderError::Blocked);
        }
        Ok(body)
    }

    /// Fetch raw payload for a specific role position from `/meta?mmr=7000&period=8&position=pos+{p}`.
    pub async fn fetch_pos_raw(&self, pos: u8) -> Result<String, ProviderError> {
        let url = format!("{D2PT_META_URL}{pos}");
        self.run_curl(&url).await
    }

    /// Fetches full meta for all 5 positions in parallel from `/meta?position=pos {1..5}`.
    pub async fn fetch_meta_all(&self, map: &HeroMap, top_n: usize) -> Result<MetaSnapshot, ProviderError> {
        let (p1, p2, p3, p4, p5) = tokio::join!(
            self.fetch_pos_raw(1),
            self.fetch_pos_raw(2),
            self.fetch_pos_raw(3),
            self.fetch_pos_raw(4),
            self.fetch_pos_raw(5),
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

        for (pos, body) in bodies {
            if patch == "unknown" {
                patch = extract_patch(&body);
            }
            let role_meta = parse_pos_meta(&body, pos, map, top_n)?;
            roles.push(role_meta);
        }

        Ok(MetaSnapshot {
            patch,
            fetched_at: chrono::Utc::now().to_rfc3339(),
            source: "d2pt".to_string(),
            roles,
        })
    }
}

#[async_trait::async_trait]
impl MetaProvider for D2ptProvider {
    fn id(&self) -> &'static str {
        "d2pt"
    }

    async fn fetch(&self, map: &HeroMap, top_n: usize) -> Result<MetaSnapshot, ProviderError> {
        // Try the rich /meta endpoint first (supports full hero pool per role).
        match self.fetch_meta_all(map, top_n).await {
            Ok(snap) => Ok(snap),
            Err(e) => {
                tracing::warn!(error = %e, "d2pt: /meta multi-fetch failed, falling back to homepage");
                let raw = self.fetch_raw().await?;
                parse_meta(&raw, map, top_n)
            }
        }
    }
}

/// A hero entry as pulled straight out of the data payload.
struct RawHero {
    hero_id: u32,
    hero_name: String,
    npc: Option<String>,
    matches: u32,
    win_rate: f32,
    d2pt_rating: u32,
}

/// Locate the `roles:[...]` array in the raw homepage HTML/JS payload.
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

/// Parse the fixed-point winrate D2PT emits (e.g. `.5321`).
fn parse_win_rate(raw: &str) -> Option<f32> {
    let normalized = if let Some(rest) = raw.strip_prefix('.') {
        format!("0.{rest}")
    } else {
        raw.to_string()
    };
    normalized.parse::<f32>().ok()
}

/// Normalize a hero display name into a slug the same way d2pt hero keys look.
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

/// Best-effort extraction of the current patch string (e.g. `"7.41e"`) from
/// the page payload.
fn extract_patch(html: &str) -> String {
    let markers = [
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
    "unknown".to_string()
}

/// Parse a single position's payload from `https://dota2protracker.com/meta?position=pos%20{N}`.
pub fn parse_pos_meta(
    raw: &str,
    target_pos: Position,
    map: &HeroMap,
    _top_n: usize,
) -> Result<RoleMeta, ProviderError> {
    // 1. Extract Top Heroes from the HTML Top Heroes container
    let mut top_hero_names = Vec::new();
    if let Some(top_idx) = raw.find("Top Heroes") {
        let slice_len = std::cmp::min(6000, raw.len() - top_idx);
        let top_slice = &raw[top_idx..top_idx + slice_len];
        if let Ok(card_re) = Regex::new(r#"<a\s+href="/hero/([^"]+)""#) {
            for cap in card_re.captures_iter(top_slice) {
                if top_hero_names.len() >= 7 {
                    break;
                }
                let decoded = urlencoding::decode(&cap[1]).unwrap_or_default().to_string();
                top_hero_names.push(decoded);
            }
        }
    }

    let hero_re = Regex::new(
        r#"\{hero_id:(\d+),hero_name:"([^"]+)",npc:"([^"]+)",position:"([^"]+)"[\s\S]*?matches:(\d+),wins:(\d+),win_rate:(\.?\d+(?:\.\d+)?)[\s\S]*?d2pt_rating:(\d+)"#,
    )
    .map_err(|e| ProviderError::Parse(format!("bad /meta hero regex: {e}")))?;

    let mut raw_heroes: Vec<RawHero> = Vec::new();
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
        let Ok(matches) = cap[5].parse::<u32>() else { continue; };
        let Some(win_rate) = parse_win_rate(&cap[7]) else { continue; };
        let d2pt_rating = cap.get(8).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(0);

        raw_heroes.push(RawHero {
            hero_id,
            hero_name,
            npc,
            matches,
            win_rate,
            d2pt_rating,
        });
    }

    if raw_heroes.is_empty() {
        return Err(ProviderError::Parse(format!(
            "no heroes parsed for {expected_pos_str}"
        )));
    }

    let total_matches: u64 = raw_heroes.iter().map(|h| h.matches as u64).sum();

    let all_heroes: Vec<HeroMeta> = raw_heroes
        .into_iter()
        .map(|h| {
            let slug = h
                .npc
                .or_else(|| map.slug_for(h.hero_id).map(|s| s.to_string()))
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
                d2pt_rating: h.d2pt_rating,
                is_top: true,
            }
        })
        .collect();

    let mut heroes: Vec<HeroMeta> = Vec::new();
    if !top_hero_names.is_empty() {
        for top_name in &top_hero_names {
            if let Some(found) = all_heroes.iter().find(|h| {
                top_name.eq_ignore_ascii_case(&h.name)
                    || top_name.eq_ignore_ascii_case(&h.slug)
                    || derive_slug(top_name).eq_ignore_ascii_case(&h.slug)
            }) {
                if !heroes.iter().any(|existing| existing.hero_id == found.hero_id) {
                    heroes.push(found.clone());
                }
            }
        }
    }

    if heroes.is_empty() {
        // Fallback: top 7 by d2pt_rating descending
        let mut sorted = all_heroes;
        sorted.sort_by(|a, b| {
            if b.d2pt_rating != a.d2pt_rating {
                b.d2pt_rating.cmp(&a.d2pt_rating)
            } else {
                b.winrate.partial_cmp(&a.winrate).unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        heroes = sorted.into_iter().take(7).collect();
    }

    let role_winrate = if heroes.is_empty() {
        0.0
    } else {
        heroes.iter().map(|h| h.winrate).sum::<f32>() / heroes.len() as f32
    };

    Ok(RoleMeta {
        position: target_pos,
        role_winrate,
        heroes,
    })
}

/// Parse a d2pt homepage payload into a `MetaSnapshot`.
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

    // Order deterministically as Pos1..Pos5 regardless of source order.
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
        assert_eq!(snap.roles.len(), 5); // POS 1..5
        for r in &snap.roles {
            assert!(r.heroes.len() <= 10);
            assert!(r.heroes.iter().all(|h| h.hero_id != 0)); // every name resolved
            assert!(r.heroes.windows(2).all(|w| w[0].pickrate >= w[1].pickrate));
        }
        assert!(!snap.patch.is_empty());
    }

    #[test]
    fn parses_pos1_meta_fixture_top_heroes() {
        let map = HeroMap::bundled();
        let role = parse_pos_meta(FIXTURE_POS1, Position::Pos1, &map, 7).unwrap();
        assert_eq!(role.position, Position::Pos1);
        assert_eq!(role.heroes.len(), 7);
        assert!(role.heroes.iter().all(|h| h.hero_id != 0));
        assert!(role.role_winrate > 0.0);
    }

    #[tokio::test]
    #[ignore] // run: cargo test --manifest-path src-tauri/Cargo.toml -- --ignored d2pt_live
    async fn d2pt_live_fetch() {
        let p = D2ptProvider::new();
        let snap = p.fetch(&HeroMap::bundled(), 15).await.unwrap();
        assert_eq!(snap.roles.len(), 5);
        for r in &snap.roles {
            assert_eq!(r.heroes.len(), 15);
        }
    }
}
