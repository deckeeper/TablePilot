//! TablePilot release metadata only. Installers are opened in the user's browser.
//! No upstream updater, executable download, or unattended installation is exposed.
use std::time::Duration;
use reqwest::{header, redirect::Policy};
use semver::Version;
use serde::Deserialize;
use serde_json::{json, Value};

const RELEASES_API: &str = "https://api.github.com/repos/deckeeper/TablePilot/releases?per_page=100";
const RELEASES_URL: &str = "https://github.com/deckeeper/TablePilot/releases";

#[derive(Deserialize)]
struct Asset { name: String, size: u64, state: String }
#[derive(Deserialize)]
struct Release {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    published_at: Option<String>,
    draft: bool,
    prerelease: bool,
    assets: Vec<Asset>,
}

fn stable_version(release: &Release) -> Option<Version> {
    if release.draft || release.prerelease { return None; }
    let version = Version::parse(release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name)).ok()?;
    if !version.pre.is_empty() || !version.build.is_empty() || release.tag_name != format!("v{version}") { return None; }
    // Ignore partially published releases until both installer assets are available.
    for name in [format!("TablePilot_{version}_x64-setup.exe"), format!("TablePilot_{version}_amd64.deb")] {
        if !release.assets.iter().any(|a| a.name == name && a.size >= 1_000_000 && a.state == "uploaded") { return None; }
    }
    Some(version)
}

async fn releases() -> Result<Vec<(Version, Release)>, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        // Release metadata is fetched only from the fixed TablePilot repository.
        .redirect(Policy::none())
        .user_agent(concat!("TablePilot/", env!("CARGO_PKG_VERSION")))
        .build().map_err(|_| "Could not initialize the TablePilot update connection".to_string())?;
    let request = client.get(RELEASES_API)
        .header(header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");
    let response = request.send().await.map_err(|_| "Unable to reach TablePilot releases. Check your network connection and retry.".to_string())?;
    match response.status().as_u16() {
        200 => {},
        401 | 404 => return Err("TablePilot releases are unavailable. The deckeeper/TablePilot repository must be public for automatic update checks.".into()),
        403 | 429 => return Err("GitHub refused the release check. Retry after the API rate limit resets.".into()),
        _ => return Err(format!("TablePilot release check failed (HTTP {}). Retry later.", response.status().as_u16())),
    }
    let rows: Vec<Release> = response.json().await.map_err(|_| "GitHub returned an invalid TablePilot release list".to_string())?;
    let mut releases: Vec<_> = rows.into_iter().filter_map(|r| stable_version(&r).map(|v| (v, r))).collect();
    releases.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(releases)
}

#[tauri::command]
pub async fn tablepilot_check_for_updates() -> Result<Value, String> {
    let rows = releases().await?;
    let (latest, release) = rows.first().ok_or("No complete official TablePilot release is available yet.")?;
    let current = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|_| "Invalid installed version")?;
    Ok(json!({
        "current_version": current.to_string(), "latest_version": latest.to_string(),
        "update_available": latest > &current, "portable_mode": false, "manual_update_only": true,
        "release_name": release.name.as_deref().unwrap_or(&release.tag_name),
        "release_url": format!("{RELEASES_URL}/tag/v{latest}"),
        "release_notes": release.body.as_deref().unwrap_or("")
    }))
}

#[tauri::command]
pub async fn tablepilot_fetch_changelog() -> Result<Value, String> {
    let rows = releases().await?;
    let releases: Vec<Value> = rows.iter().map(|(version, release)| json!({
        "tag": format!("v{version}"), "name": release.name.as_deref().unwrap_or(&release.tag_name),
        "date": release.published_at.as_deref().unwrap_or(""),
        "sections": [{"type": "changed", "title": "Release notes", "items": [{
            "title": release.body.as_deref().unwrap_or("No release notes provided."), "desc": ""
        }]}]
    })).collect();
    Ok(json!({"updatedAt": rows.first().and_then(|(_, r)| r.published_at.as_deref()).unwrap_or(""), "releases": releases}))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn release() -> Release {
        serde_json::from_value(json!({"tag_name":"v0.4.0", "name":null, "body":null, "published_at":null,
            "draft":false, "prerelease":false, "assets":[
                {"name":"TablePilot_0.4.0_x64-setup.exe","size":2000000,"state":"uploaded"},
                {"name":"TablePilot_0.4.0_amd64.deb","size":2000000,"state":"uploaded"}]})).unwrap()
    }
    #[test]
    fn only_complete_stable_releases_are_offered() {
        let mut r = release();
        assert_eq!(stable_version(&r).unwrap(), Version::new(0, 4, 0));
        r.prerelease = true; assert!(stable_version(&r).is_none());
        r.prerelease = false; r.draft = true; assert!(stable_version(&r).is_none());
        r.draft = false; r.tag_name = "v0.4.1-beta.1".into(); assert!(stable_version(&r).is_none());
        r.tag_name = "bad-tag".into(); assert!(stable_version(&r).is_none());
        r.tag_name = "v0.4.0".into(); r.assets.pop(); assert!(stable_version(&r).is_none());
    }
}
