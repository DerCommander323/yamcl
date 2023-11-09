use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

use super::libraries::{MCLibrary, MCRule};

const VERSION_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";


impl MCVersionList {
    pub fn get(client: &Client) -> Option<Self> {
        let version_list: Result<MCVersionList, reqwest::Error> = client.get(VERSION_URL).send().unwrap().json();
        match version_list {
            Ok(list) => Some(list),
            Err(e) => {
                println!("Failed to get Minecraft version list: {e}");
                None
            }
        }
    }
}

impl MCCompactVersion {
    pub fn from_id(version_id: String, client: &Client) -> Option<Self> {
        let version_list = MCVersionList::get(client)?;
        version_list.versions.into_iter().find(|ver| {
            ver.id == version_id
        })
    }
}

impl MCExtendedVersion {
    pub fn from_compact(compact_version: MCCompactVersion, client: &Client) -> Option<Self> {
        let extended_version: Result<MCExtendedVersion, reqwest::Error> = client.get(compact_version.url).send().unwrap().json();
        match extended_version {
            Ok(ver) => Some(ver),
            Err(e) => {
                println!("Failed to get extended Minecraft version info: {e}");
                None
            },
        }
    }

    pub fn get_jvm_args(&self) -> Vec<String> {
        let mut final_args: Vec<String> = Vec::new();
        if let Some(args) = self.arguments.as_ref() {
            for arg in args.jvm.iter() {
                match arg {
                    MCJvmArg::JvmArg(string) => final_args.push(string.to_string()),
                    MCJvmArg::JvmRule(rule) => {
                        if rule.rules.iter().all(MCRule::applies) {
                            match &rule.value {
                                MCValue::String(string) => final_args.push(string.to_string()),
                                MCValue::StringList(string_list) => final_args.append(&mut string_list.clone()),
                            }
                        }
                    }
                }
            }
        }
        final_args
    }
    pub fn get_game_args(&self) -> Vec<String> {
        Vec::new()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MCVersionList {
    latest: MCLatest,
    versions: Vec<MCCompactVersion>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCCompactVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String,
    pub compliance_level: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLatest {
    release: String,
    snapshot: String
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCExtendedVersion {
    arguments: Option<MCArguments>,
    minecraft_arguments: Option<String>,
    asset_index: MCAssetIndex,
    assets: String,
    compliance_level: u16,
    downloads: MCDownloads,
    id: String,
    java_version: MCJavaVersion,
    libraries: Vec<MCLibrary>,
    main_class: String,
    minimum_launcher_version: u16,
    release_time: String,
    time: String,
    logging: Option<MCLogging>,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCArguments {
    pub game: Vec<MCGameArg>,
    pub jvm: Vec<MCJvmArg>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCGameArg {
    GameArg(String),
    GameRule(MCGameRule)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCJvmArg {
    JvmArg(String),
    JvmRule(MCJvmRule)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCGameRule {
    rules: Vec<MCRule>,
    value: MCValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCJvmRule {
    rules: Vec<MCRule>,
    value: MCValue
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCValue {
    String(String),
    StringList(Vec<String>)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCAssetIndex {
    id: String,
    url: String,
    size: u64,
    total_size: u64,
    sha1: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCDownloads {
    client: MCDownload,
    client_mappings: Option<MCDownload>,
    server: MCDownload,
    server_mappings: Option<MCDownload>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCDownload {
    url: String,
    size: u64,
    sha1: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCJavaVersion {
    component: String,
    major_version: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLogging {
    client: MCLoggingClient
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLoggingClient {
    argument: String,
    file: MCLoggingClientFile,
    #[serde(rename = "type")]
    typ: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLoggingClientFile {
    id: String,
    size: u64,
    url: String,
    sha1: String,
}