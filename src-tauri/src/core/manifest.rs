use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

pub async fn fetch_version_manifest() -> Result<VersionManifest, Box<dyn Error>> {
    let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    let resp = reqwest::get(url).await?.json::<VersionManifest>().await?;
    Ok(resp)
}
