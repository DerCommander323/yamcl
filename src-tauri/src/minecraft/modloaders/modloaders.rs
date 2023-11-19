use std::fmt;

use serde::{Deserialize, Serialize};

use super::fabric::FabricVersionManifest;


pub enum LoaderManifests {
    Fabric(FabricVersionManifest)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModLoaders {
    Vanilla,
    Forge,
    NeoForge,
    Fabric,
    Quilt,
    LiteLoader // not that important for now
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

pub fn from_uid(mmc_uid: &str) -> Option<ModLoaders> {
    MMC_LOADERS.iter().find(|&loader| {
        loader.0 == mmc_uid
    }).map_or(None, |v| Some(v.1))
}
pub fn from_cf(cf_name: &str) -> Option<ModLoaders> {
    if let Some(name) = cf_name.split('-').nth(0) {
        STRING_LOADERS.iter().find(|&loader| {
            loader.0 == name
        }).map_or(None, |v| Some(v.1))
    } else {
        None
    }
}
