#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Instance {
    id: i32,
    name: String,
    desc: String
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_instances])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_instances() -> Vec<Instance> {
    let path = "../data/instances.json";

    let instances: Vec<Instance> = serde_json::from_str(path).unwrap();
    instances
}


