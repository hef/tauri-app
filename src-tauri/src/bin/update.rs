use regex::Regex;
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFile {
    pub version: String,
    pub notes: String,
    #[serde(rename = "pub_date")]
    pub pub_date: String,
    pub platforms: Platforms,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platforms {
    #[serde(rename = "darwin-x86_64")]
    pub darwin_x86_64: OsArch,
    #[serde(rename = "linux-x86_64")]
    pub linux_x86_64: OsArch,
    #[serde(rename = "windows-x86_64")]
    pub windows_x86_64: OsArch,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsArch {
    pub signature: String,
    pub url: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    do_stuff().await?;
    Ok(())
}

async fn do_stuff() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response: Root = client.get("https://api.github.com/repos/hef/tauri-app/releases/latest")
        .header("user-agent", "tauri-app/0.0.1")
        .send().await?.json().await?;
    
    let mut update_file = UpdateFile{
        version: response.name,
        notes: response.body,
        pub_date: response.published_at,
        ..Default::default()
    };

    // "tauri-app.app.tar.gz"
    // "tauri-app.app.tar.gz.sig"
    // "tauri-app_1.13.4_amd64.AppImage"
    // "tauri-app_1.13.4_amd64.AppImage.tar.gz"
    // "tauri-app_1.13.4_amd64.AppImage.tar.gz.sig"
    // "tauri-app_1.13.4_amd64.deb"
    // "tauri-app_1.13.4_x64.dmg"
    // "tauri-app_1.13.4_x64_en-US.msi"
    // "tauri-app_1.13.4_x64_en-US.msi.zip"
    // "tauri-app_1.13.4_x64_en-US.msi.zip.sig"

    let darwin = Regex::new(r"^.+.app.tar.gz$").unwrap();
    let darwin_sig = Regex::new(r"^.+.app.tar.gz.sig$").unwrap();
    let windows = Regex::new(r"^.+_\d+.\d+.\d+_x64_en-US.msi.zip$").unwrap();
    let windows_sig = Regex::new(r"^.+_\d+.\d+.\d+_x64_en-US.msi.zip.sig$").unwrap();
    let appimage = Regex::new(r"^.+_\d+.\d+.\d+_amd64.AppImage.tar.gz$").unwrap();
    let appimage_sig = Regex::new(r"^.+_\d+.\d+.\d+_amd64.AppImage.tar.gz.sig$").unwrap();

    for asset in response.assets.iter() {
        if darwin.is_match(&asset.name) {
            update_file.platforms.darwin_x86_64.url = asset.browser_download_url.clone();
            continue;
        }
        if windows.is_match(&asset.name) {
            update_file.platforms.windows_x86_64.url = asset.browser_download_url.clone();
            continue;
        }
        if appimage.is_match(&asset.name) {
            update_file.platforms.linux_x86_64.url = asset.browser_download_url.clone();
            continue;
        }
        if darwin_sig.is_match(&asset.name) {
            let signature_bytes = client.get(asset.browser_download_url.clone()).header("user-agent", "tauri-app/0.0.1").send().await?.bytes().await?;
            update_file.platforms.darwin_x86_64.signature = String::from_utf8_lossy(&signature_bytes).to_string();
            continue;
        }
        if windows_sig.is_match(&asset.name) {
            let signature_bytes = client.get(asset.browser_download_url.clone()).header("user-agent", "tauri-app/0.0.1").send().await?.bytes().await?;
            update_file.platforms.windows_x86_64.signature = String::from_utf8_lossy(&signature_bytes).to_string();
            continue;
        }
        if appimage_sig.is_match(&asset.name) {
            let signature_bytes = client.get(asset.browser_download_url.clone()).header("user-agent", "tauri-app/0.0.1").send().await?.bytes().await?;
            update_file.platforms.linux_x86_64.signature = String::from_utf8_lossy(&signature_bytes).to_string();
            continue;
        }
    }
    let x = serde_json::to_string(&update_file)?;
    println!("{}", x);
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub url: String,
    #[serde(rename = "assets_url")]
    pub assets_url: String,
    #[serde(rename = "upload_url")]
    pub upload_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    pub author: Author,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    pub assets: Vec<Asset>,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub uploader: Uploader,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub state: String,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}