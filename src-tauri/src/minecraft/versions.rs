use std::{iter, path::PathBuf};

use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::{get_client_jar_dir, download_file_checked, get_log4j_dir};

use super::libraries::{MCLibrary, MCRule};

const VERSION_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";


impl MCVersionList {
    pub async fn get(client: &Client) -> Option<Self> {
        let version_list: Result<MCVersionList, reqwest::Error> = client.get(VERSION_URL).send().await.unwrap().json().await;
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
    pub async fn from_id(version_id: String, client: &Client) -> Option<Self> {
        let version_list = MCVersionList::get(client).await?;
        version_list.versions.into_iter().find(|ver| {
            ver.id == version_id
        })
    }
}

impl MCExtendedVersion {
    pub async fn from_compact(compact_version: MCCompactVersion, client: &Client) -> Option<Self> {
        let extended_version: Result<MCExtendedVersion, reqwest::Error> = client.get(compact_version.url).send().await.unwrap().json().await;
        match extended_version {
            Ok(ver) => Some(ver),
            Err(e) => {
                println!("Failed to get extended Minecraft version info: {e}");
                None
            }
        }
    }

    pub async fn get_jvm_args(&self, client: &Client) -> Vec<String> {
        let mut final_args: Vec<String> = Vec::new();

        if let Some(args) = self.arguments.as_ref() {
            for arg in args.jvm.iter() {
                match arg {
                    MCJvmArg::JvmArg(string) => final_args.push(string.to_string()),
                    MCJvmArg::JvmRule(rule) => {
                        if rule.rules.iter().all(MCRule::applies) {
                            match &rule.value {
                                MCValue::String(string) => final_args.push(string.to_string()),
                                MCValue::StringList(string_list) => final_args.append(&mut string_list.clone())
                            }
                        }
                    }
                }
            }
        }

        if !final_args.iter().any(|arg| arg.contains("-cp")) {
            final_args.append(&mut vec!["-cp".to_string(), "${classpath}".to_string()])
        }

        if let Some(config) = self.get_log4j_config(client).await {
            final_args.push(config.0.replace("${path}", &config.1.to_string_lossy().to_string()))
        }

        final_args
    }

    pub fn get_game_args(&self) -> Vec<String> {
        let mut final_args: Vec<String> = Vec::new();

        match &self.arguments {
            Some(args) => {
                for arg in args.game.iter() {
                    match arg {
                        MCGameArg::GameArg(string) => final_args.push(string.to_string()),
                        MCGameArg::GameRule(rule) => {
                            if rule.rules.iter().all(MCRule::applies) {
                                match &rule.value {
                                    MCValue::String(string) => final_args.push(string.to_string()),
                                    MCValue::StringList(string_list) => final_args.append(&mut string_list.clone()),
                                }
                            }
                        }
                    }
                }
            },
            None => if let Some(args_string) = &self.minecraft_arguments {
                let mut args: Vec<String> = args_string.split_whitespace().map(String::from).collect();
                final_args.append(&mut args);
            } else {
                panic!("No arguments found in this version manifest???")
            }
        }

        final_args
    }

    pub async fn get_classpath(&self, client: &Client) -> String {
        let libraries: Vec<&MCLibrary> = self.libraries
            .iter()
            .filter(|&lib| if let Some(rules) = &lib.rules {
                rules.iter().all(MCRule::applies)
            } else { true })
            .collect();

        for lib in &libraries {
            lib.download_checked(&client).await
        }

        libraries.iter()
            .map(|&lib| {
                lib.get_path().to_string_lossy().to_string()
            })
            .chain(iter::once(
                self.get_client_jar(&client).await.to_string_lossy().to_string()
            ))
            .collect::<Vec<String>>()
            .join(if cfg!(windows) { ";" } else { ":" })
    }

    pub fn get_main_class(&self) -> String {
        self.main_class.to_string()
    }

    pub async fn get_client_jar(&self, client: &Client) -> PathBuf {
        let path = get_client_jar_dir().join(format!("{}.jar", self.id));
        download_file_checked(
            client,
            &self.downloads.client.sha1,
            &path,
            &self.downloads.client.url
        ).await;
        path
    }

    pub async fn get_log4j_config(&self, client: &Client) -> Option<(String, PathBuf)> {
        if let Some(logging) = &self.logging {
            let path = get_log4j_dir().join(&logging.client.file.id);
            download_file_checked(
                client,
                &logging.client.file.sha1,
                &path,
                &logging.client.file.url
            ).await;
            Some((logging.client.argument.to_string(), path))
        } else { None }
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
    pub arguments: Option<MCArguments>,
    pub minecraft_arguments: Option<String>,
    pub asset_index: MCAssetIndex,
    pub assets: String,
    pub compliance_level: u16,
    pub downloads: MCDownloads,
    pub id: String,
    pub java_version: MCJavaVersion,
    pub libraries: Vec<MCLibrary>,
    pub main_class: String,
    pub minimum_launcher_version: u16,
    pub release_time: String,
    pub time: String,
    pub logging: Option<MCLogging>,
    #[serde(rename = "type")]
    pub typ: String,
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
    pub id: String,
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