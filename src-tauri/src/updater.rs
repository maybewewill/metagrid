use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncWriteExt;

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

    let is_matching_asset = |name: &str| {
        #[cfg(target_os = "windows")]
        {
            name.ends_with(".exe") || name.ends_with(".zip")
        }
        #[cfg(target_os = "macos")]
        {
            name.ends_with(".dmg") || name.ends_with(".app.tar.gz") || name.ends_with(".zip")
        }
        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            name.ends_with(".AppImage") || name.ends_with(".deb") || name.ends_with(".tar.gz") || name.ends_with(".pkg.tar.zst")
        }
    };

    let download_url = release
        .assets
        .iter()
        .find(|a| is_matching_asset(&a.name))
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

pub async fn download_and_install(app: &AppHandle, download_url: Option<String>) -> Result<(), String> {
    let url = match download_url {
        Some(u) if !u.is_empty() => u,
        _ => {
            let info = check_for_updates().await?;
            info.download_url
                .ok_or_else(|| "No update installer found in release".to_string())?
        }
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| e.to_string())?;

    let mut response = client
        .get(&url)
        .header("User-Agent", "MetaGrid-App")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Download failed with HTTP status: {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    #[cfg(target_os = "windows")]
    let temp_file = std::env::temp_dir().join("MetaGrid_Update_Setup.exe");

    #[cfg(target_os = "macos")]
    let temp_file = std::env::temp_dir().join("MetaGrid_Update.dmg");

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let temp_file = {
        let ext = if url.ends_with(".AppImage") {
            "AppImage"
        } else if url.ends_with(".deb") {
            "deb"
        } else if url.ends_with(".pkg.tar.zst") {
            "pkg.tar.zst"
        } else {
            "bin"
        };
        std::env::temp_dir().join(format!("MetaGrid_Update.{ext}"))
    };

    let mut file = tokio::fs::File::create(&temp_file)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        downloaded += chunk.len() as u64;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        if total_size > 0 {
            let percent = (downloaded as f64 / total_size as f64) * 100.0;
            let _ = app.emit("metagrid://update-progress", percent.min(100.0));
        }
    }

    file.flush().await.map_err(|e| e.to_string())?;
    drop(file);

    let _ = app.emit("metagrid://update-progress", 100.0);
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        std::process::Command::new(&temp_file)
            .arg("/S")
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("Failed to start silent update: {e}"))?;
    }
    #[cfg(target_os = "macos")]
    {
        let _ = tauri_plugin_opener::OpenerExt::opener(app).open_path(temp_file.to_string_lossy(), None::<&str>);
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        if temp_file.extension().and_then(|e| e.to_str()) == Some("AppImage") {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(&temp_file, std::fs::Permissions::from_mode(0o755));
            }
            std::process::Command::new(&temp_file)
                .spawn()
                .map_err(|e| format!("Failed to start AppImage: {e}"))?;
        } else {
            let _ = tauri_plugin_opener::OpenerExt::opener(app).open_path(temp_file.to_string_lossy(), None::<&str>);
        }
    }

    app.exit(0);
    Ok(())
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
