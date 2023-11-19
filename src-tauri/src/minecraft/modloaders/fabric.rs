use log::{info, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::minecraft::launching::mc_structs::{MCArguments, MCAssetIndex, MCDownloads, MCJavaVersion, MCLogging};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricVersionManifest {
    pub arguments: MCArguments,
    pub minecraft_arguments: Option<String>,
    pub asset_index: MCAssetIndex,
    pub assets: String,
    pub compliance_level: u16,
    pub downloads: MCDownloads,
    pub id: String,
    pub java_version: MCJavaVersion,
    pub libraries: Vec<FabricLibrary>,
    pub main_class: String,
    pub minimum_launcher_version: u16,
    pub release_time: String,
    pub time: String,
    pub logging: Option<MCLogging>,
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricLibrary {
    name: String,
    url: String
}

impl FabricVersionManifest {
    pub async fn get(mc_ver: String, loader_ver: String, client: &Client) -> Option<Self> {
        let url = format!("https://meta.fabricmc.net/v2/versions/loader/{mc_ver}/{loader_ver}/profile/json");
        info!("Getting Fabric version manifest from {url}...");

        match client.get(url).send().await.unwrap().json::<Self>().await {
            Ok(manifest) => Some(manifest),
            Err(e) => {
                error!("Failed to get fabric version manifest: {}", e.to_string());
                None
            }
        }
    }
}