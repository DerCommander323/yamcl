use std::process::Command;

use reqwest::blocking::Client;

use crate::{auth::get_active_account, minecraft::versions::MCExtendedVersion};

use super::versions::MCCompactVersion;


#[tauri::command]
pub fn launch_instance(minecraft_path: String, version_id: String, java_path: String) {
    println!("Launching: {minecraft_path}, Version: {version_id}");

    let args = get_arguments(version_id);

    //let mut _process = Command::new(java_path).args(args).spawn().unwrap();
}

fn get_arguments(version_id: String) -> Result<Vec<String>, String> {
    let client = Client::new();
    let account = get_active_account()
        .or(Err("Could not get the selected account!".to_string()))?;

    println!("Getting compact version info for {version_id}");
    let version = MCCompactVersion::from_id(version_id, &client)
        .ok_or("Could not get compact Minecraft version details!".to_string())?;

    println!("Got compact version info: {:?}", version);
    println!("Getting extended version info from {}", version.url);

    let extended_version = MCExtendedVersion::from_compact(version, &client)
        .ok_or("Could not get extended Minecraft version details!".to_string())?;

    println!("Got extended version info. (not listed due to length)");

    let mut args: Vec<String> = Vec::new();

    args.append(&mut extended_version.get_jvm_args());
    args.append(&mut extended_version.get_game_args());
    
    Ok(args)
}