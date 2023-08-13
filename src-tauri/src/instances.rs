use std::fs::DirEntry;
use std::fs::read_to_string;
use std::fs::write;

use configparser::ini::Ini;
use serde_json::Value;
use tauri::{Manager, AppHandle};

#[derive(Clone, serde::Serialize)]
struct InstanceData {
  name: String,
  icon: String,
  path: String,
  id: u32,
  last_played_string: String,
  last_played_epoch: u64
}


pub fn handle_instance_cf(dir: DirEntry, app_handle: AppHandle) {
    let file = read_to_string(dir.path().join("minecraftinstance.json")).unwrap_or(String::from(""));
    let json: Value = serde_json::from_str(&file).unwrap();
    let name = json["name"].as_str().unwrap_or("Name not found!").to_string();
    let mut icon = json["installedModpack"]["thumbnailUrl"].as_str().unwrap_or("").to_string();
    if icon.is_empty() {
        icon = "curse:".to_owned()+&json["installedModpack"]["addonID"].as_u64().unwrap_or(666).to_string()
    }
    let path = dir.path().to_str().unwrap_or("invalid_path").to_string();
    let last_played_string = json["lastPlayed"].as_str().unwrap_or("invalid").to_string();

    let id = get_or_create_instance_id(dir);

    println!("Curseforge: {}, instanceID: {}", &name, &id);
    emit_instance_create(InstanceData { name, icon, path, id, last_played_string, last_played_epoch: 0 }.into(), app_handle)
}

pub fn handle_instance_mmc(dir: DirEntry, app_handle: AppHandle) {
    let mut file = read_to_string(dir.path().join("instance.cfg")).unwrap_or(String::from(""));
    file = file.replace("[General]", "");

    let mut config = Ini::new();
    config.read(file).unwrap();

    let name = config.get("default","name").unwrap_or(String::from("Name not found!"));
    let icon = config.get("default","iconKey").unwrap_or(String::from("default"));
    let path = dir.path().to_str().unwrap_or("invalid_path").to_string();
    let last_played_epoch: u64 = config.get("default","lastLaunchTime").unwrap_or(String::from("1")).parse::<u64>().unwrap_or(0);

    let id = get_or_create_instance_id(dir);

    println!("MultiMC: {}, instanceID: {}", &name, &id);
    emit_instance_create(InstanceData { name, icon, path, id, last_played_string: "".to_string(), last_played_epoch }.into(), app_handle)
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


fn emit_instance_create(data: InstanceData, app_handle: AppHandle) {
    app_handle.emit_all("instance_create", data).unwrap()
}