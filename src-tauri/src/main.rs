//Prevents console window on windows
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::{self, create_dir_all}, path::{Path, PathBuf}, io::Cursor};
use log::debug;
use reqwest::Client;
use sha1_smol::Sha1;
use simple_logger::SimpleLogger;
use tauri::{AppHandle, Manager, api::path::data_dir};
use authentication::{auth, auth_structs};
use minecraft::{launching, java};

mod minecraft {
    pub mod launching {
        pub mod launching;
        pub mod versions;
        pub mod libraries;
        pub mod mc_structs;
    }
    pub mod modloaders {
        pub mod modloaders;
        pub mod fabric;
    }
    pub mod java;
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
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .expect("Failed to initialize logger!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_instances,
            unlock_icons,
            file_exists,
            launching::launching::launch_instance,
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
    app_handle.asset_protocol_scope().allow_directory(Path::new(&path), false).unwrap();
}

#[tauri::command()]
fn file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command(async)]
fn get_instances(path: String, app_handle: AppHandle) {
    let paths = fs::read_dir(path).unwrap();
    let mut instance_count: u32 = 0;

    for path in paths {
        if path.as_ref().unwrap().file_type().unwrap().is_dir() {
            let instance_folder = path.unwrap();

            let instance_contents = fs::read_dir(instance_folder.path()).unwrap();
            
            // TODO: Use much faster checking using path.exists(), and multithreading
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
        &format!("notification_{name}"),
        Notification { text: text.to_string(), status }
    ).unwrap()
}

/// Checks if the checksum of the file at `path` matches `checksum` and downloads it from `url` if not.
pub async fn download_file_checked(client: &Client, checksum: &String, path: &PathBuf, url: &String) {
    if !path.is_file() || {
        if let Ok(contents) = fs::read(&path) {
            let contents_checksum = Sha1::from(contents).digest().to_string();
            &contents_checksum != checksum
        } else { true }
    } {
        download_file(client, path, url).await
    } else {
        debug!("Skipped downloading {}", path.to_string_lossy())
    }
}

async fn download_file(client: &Client, path: &PathBuf, url: &String) {
    debug!("Downloading to {} from {url}", path.to_string_lossy());
    let response = client.get(url).send().await.unwrap();
    if let Some(parent_path) = path.parent() {
        create_dir_all(parent_path).expect(&format!("Failed to create directories: {}", parent_path.to_string_lossy()));
    }
    let mut file = std::fs::File::create(path).expect(&format!("Failed create file: {}", path.to_string_lossy()));
    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).expect(&format!("Failed to write to {}", path.to_string_lossy()));
}

pub fn get_data_dir() -> PathBuf { data_dir().unwrap().join("yamcl") }
pub fn get_client_jar_dir() -> PathBuf { get_data_dir().join("client_jars") }
pub fn get_library_dir() -> PathBuf { get_data_dir().join("libraries") }
pub fn get_assets_dir() -> PathBuf { get_data_dir().join("assets") }
pub fn get_log4j_dir() -> PathBuf { get_data_dir().join("log4j_configs") }