use std::{path::PathBuf, str::FromStr};

use log::info;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use tokio::fs;

use crate::{configuration::settings::AppSettings, download_file_checked};

use super::{errors::InstanceGatherError, instances::{IResult, META_FILENAME}};


// Handling the "minecraftinstance.json" file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CFInstance {
    pub last_played: String,
    pub name: String,
    pub game_version: String,
    pub base_mod_loader: Option<CFBaseLoader>,
    pub installed_modpack: Option<CFInstalledPack>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CFBaseLoader {
    #[serde(rename = "forgeVersion")]
    pub version: String,
    pub minecraft_version: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CFInstalledPack {
    pub thumbnail_url: Option<String>,
    pub addon_i_d: u64
}

#[derive(Debug, Serialize, Deserialize)]
struct CFProject {
    icon_url: String
}


impl CFInstance {
    pub async fn get(instance_path: &PathBuf) -> IResult<Self> {
        let path = instance_path.join("minecraftinstance.json");
        let pack_file = fs::read(&path).await.map_err(
            |err| InstanceGatherError::FileReadFailed(path.clone(), err)
        )?;

        serde_json::from_slice(&pack_file).map_err(
            |err| InstanceGatherError::ParseFailed(path, err)
        )
    }

    async fn download_icon(instance_path: &PathBuf) -> IResult<Option<String>> {
        let instance = Self::get(instance_path).await?;
        if let Some(path) = AppSettings::get().icon_path {
            let file = PathBuf::from_str(&path).map_err(
                |err| InstanceGatherError::IconPathParseFailed(path, err)
            )?.join(format!("curseforge_{}", fastrand::u32(..)));
    
            if let Some(pack) = instance.installed_modpack {
                let client = Client::new();
                if let Some(url) = pack.thumbnail_url {
                    download_file_checked(&client, None, &file, &url).await;
                    Ok(Some(file.to_string_lossy().to_string()))
                } else {
                    info!("Requesting icon for project {}", pack.addon_i_d);
                    let project: Result<CFProject, reqwest::Error> = client
                    .get(format!("https://curserinth-api.kuylar.dev/v2/project/{}", pack.addon_i_d))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await;

                    if let Ok(proj) = project {
                        download_file_checked(&client, None, &file, &proj.icon_url).await;
                        Ok(Some(file.to_string_lossy().to_string()))
                    } else { Ok(None) }
                }
            } else { Ok(None) }
        } else { Ok(None) }
    }
}

// Handling our metadata ("yamcl-data.json" file)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CFMetadata {
    pub instance_id: u32,
    pub saved_icon: String
}

impl CFMetadata {
    pub async fn get(instance_path: &PathBuf) -> IResult<Self> {
        let file = fs::read(instance_path.join(META_FILENAME)).await.map_err(
            |err| InstanceGatherError::FileReadFailed(instance_path.clone(), err)
        )?;

        let result = match serde_json::from_slice(&file) {
            Ok(parsed) => Ok(parsed),
            Err(_) => Ok(Self::generate(instance_path).await?),
        };

        if result.as_ref().is_ok_and(
            |meta| PathBuf::from_str(&meta.saved_icon).map_or(
                true, |icon| !icon.exists()
            )
        ) {
            Self::generate(instance_path).await
        } else {
            result
        }
    }

    async fn generate(instance_path: &PathBuf) -> IResult<Self> {
        let path = instance_path.join(META_FILENAME);

        let meta = CFMetadata {
            instance_id: fastrand::u32(..),
            saved_icon: CFInstance::download_icon(instance_path).await?.unwrap_or("default_instance.png".to_string())
        };

        fs::write(&path, serde_json::to_string_pretty(&meta).unwrap()).await.map_err(
            |err| InstanceGatherError::FileWriteFailed(path, err)
        )?;

        Ok(meta)
    }
}