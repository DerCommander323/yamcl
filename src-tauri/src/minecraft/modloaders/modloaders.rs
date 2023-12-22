use std::fmt;

use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::minecraft::modloaders::forge_installer::ForgeInstaller;

use super::{fabric::FabricVersionManifest, forge::ForgeVersionManifest};


pub enum LoaderManifests {
    Fabric(FabricVersionManifest),
    Forge(ForgeVersionManifest)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModLoaders {
    Vanilla,
    Forge,
    NeoForge,
    Fabric,
    Quilt,
    LiteLoader, // not that important for now
    Rift // neither is this one
}

impl ModLoaders {
    pub async fn prepare_launch(&self, mc_ver: &str, forge_ver: &str, client: &Client, java_path: &str) {
        match self {
            ModLoaders::Forge => {
                info!("Preparing launch with Forge...");
                ForgeInstaller::prepare_jar(mc_ver, forge_ver, client, java_path).await;
            },
            _ => {}
        }
    }

    pub async fn get_manifest(&self, mc_ver: &str, loader_ver: &str, client: &Client) -> Option<LoaderManifests> {
        match self {
            ModLoaders::Forge => ForgeVersionManifest::get(mc_ver, loader_ver, client).await.map(|mf| LoaderManifests::Forge(mf)),
            ModLoaders::Fabric => FabricVersionManifest::get(mc_ver, loader_ver, client).await.map(|mf| LoaderManifests::Fabric(mf)),
            _ => None,
        }
    }

    pub fn from_uid(mmc_uid: &str) -> Option<Self> {
        MMC_LOADERS.iter().find(|&loader| {
            loader.0 == mmc_uid
        }).map_or(None, |v| Some(v.1))
    }
    
    pub fn from_cf(cf_name: &str) -> Option<Self> {
        if let Some(name) = cf_name.split('-').nth(0) {
            STRING_LOADERS.iter().find(|&loader| {
                loader.0 == name
            }).map_or(None, |v| Some(v.1))
        } else {
            None
        }
    }
}

impl fmt::Display for ModLoaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const MMC_LOADERS: [(&str, ModLoaders); 5] = [
    ("net.minecraftforge", ModLoaders::Forge),
    ("net.neoforged", ModLoaders::NeoForge),
    ("net.fabricmc.fabric-loader", ModLoaders::Fabric),
    ("something", ModLoaders::Quilt),
    ("something2", ModLoaders::LiteLoader),
];
const STRING_LOADERS: [(&str, ModLoaders); 2] = [
    ("forge", ModLoaders::Forge),
    ("fabric", ModLoaders::Fabric),
];

