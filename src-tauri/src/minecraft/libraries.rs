use serde::{Deserialize, Serialize};
use serde_json::Value;


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
    downloads: MCLibraryDownloads,
    name: String,
    rules: Option<Vec<MCRule>>,
    natives: Option<Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLibraryDownloads {
    artifact: Option<MCLibraryDownloadsArtifacts>,
    classifiers: Option<Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCLibraryDownloadsArtifacts {
    path: String,
    url: String,
    size: u32,
    sha1: String
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