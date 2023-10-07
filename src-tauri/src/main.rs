//Prevents console window on windows
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::{self}, path::Path};
use tauri::{AppHandle, Manager};

mod instances;
mod minecraft;
mod authentication;
mod auth_structs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_instances,
            unlock_icons,
            file_exists,
            minecraft::launch_instance,
            minecraft::get_java_version,
            authentication::get_selected_account,
            authentication::set_selected_account,
            authentication::remove_account,
            authentication::load_accounts,
            authentication::add_account
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
                    "minecraftinstance.json" => {instances::handle_instance_cf(instance_folder, app_handle.clone()); instance_count += 1; break;},
                    "instance.cfg" => {instances::handle_instance_mmc(instance_folder, app_handle.clone()); instance_count += 1; break;},
                    _ => continue
                }   
            }
        }
    }
    app_handle.emit_all("instance_finish", instance_count).unwrap();
}

