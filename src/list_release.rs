use reqwest::header;
use serde::Deserialize;

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
    name: String,
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
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()?;
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    let resp = client.get(&url).send().await?;
    let releases: Vec<Release> = resp.json().await?;
    Ok(releases)
}