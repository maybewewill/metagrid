use std::path::{Path, PathBuf};

/// Default CDN base URL for Dota 2 hero portrait images.
pub const DEFAULT_BASE_URL: &str =
    "https://cdn.cloudflare.steamstatic.com/apps/dota2/images/dota_react/heroes/";

/// Path on disk where a hero's portrait is (or will be) cached:
/// `dir/portraits/<slug>.png`.
pub fn portrait_path(dir: &Path, slug: &str) -> PathBuf {
    dir.join("portraits").join(format!("{slug}.png"))
}

/// Ensure every hero in `slugs` has a cached portrait under `dir/portraits/`,
/// downloading only the ones that are missing. Returns the number of
/// portraits actually fetched.
pub async fn ensure(
    dir: &Path,
    client: &reqwest::Client,
    base_url: &str,
    slugs: &[String],
) -> usize {
    let portraits_dir = dir.join("portraits");
    if let Err(e) = std::fs::create_dir_all(&portraits_dir) {
        tracing::warn!("failed to create portraits dir {:?}: {e}", portraits_dir);
        return 0;
    }

    let mut fetched = 0usize;
    for slug in slugs {
        let path = portrait_path(dir, slug);
        if path.exists() {
            continue;
        }

        let url = format!("{base_url}{slug}.png");
        let resp = match client.get(&url).send().await {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!("failed to fetch portrait for {slug}: {e}");
                continue;
            }
        };
        let bytes = match resp.error_for_status() {
            Ok(r) => match r.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    tracing::warn!("failed to read portrait bytes for {slug}: {e}");
                    continue;
                }
            },
            Err(e) => {
                tracing::warn!("portrait fetch for {slug} returned error status: {e}");
                continue;
            }
        };

        if let Err(e) = std::fs::write(&path, &bytes) {
            tracing::warn!("failed to write portrait for {slug}: {e}");
            continue;
        }
        fetched += 1;
    }

    fetched
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_is_under_portraits() {
        let p = portrait_path(std::path::Path::new("/data"), "antimage");
        assert!(p.ends_with("portraits/antimage.png"));
    }

    /// Live network test — downloads a real portrait from the CDN.
    /// Run explicitly with:
    /// `cargo test --manifest-path src-tauri/Cargo.toml -- --ignored portraits --nocapture`
    #[tokio::test]
    #[ignore]
    async fn live_download_fetches_missing_only() {
        let tmp = tempfile::tempdir().unwrap();
        let client = reqwest::Client::new();
        let slugs = vec!["antimage".to_string()];

        let fetched = ensure(tmp.path(), &client, DEFAULT_BASE_URL, &slugs).await;
        assert_eq!(fetched, 1);
        assert!(portrait_path(tmp.path(), "antimage").exists());

        // Second call should skip the now-cached file.
        let fetched_again = ensure(tmp.path(), &client, DEFAULT_BASE_URL, &slugs).await;
        assert_eq!(fetched_again, 0);
    }
}
