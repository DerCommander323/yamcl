use std::path::PathBuf;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{download_file_checked, get_library_dir};

impl MCLibrary {
    pub fn get_path(&self) -> PathBuf {
        if let Some(artifact) = &self.downloads.artifact {
            get_library_dir().join(&artifact.path)
        } else if let Some(classifiers) = &self.downloads.classifiers {
            let natives = if cfg!(windows) {
                &classifiers.natives_windows
            } else if cfg!(macos) {
                &classifiers.natives_osx
            } else {
                &classifiers.natives_linux // in the hopes of these natives working on platforms like OpenBSD too (probably not)
            };
            get_library_dir().join(&natives.as_ref().unwrap().path)
        } else {
            PathBuf::new()
        }
    }

    pub async fn download_checked(&self, client: &Client) {
        if let Some(artifact) = &self.downloads.artifact {
            download_file_checked(
                &client,
                &artifact.sha1,
                &self.get_path(),
                &artifact.url
            ).await
        }
    }
}

impl MCRule {
    pub fn applies(&self) -> bool {
        if let Some(os_rule) = &self.os {
            let arch_matches = os_rule.arch.as_ref().map_or(true, |arch| {
                match arch.as_str() {
                    "x86" => cfg!(target_arch = "x86"),
                    "x86_64" => cfg!(target_arch = "x86_64"), // haven't seen this one yet, but might exist; won't hurt to have
                    _ => false
                }
            });

            let os_matches = os_rule.name.as_ref().map_or(true, |os| {
                match os.as_str() {
                    "linux" => cfg!(target_os = "linux"),
                    "osx" => cfg!(target_os = "macos"),
                    "windows" => cfg!(target_os = "windows"),
                    _ => false
                }
            });

            match self.action {
                Action::Allow => arch_matches && os_matches,
                Action::Disallow => !(arch_matches && os_matches) // idk if this is accurate, doesn't even happen though
            }
        } else { true }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLibrary {
    pub downloads: MCLibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<MCRule>>,
    pub natives: Option<Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLibraryDownloads {
    artifact: Option<MCLibraryDownloadsArtifacts>,
    classifiers: Option<MCLibraryDownloadsClassifiers>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLibraryDownloadsArtifacts {
    path: String,
    url: String,
    size: u32,
    sha1: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MCLibraryDownloadsClassifiers {
    natives_linux: Option<MCLibraryDownloadsArtifacts>,
    natives_osx: Option<MCLibraryDownloadsArtifacts>,
    natives_windows: Option<MCLibraryDownloadsArtifacts>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCRule {
    action: Action,
    os: Option<OSRule>,
    features: Option<FeatureFlags>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Allow,
    Disallow
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlags {
    is_demo_user: Option<bool>,
    has_custom_resolution: Option<bool>,
    has_quick_plays_support: Option<bool>,
    is_quick_play_singleplayer: Option<bool>,
    is_quick_play_multiplayer: Option<bool>,
    is_quick_play_realms: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSRule {
    name: Option<String>,
    arch: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureRule {
    name: String
}