use std::fs::DirEntry;
use std::fs::read_to_string;

use configparser::ini::Ini;
use serde_json::Value;
use tauri::{Manager, AppHandle};

#[derive(Clone, serde::Serialize)]
struct InstanceData {
  name: String,
  icon: String,
  path: String
}


pub fn handle_instance_cf(dir: DirEntry, app_handle: AppHandle) {
    let data = read_to_string(dir.path().join("minecraftinstance.json")).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let name = json["name"].as_str().unwrap_or("Name not found!").to_string();
    let mut icon = json["installedModpack"]["thumbnailUrl"].as_str().unwrap_or("").to_string();
    if icon.is_empty() {
        icon = "curse:".to_owned()+&json["installedModpack"]["addonID"].as_u64().unwrap_or(666).to_string()
    }
    let path = dir.path().to_str().unwrap_or("invalid_path").to_string();

    println!("Curseforge: {}, Icon: {}", &name, &icon);
    emit_instance_create(InstanceData { name, icon, path }.into(), app_handle)
}

pub fn handle_instance_mmc(dir: DirEntry, app_handle: AppHandle) {
    let mut data = read_to_string(dir.path().join("instance.cfg")).unwrap();
    data = data.replace("[General]", "");

    let mut config = Ini::new();
    config.read(data).unwrap();

    let name = config.get("default","name").unwrap();
    let icon = config.get("default","iconKey").unwrap();
    let path = dir.path().join(".minecraft").to_str().unwrap_or("invalid_path").to_string();

    println!("MultiMC: {}, Icon: {}", &name, &icon);
    emit_instance_create(InstanceData { name, icon, path }.into(), app_handle)
}

fn emit_instance_create(data: InstanceData, app_handle: AppHandle) {
    app_handle.emit_all("instance_create", data).unwrap()
}