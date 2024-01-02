use std::path::Path;

use serde::{Serialize, Deserialize};
use tokio::fs;

use super::{errors::InstanceGatherError, instances::{IResult, InstanceType, META_FILENAME}};
use log::error;


// Handling the "instance.cfg" file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MMCConfig {
    pub name: String,
    #[serde(default)] // Make iconKey optional or provide a default value
    pub icon_key: String,
    #[serde(rename = "lastLaunchTime")]
    pub last_played: Option<i64>,
}

impl MMCConfig {
    pub async fn get(path: &Path) -> IResult<Self> {
        let instance_file = fs::read_to_string(path.join("instance.cfg"))
            .await
            .map_err(|err| {
                error!("Failed to read instance.cfg file: {:?}", err);
                InstanceGatherError::FileReadFailed(path.to_path_buf(), err)
            })?
            .replace("[General]", ""); // Remove the section if there is one

        serde_ini::from_str(&instance_file).map_err(|err| {
            error!(
                "Failed to parse instance.cfg file at {:?}: {:?}",
                path, err
            );
            InstanceGatherError::ParseFailedIni(InstanceType::MultiMC, path.to_path_buf(), err)
        })
    }

    pub fn check_icon(icon_key: &str) -> Option<String> {
        let internal_icons = vec![
            "default", "bee", "brick", "chicken", "creeper", "diamond", "dirt", "enderman", "enderpearl", "flame", "fox", "gear", "herobrine",
            "gold", "grass", "iron", "magitech", "meat", "modrinth", "netherstar", "planks", "prismlauncher", "squarecreeper", "steve", 
            "stone", "tnt", "bee_legacy", "brick_legacy", "chicken_legacy", "creeper_legacy", "diamond_legacy", "dirt_legacy",
            "enderman_legacy", "enderpearl_legacy", "flame_legacy", "fox_legacy", "gear_legacy", "herobrine_legacy", "gold_legacy", "grass_legacy", 
            "iron_legacy", "magitech_legacy", "meat_legacy", "modrinth_legacy", "netherstar_legacy", "planks_legacy", "prismlauncher_legacy",
            "squarecreeper_legacy", "steve_legacy", "stone_legacy", "tnt_legacy"
        ]; // We can't display those since they're built into MultiMC

        if internal_icons.iter().any(|&i| icon_key == i) {
            None
        } else {
            Some(icon_key.to_string())
        }
    }
}


// Handling the "mmc-instance.json" file
#[derive(Debug, Deserialize)]
pub struct MMCPack {
    pub components: Vec<MMCPackComponent>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct MMCPackComponent {
    pub cached_name: String,
    pub cached_version: Option<String>,
    pub dependency_only: Option<bool>,
    pub uid: String,
    pub version: Option<String>
}

impl MMCPack {
    pub async fn get(instance_path: &Path) -> IResult<Self> {
    let path = instance_path.join("mmc-pack.json");
    let pack_file = fs::read(&path)
        .await
        .map_err(|err| {
            error!("Failed to read mmc-pack.json file: {:?}", err);
            InstanceGatherError::FileReadFailed(path.clone(), err)
        })?;

    serde_json::from_slice(&pack_file).map_err(|err| {
        error!(
            "Failed to parse mmc-pack.json file at {:?}: {:?}",
            path, err
        );
        InstanceGatherError::ParseFailed(path, err)
    })
    }
}

// Handling our metadata ("yamcl-data.json" file)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MMCMetadata {
    pub instance_id: u32
}

impl MMCMetadata {
    pub async fn get(instance_path: &Path) -> IResult<Self> {
        let file_path = instance_path.join(META_FILENAME);

        // Check if the file exists before reading it
        if file_path.exists() {
            // File exists, attempt to read it
            let file_content = fs::read(&file_path)
                .await
                .map_err(|err| {
                    error!("Failed to read yamcl-data.json file: {:?}", err);
                    InstanceGatherError::FileReadFailed(instance_path.to_path_buf(), err)
                })?;

            // Deserialize the file content into MMCMetadata
            match serde_json::from_slice(&file_content) {
                Ok(parsed) => Ok(parsed),
                Err(_) => {
                    error!("Failed to parse yamcl-data.json file at {:?}", instance_path);
                    Ok(Self::generate(&file_path).await?)
                }
            }
        } else {
            // File doesn't exist, generate it
            Self::generate(&file_path).await
        }
    }

    async fn generate(file_path: &Path) -> IResult<Self> {
        let meta = MMCMetadata {
            instance_id: fastrand::u32(..),
        };

        // Serialize the metadata and write it to the file
        let serialized = serde_json::to_string_pretty(&meta).map_err(|err| {
            error!("Failed to serialize metadata: {:?}", err);
            InstanceGatherError::SerializationFailed(err)
        })?;

        fs::write(file_path, serialized)
            .await
            .map_err(|err| InstanceGatherError::FileWriteFailed(file_path.to_path_buf(), err))?;

        Ok(meta)
    }

}