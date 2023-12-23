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
use tauri::{AppHandle, Manager, api::path::{data_dir, config_dir}};
use authentication::auth;
use minecraft::{launching, java};

mod minecraft {
    pub mod launching {
        pub mod launching;
        pub mod manifests;
        pub mod libraries;
        pub mod mc_structs;
    }
    pub mod modloaders {
        pub mod modloaders;
        pub mod fabric;
        pub mod forge;
        pub mod forge_installer;
    }
    pub mod java;
    pub mod instances {
        pub mod instances;
        pub mod errors;
        pub mod curseforge;
        pub mod multimc;
    }
}
mod authentication { 
    pub mod auth;
    pub mod auth_structs;
}
mod configuration {
    pub mod accounts;
    pub mod settings;
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
            unlock_icons,
            file_exists,
            minecraft::instances::instances::get_instances,
            launching::launching::launch_instance,
            java::get_java_version,
            configuration::accounts::get_selected_index,
            configuration::accounts::set_selected_index,
            configuration::accounts::remove_account,
            configuration::accounts::get_accounts,
            configuration::settings::get_settings,
            configuration::settings::update_settings,
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

pub fn notify(app_handle: &AppHandle, name: &str, text: &str, status: NotificationState) {
    app_handle.emit_all(
        &format!("notification_{name}"),
        Notification { text: text.to_string(), status }
    ).unwrap()
}

/// Checks if the checksum of the file at `path` matches `checksum` and downloads it from `url` if not.
pub async fn download_file_checked(client: &Client, checksum: Option<&String>, path: &PathBuf, url: &String) {
    if !path.is_file() || if let Some(csum) = checksum {
        if let Ok(contents) = fs::read(&path) {
            let contents_checksum = Sha1::from(contents).digest().to_string();
            &contents_checksum != csum
        } else { true }
    } else { false }
    {
        download_file(client, path, url).await
    } else {
        debug!("Skipped downloading {}", path.to_string_lossy())
    }
}

async fn download_file(client: &Client, path: &PathBuf, url: &String) {
    debug!("Downloading to {} from {url}", path.to_string_lossy());
    let response = client.get(url).send().await.unwrap();
    if let Some(parent_path) = path.parent() {
        if !parent_path.exists() {
            create_dir_all(parent_path).expect(&format!("Failed to create directories: {}", parent_path.to_string_lossy()));
        }
    }
    let mut file = std::fs::File::create(path).expect(&format!("Failed create file: {}", path.to_string_lossy()));
    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).expect(&format!("Failed to write to {}", path.to_string_lossy()));
}

pub fn maven_identifier_to_path(identifier: &str) -> String {
    let mut id = identifier.to_string();
    let extension = if let Some(i) = identifier.find("@") {
        let ext = &identifier[i..];
        id = id.replace(ext, "");
        &ext[1..]
    } else { "jar" };

    let parts: Vec<&str> = id.splitn(3, ":").collect();
    let (raw_path, raw_name, raw_version) = (parts[0], parts[1], parts[2]);

    let path = raw_path.replace(".", "/");
    let version_path = raw_version.split(":").nth(0).unwrap_or(raw_version);
    let version = raw_version.replace(":", "-");

    format!("{path}/{raw_name}/{version_path}/{raw_name}-{version}.{extension}")
}

pub fn get_classpath_separator() -> String { String::from(if cfg!(windows) { ";" } else { ":" }) }

pub fn get_config_dir() -> PathBuf { config_dir().unwrap().join("yamcl") }
pub fn get_data_dir() -> PathBuf { data_dir().unwrap().join("yamcl") }

pub fn get_client_jar_dir() -> PathBuf { get_data_dir().join("client_jars") }
pub fn get_library_dir() -> PathBuf { get_data_dir().join("libraries") }
pub fn get_assets_dir() -> PathBuf { get_data_dir().join("assets") }
pub fn get_log4j_dir() -> PathBuf { get_data_dir().join("log4j_configs") }

pub fn get_forge_cache_dir() -> PathBuf { get_data_dir().join("forge_cache") }