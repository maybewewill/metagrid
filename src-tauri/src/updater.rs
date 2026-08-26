use serde::{Deserialize, Serialize};

const GITHUB_API_URL: &str = "https://api.github.com/repos/maybewewill/metagrid/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

pub fn get_current_version_display() -> String {
    match GIT_HASH {
        Some(hash) if !hash.is_empty() && hash != "unknown" => format!("v{CURRENT_VERSION}+{hash}"),
        _ => format!("v{CURRENT_VERSION}"),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub release_notes: Option<String>,
    pub download_url: Option<String>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
    #[serde(default)]
    assets: Vec<GitHubAsset>,
}

pub async fn check_for_updates() -> Result<UpdateInfo, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "MetaGrid-App")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }

    let text = response.text().await.map_err(|e| e.to_string())?;
    let release: GitHubRelease = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let latest_tag = release.tag_name.trim();
    let latest_ver = latest_tag.trim_start_matches('v');

    let available = is_newer_version(latest_ver, CURRENT_VERSION);

    let download_url = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(".exe") || a.name.ends_with(".zip"))
        .map(|a| a.browser_download_url.clone())
        .or_else(|| Some(release.html_url.clone()));

    Ok(UpdateInfo {
        available,
        current_version: get_current_version_display(),
        latest_version: latest_tag.to_string(),
        release_url: release.html_url,
        release_notes: release.body,
        download_url,
    })
}

fn is_newer_version(latest: &str, current: &str) -> bool {
    fn clean(v: &str) -> &str {
        v.split('+').next().unwrap_or(v).trim_start_matches('v')
    }
    let parse = |v: &str| -> Vec<u32> {
        clean(v)
            .split('.')
            .map(|p| p.chars().take_while(|c| c.is_ascii_digit()).collect::<String>())
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    };

    let l_parts = parse(latest);
    let c_parts = parse(current);

    l_parts > c_parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_semver_correctly() {
        assert!(is_newer_version("1.0.2", "1.0.1"));
        assert!(is_newer_version("1.1.0", "1.0.9"));
        assert!(is_newer_version("2.0.0", "1.9.9"));
        assert!(is_newer_version("1.0.2", "1.0.1+abc1234"));
        assert!(!is_newer_version("1.0.1", "1.0.1"));
        assert!(!is_newer_version("1.0.1", "1.0.1+abc1234"));
        assert!(!is_newer_version("1.0.0", "1.0.1"));
    }
}
