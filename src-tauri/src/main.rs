//Prevents console window on windows
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::{self}, path::Path};
use tauri::{AppHandle, Manager};
use authentication::{auth, auth_structs};
use minecraft::{launching, java};

mod minecraft {
    pub mod launching;
    pub mod java;
    pub mod rules;
    pub mod modloaders;
    pub mod instances;
}
mod authentication { 
    pub mod auth;
    pub mod auth_structs;
}

#[derive(serde::Serialize, Clone)]
struct Notification {
    status: NotificationState,
    text: String
}
#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NotificationState {
    Running,
    Error,
    Success
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_instances,
            unlock_icons,
            file_exists,
            launching::launch_instance,
            java::get_java_version,
            auth::get_selected_index,
            auth::set_selected_index,
            auth::remove_account,
            auth::load_accounts,
            auth::add_account
        ])
        .run(tauri::generate_context!())
        .expect("Failed to start YAMCL!");
}

#[tauri::command]
fn unlock_icons(path: String, app_handle: AppHandle) {
    app_handle.asset_protocol_scope().allow_directory(Path::new(&path), true).unwrap();
}

#[tauri::command()]
fn file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command(async)]
fn get_instances(path: String, app_handle: AppHandle) {
    let paths = fs::read_dir(path).unwrap();
    let mut instance_count: u16 = 0;

    for path in paths {
        if path.as_ref().unwrap().file_type().unwrap().is_dir() {
            let instance_folder = path.unwrap();

            let instance_contents = fs::read_dir(instance_folder.path()).unwrap();

            for file in instance_contents {
                match file.unwrap().file_name().into_string().unwrap().as_ref() {
                    "minecraftinstance.json" => {minecraft::instances::handle_instance_cf(instance_folder, app_handle.clone()); instance_count += 1; break;},
                    "instance.cfg" => {minecraft::instances::handle_instance_mmc(instance_folder, app_handle.clone()); instance_count += 1; break;},
                    _ => continue
                }   
            }
        }
    }
    app_handle.emit_all("instance_finish", instance_count).unwrap();
}

pub fn notify(app_handle: &AppHandle, name: &str, text: &str, status: NotificationState) {
    app_handle.emit_all(
        &String::from_iter(["notification_", name]),
        Notification { text: text.to_string(), status }
    ).unwrap()
}