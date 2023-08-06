use std::fs::DirEntry;
use std::fs::read_to_string;

use configparser::ini::Ini;
use serde_json::Value;
use tauri::{Manager, AppHandle};

#[derive(Clone, serde::Serialize)]
struct InstanceData {
  name: String,
  icon: String
}


pub fn handle_instance_cf(dir: DirEntry, app_handle: AppHandle) {
    let data = read_to_string(dir.path().join("minecraftinstance.json")).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let name = json["name"].as_str().unwrap_or("Name not found!").to_string();

    println!("Curseforge: {}", &name);
    emit_instance_create(InstanceData { name, icon: "".to_string() }.into(), app_handle)
}

pub fn handle_instance_mmc(dir: DirEntry, app_handle: AppHandle) {
    let mut data = read_to_string(dir.path().join("instance.cfg")).unwrap();
    data = data.replace("[General]", "");

    let mut config = Ini::new();
    config.read(data).unwrap();

    let name = config.get("default","name").unwrap();
    let icon = config.get("default","iconKey").unwrap();

    println!("MultiMC: {}, Icon: {}", &name, &icon);
    emit_instance_create(InstanceData { name, icon }.into(), app_handle)
}

fn emit_instance_create(data: InstanceData, app_handle: AppHandle) {
    app_handle.emit_all("instance_create", data).unwrap()
}