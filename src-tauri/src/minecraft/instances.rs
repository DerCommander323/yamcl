use std::fs::{DirEntry, read_to_string, write};

use chrono::NaiveDateTime;
use configparser::ini::Ini;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{Manager, AppHandle};

use super::modloaders::modloaders::{self, ModLoaders};

#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleInstance {
    pub name: String,
    pub icon: String,
    pub path: String,
    pub id: u32,
    pub mc_version: String,
    pub modloader: ModLoader,
    pub last_played: Option<NaiveDateTime>,
    pub instance_type: InstanceType
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModLoader {
    pub name: String,
    pub typ: modloaders::ModLoaders,
    pub version: String
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InstanceType {
    CurseForge,
    MultiMC
}


pub fn handle_instance_cf(dir: DirEntry, app_handle: AppHandle) {
    let file = read_to_string(dir.path().join("minecraftinstance.json")).unwrap_or(String::from(""));
    let json: Value = serde_json::from_str(&file).unwrap();

    emit_instance_create(app_handle, SimpleInstance {
        name: json["name"].as_str().unwrap_or("Name not found!").to_string(),
        icon: {
            let mut icon = json["installedModpack"]["thumbnailUrl"].as_str().unwrap_or("").to_string();
            if icon.is_empty() {
                icon = "curse:".to_owned()+&json["installedModpack"]["addonID"].as_u64().unwrap_or(666).to_string()
            }
            icon
        },
        path: dir.path().to_string_lossy().to_string(),
        id: get_or_create_instance_id(dir),
        mc_version: json["gameVersion"].as_str().unwrap_or("Name not found!").to_string(),
        modloader: {
            let loader = &json["baseModLoader"];
            if loader.is_object() {
                let loader_enum = 
                    if let Some(l) = 
                        if let Some(name) = 
                            loader["name"].as_str() {
                                modloaders::from_cf(name)
                            } else { None }
                        {
                            l
                        } else { ModLoaders::Vanilla };
                ModLoader {
                    name: loader_enum.to_string(),
                    typ: loader_enum,
                    version: loader["forgeVersion"].as_str().unwrap_or("Unknown version").into()
                }
            } else {
                ModLoader {
                    name: "Vanilla".into(),
                    typ: ModLoaders::Vanilla,
                    version: "".into(),
                }
            }
        },
        last_played: if let Some(string) = json["lastPlayed"].as_str() {
            let time = NaiveDateTime::parse_and_remainder(string, "%Y-%m-%dT%H:%M:%S").unwrap().0;
            if time.timestamp() < 5 { None } else { Some(time) }
        } else { None },
        instance_type: InstanceType::CurseForge,
    }.into())
}

pub fn handle_instance_mmc(dir: DirEntry, app_handle: AppHandle) {
    let mut config_file = if let Ok(contents) = read_to_string(dir.path().join("instance.cfg")) { contents } else { String::from("") };
    config_file = config_file.replace("[General]", "");
    
    let pack_file = if let Ok(contents) = read_to_string(dir.path().join("mmc-pack.json")) { contents } else { String::from("") };
    let json: Value = serde_json::from_str(&pack_file).unwrap();
    let components = json["components"].as_array().unwrap();

    let mut config = Ini::new();
    config.read(config_file).unwrap();

    emit_instance_create(app_handle, SimpleInstance {
        name: if let Some(name) = config.get("default","name") { name } else { String::from("Instance name not found!") },
        icon: if let Some(icon) = config.get("default","iconKey") { icon } else { String::from("default") },
        path: dir.path().join(".minecraft").to_string_lossy().to_string(),
        id: get_or_create_instance_id(dir),
        mc_version: {
            components.iter().filter(|ver| {
                if let Some(uid) = ver["uid"].as_str() {
                    uid == "net.minecraft"
                } else { true }
            }).collect::<Vec<&Value>>()[0]["version"].as_str().unwrap_or("fallback version").to_string()
        },
        modloader: {
            let loader = components.iter().find(|&ver| {
                if let Some(uid) = ver["uid"].as_str() {
                    modloaders::from_uid(uid).is_some()
                } else { false }
            });
            if let Some(loader) = loader {
                ModLoader { 
                    name: loader["cachedName"].as_str().unwrap_or("Unknown loader").into(),
                    typ: modloaders::from_uid(loader["uid"].as_str().unwrap_or("")).unwrap_or(ModLoaders::Vanilla),
                    version: loader["version"].as_str().unwrap_or("Unknown version").into()
                }
            } else {
                ModLoader {
                    name: "Vanilla".into(),
                    typ: ModLoaders::Vanilla,
                    version: "".into(),
                }
            }
        },
        last_played: if let Some(last_launch_time) = config.get("default","lastLaunchTime") {
            if let Ok(last_played) = last_launch_time.parse::<u64>() {
                NaiveDateTime::from_timestamp_millis(last_played.try_into().unwrap())
            } else { None }
        } else { None },
        instance_type: InstanceType::MultiMC
    });
}

fn get_or_create_instance_id(dir: DirEntry) -> u32 {
    let file = read_to_string(dir.path().join("yamcl-data.json")).unwrap_or(String::from(""));
    let rand = fastrand::u32(..);

    if file.is_empty() || !serde_json::from_str::<Value>(&file).unwrap().is_object() {
        let mut json = serde_json::json!({});
        json["instanceID"] = Value::from(rand);
        write(dir.path().join("yamcl-data.json"), serde_json::to_string(&json).unwrap()).unwrap();
        rand    
    } else {
        let mut json: Value = serde_json::from_str(&file).unwrap();
        let id: u32 = json["instanceID"].as_u64().unwrap_or(0).try_into().unwrap_or(0);
        if id==0 {
            json["instanceID"] = Value::from(rand);
            write(dir.path().join("yamcl-data.json"), serde_json::to_string(&json).unwrap()).unwrap();
            rand
        } else {
            id
        }
    }
}


fn emit_instance_create(app_handle: AppHandle, instance: SimpleInstance) {
    debug!("{:?} - {} | Version: {:?}", &instance.instance_type, &instance.name, &instance.modloader);

    app_handle.emit_all("instance_create", instance).unwrap()
}