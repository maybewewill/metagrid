use regex::Regex;
use tokio::process::Command;

use crate::hero_map::HeroMap;
use crate::model::{HeroMeta, MetaSnapshot, Position, RoleMeta};
use crate::provider::{MetaProvider, ProviderError};

const D2PT_URL: &str = "https://dota2protracker.com/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const ACCEPT: &str =
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9";

/// Provider for https://dota2protracker.com/ meta stats.
///
/// `fetch_raw` shells out to `curl.exe` (see `MetaProvider` impl), so this is
/// a unit struct with no reqwest client to hold.
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
    /// Fetch the raw d2pt homepage payload.
    ///
    /// Shells out to `curl.exe` (Windows' built-in Schannel-backed curl)
    /// rather than using `reqwest`: dota2protracker.com sits behind
    /// Cloudflare and rejects reqwest/rustls TLS handshakes with a 403,
    /// while curl.exe's native Windows TLS stack passes cleanly. This is a
    /// deliberate, proven workaround — do not swap back to reqwest for this
    /// provider.
    pub async fn fetch_raw(&self) -> Result<String, ProviderError> {
        let mut cmd = Command::new("curl.exe");
        cmd.arg("-s")
            .arg("-H")
            .arg(format!("User-Agent: {USER_AGENT}"))
            .arg("-H")
            .arg(format!("Accept: {ACCEPT}"))
            .arg("-H")
            .arg(format!("Accept-Language: {ACCEPT_LANGUAGE}"))
            .arg(D2PT_URL);

        // Suppress the console window curl.exe would otherwise flash on every
        // fetch: CREATE_NO_WINDOW (0x0800_0000). Without this a black cmd
        // window pops up each refresh — the app runs windowless in the tray.
        #[cfg(windows)]
        cmd.creation_flags(0x0800_0000);

        let output = cmd
            .output()
            .await
            .map_err(|e| {
                tracing::warn!(
                    error = %e,
                    "d2pt: Windows curl.exe not found (need Win10 1803+)"
                );
                ProviderError::Blocked
            })?;

        // A non-zero exit (e.g. TLS/connection failure) is treated the same
        // as a blocked request rather than a distinct failure mode.
        if !output.status.success() {
            tracing::warn!(status = ?output.status, "d2pt: curl.exe exited non-zero");
            return Err(ProviderError::Blocked);
        }

        let body = String::from_utf8_lossy(&output.stdout).into_owned();

        if body.is_empty() || body.contains("Just a moment") || !body.contains("roles:[") {
            tracing::warn!("d2pt: response looked blocked (empty/challenge/no roles marker)");
            return Err(ProviderError::Blocked);
        }

        Ok(body)
    }
}

#[async_trait::async_trait]
impl MetaProvider for D2ptProvider {
    fn id(&self) -> &'static str {
        "d2pt"
    }

    async fn fetch(&self, map: &HeroMap, top_n: usize) -> Result<MetaSnapshot, ProviderError> {
        let raw = self.fetch_raw().await?;
        parse_meta(&raw, map, top_n)
    }
}

/// A hero entry as pulled straight out of the `roles:[...]` blob, before
/// mapping into the domain `HeroMeta`.
struct RawHero {
    hero_id: u32,
    hero_name: String,
    matches: u32,
    win_rate: f32,
}

/// Locate the `roles:[...]` array in the raw HTML/JS payload and return the
/// slice spanning from its opening `[` to the matching closing `]`,
/// inclusive. Balanced-bracket, string-aware (honors `\"` escapes) so that
/// brackets appearing inside quoted strings don't confuse the depth count.
///
/// Ported from the proven reference parser
/// (`~/.gemini/antigravity/scratch/d2pt_rust/src/main.rs::parse_d2pt_roles`),
/// but tracks byte offsets via `char_indices` instead of a `Vec<char>` index
/// so it stays correct if the payload ever contains multi-byte characters
/// inside the array.
fn extract_roles_array(html: &str) -> Result<&str, ProviderError> {
    let start = html
        .find("roles:[")
        .ok_or_else(|| ProviderError::Parse("missing 'roles:[' marker in response".into()))?;
    // "roles:" is 6 bytes (all ASCII); this lands exactly on the '['.
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

/// Parse the fixed-point winrate D2PT emits (e.g. `.5321`, which JS/JSON
/// serializes without the leading zero) into a normal `0.0..1.0` f32.
fn parse_win_rate(raw: &str) -> Option<f32> {
    let normalized = if let Some(rest) = raw.strip_prefix('.') {
        format!("0.{rest}")
    } else {
        raw.to_string()
    };
    normalized.parse::<f32>().ok()
}

/// Normalize a hero display name into a slug the same way d2pt/OpenDota
/// hero keys look: lowercase, spaces/hyphens/apostrophes stripped.
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
/// the page payload. Tries the most specific markers first.
fn extract_patch(html: &str) -> String {
    let markers = [
        r#"patchVersion:\s*"([^"]+)""#,
        r#"patch:\s*\{\s*version:\s*"([^"]+)""#,
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

/// Parse a d2pt homepage payload (raw HTML containing an embedded
/// `roles:[...]` JS array) into a `MetaSnapshot`.
///
/// Fails closed: if fewer than all 5 positions parse successfully, returns
/// `ProviderError::Parse` rather than a partial snapshot.
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
                matches,
                win_rate,
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

    const FIXTURE: &str = include_str!("../../tests/fixtures/d2pt_home.html");

    #[test]
    fn parses_five_roles_top_n_sorted() {
        let map = HeroMap::bundled();
        let snap = parse_meta(FIXTURE, &map, 10).unwrap();
        assert_eq!(snap.roles.len(), 5); // POS 1..5
        for r in &snap.roles {
            assert!(r.heroes.len() <= 10);
            assert!(r.heroes.iter().all(|h| h.hero_id != 0)); // every name resolved
                                                                // default sort = pickrate desc
            assert!(r.heroes.windows(2).all(|w| w[0].pickrate >= w[1].pickrate));
        }
        assert!(!snap.patch.is_empty());
    }

    #[tokio::test]
    #[ignore] // run: cargo test --manifest-path src-tauri/Cargo.toml -- --ignored d2pt_live
    async fn d2pt_live_fetch() {
        let p = D2ptProvider::new();
        let snap = p.fetch(&HeroMap::bundled(), 10).await.unwrap();
        assert_eq!(snap.roles.len(), 5);
    }
}
