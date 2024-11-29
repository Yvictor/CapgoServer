use poem_openapi::{payload::Json, ApiResponse, Object};
use reqwest::{header, Error};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

#[derive(Debug, Deserialize)]
pub struct Release {
    url: String,
    html_url: String,
    assets_url: String,
    upload_url: String,
    tarball_url: String,
    zipball_url: String,
    id: u64,
    node_id: String,
    pub tag_name: String,
    target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    draft: bool,
    pub prerelease: bool,
    created_at: String,
    published_at: String,
    author: Author,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    author_type: String,
    site_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub url: String,
    pub browser_download_url: String,
    id: u64,
    node_id: String,
    pub name: String,
    label: Option<String>,
    state: String,
    content_type: String,
    size: u64,
    download_count: u64,
    created_at: String,
    updated_at: String,
    uploader: Author,
}

pub async fn list_releases(owner: &str, repo: &str) -> Result<Vec<Release>, reqwest::Error> {
    let user_agent = header::HeaderValue::from_str("CapgoServer").unwrap();
    let client = reqwest::Client::builder().user_agent(user_agent).build()?;
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    let resp = client.get(&url).send().await?;
    let releases: Vec<Release> = resp.json().await?;
    Ok(releases)
}

async fn download_and_read_asset(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;

    Ok(content)
}

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct UpdateInfo {
    pub version: String,
    pub url: String,
    pub session_key: Option<String>,
    pub checksum: Option<String>,
}

impl UpdateInfo {
    pub fn new(version: &str, url: &str, session_key: Option<String>, checksum: Option<String>) -> Self {
        UpdateInfo {
            version: version.to_string(),
            url: url.to_string(),
            session_key,
            checksum,
        }
    }
}

#[derive(Debug, Deserialize, Object)]
pub struct AppInfos {
    platform: String,
    device_id: String,
    app_id: String,
    custom_id: Option<String>,
    plugin_version: String,
    version_build: String,
    version_code: String,
    version_name: String,
    version_os: String,
    is_emulator: bool,
    is_prod: bool,
}

#[derive(Debug, Serialize, Object)]
pub struct ErrorResponse {
    pub message: String,
    pub error: String,
    // version: String,
    // url: String,
}

#[derive(ApiResponse)]
pub enum UpdateResponse {
    #[oai(status = 200)]
    UpdateInfo(Json<UpdateInfo>),

    #[oai(status = 404)]
    Error(Json<ErrorResponse>),
}

pub async fn get_update_info(app_infos: &AppInfos) -> Option<UpdateInfo> {
    // Define the latest version for each platform
    // let latest_versions = vec![
    //     ("ios", "1.2.0"),
    //     ("android", "1.1.0"),
    // ];
    // let platform_latest_version = latest_versions.iter().find(|(p, _)| p == &app_infos.platform);
    let split_app_id: Vec<&str> = app_infos.app_id.split(".").collect();

    info!(
        platform = app_infos.platform,
        device_id = app_infos.device_id,
        app_id = app_infos.app_id,
        version_name = app_infos.version_name,
        "Received app_infos data"
    );

    if let [mut owner, mut repo] = split_app_id.as_slice() {
        owner = if repo == "scone" { "Sinotrade" } else { owner };
        info!(owner = owner, repo = repo, "Resolved repository information");

        let mut latest_version = String::from("0.0.1");
        let mut url = String::from(
            "https://github.com/Sinotrade/scone/releases/download/0.0.1s/yvictor.scone_0.0.1.zip",
        );
        let mut session_key: Option<String> = None;
        let mut checksum: Option<String> = None;
    
        let releases = list_releases(owner, repo).await.ok().unwrap_or_default();
        for release in releases {
            info!(
                tag_name = release.tag_name,
                release_name = ?release.name,
                "Processing release"
            );

            if (app_infos.version_name == "builtin") || (app_infos.version_name < release.tag_name) {
                for asset in &release.assets {
                    if asset.name == "key" {
                        let key_data = download_and_read_asset(&asset.browser_download_url).await;
                        match key_data {
                            Ok(k) => session_key = Some(k.trim().to_string()),
                            Err(e) => error!("Error downloading and reading key asset: {}", e),
                        }
                    } else if asset.name.ends_with(".zip") {
                        latest_version = release.tag_name.clone();
                        url = asset.browser_download_url.clone();
                        let checksum_filename = asset.name.replace(".zip", ".checksum");
                        if let Some(checksum_asset) = release.assets.iter().find(|a| a.name == checksum_filename) {
                            debug!("Found checksum asset: {}", checksum_asset.browser_download_url);
                            let checksum_data = download_and_read_asset(&checksum_asset.browser_download_url).await;
                            match checksum_data {
                                Ok(c) => checksum = Some(c.trim().to_string()),
                                Err(e) => error!("Error downloading and reading checksum asset: {}", e),
                            }
                        } else {
                            error!("Checksum asset '{}' not found for zip file '{}'", checksum_filename, asset.name);
                        }
                    }
                }
                break;
            } else {
                return None;
            }
        }
        Some(UpdateInfo {
            version: latest_version,
            url: url,
            session_key,
            checksum,
        })
    } else {
        return None;
    }
}