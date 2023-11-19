use std::process::Command;

use log::{info, debug, warn};
use reqwest::Client;
use tauri::AppHandle;

use crate::{auth::get_active_account, minecraft::{launching::mc_structs::MCVersionManifest, modloaders::{modloaders::{ModLoaders, LoaderManifests}, fabric::FabricVersionManifest}}, authentication::auth_structs::MCAccount, notify, NotificationState};

use super::mc_structs::MCVersionDetails;

#[derive(Debug)]
struct Args {
    jvm: Vec<String>,
    game: Vec<String>,
    main_class: String
}

#[tauri::command(async)]
pub async fn launch_instance(
    minecraft_path: String,
    version_id: String,
    loader_version: String,
    loader: ModLoaders,
    java_path: String,
    additional_args: String,
    instance_id: u32,
    app_handle: AppHandle
) -> Result<(), String> {
    info!("Launching: {minecraft_path}, Version: {version_id}, id: {instance_id}");

    let args = get_arguments(version_id, loader, loader_version, minecraft_path.clone()).await?;

    debug!("Args: {:#?}\nCustom Args: {}", args, additional_args);
    info!("Launching NOW!");

    let mut process = Command::new(java_path)
    .current_dir(&minecraft_path)
    .args(additional_args.split_whitespace())
    .args(args.jvm)
    .arg(args.main_class)
    .args(args.game)
    .spawn()
    .or_else(|err| Err(format!("Failed to run Minecraft command: {}", err.to_string())))?;

    notify(&app_handle, &format!("{}_status", instance_id), "Instance launched successfully!", NotificationState::Success);

    let exit_status = process.wait().expect("Failed to wait on Java process! How did this happen?");
    info!("Exited with status: {}", exit_status);

    if exit_status.success() {
        info!("{minecraft_path} exited successfully.");
        notify(&app_handle, &format!("{}_status", instance_id), "Instance exited successfully.", NotificationState::Success);
    } else {
        warn!("{minecraft_path} exited (crashed) with status {}", exit_status);
        notify(&app_handle, &format!("{}_status", instance_id), &format!("Instance crashed with code {}", exit_status.code().unwrap_or(323)), NotificationState::Error);
    }

    Ok(())
}

async fn get_arguments(version_id: String, loader: ModLoaders, loader_version: String, minecraft_path: String) -> Result<Args, String> {
    let client = Client::new();
    let account = get_active_account()
        .or(Err("Could not get the selected account!".to_string()))?;

    info!("Getting compact version info for {version_id}");
    let compact_version = MCVersionDetails::from_id(version_id.clone(), &client)
        .await
        .ok_or("Could not get compact Minecraft version details!".to_string())?;

    debug!("Got compact version info: {:?}", compact_version);
    info!("Getting extended version info from {}", compact_version.url);

    let mut version = compact_version.get_manifest(&client)
        .await
        .ok_or("Could not get extended Minecraft version details!".to_string())?;

    match loader {
        ModLoaders::Forge => todo!(),
        ModLoaders::Fabric => if let Some(mf) = FabricVersionManifest::get(version_id, loader_version, &client).await {
            version.merge_with(LoaderManifests::Fabric(mf))
        },
        _ => {},
    }

    debug!("Got extended version info. (not listed due to length)");

    info!("Beginning argument parsing...");
    Ok(
        parse_arguments(
            Args {
                jvm: version.get_jvm_args(&client).await,
                game: version.get_game_args(),
                main_class: version.get_main_class()
            },
            account,
            version,
            minecraft_path,
            &client
        ).await
    )
}

async fn parse_arguments(args_struct: Args, account: MCAccount, version: MCVersionManifest, minecraft_path: String, client: &Client) -> Args {
    let replacements = vec![
        ("${auth_player_name}", account.mc_profile.name),
        ("${auth_uuid}", account.mc_profile.id),
        ("${auth_access_token}", account.mc_response.access_token),
        ("${auth_xuid}", account.xsts_response.display_claims.xui[0].uhs.to_string()), // idk what else a "xuid" could be
        ("${user_properties}", "something".to_string()),

        ("${classpath}", version.get_classpath(client).await),
        ("${assets_root}", version.get_client_assets(client).await),
        ("${version_name}", version.id.replace(' ', "_").replace(':', "_")),
        ("${assets_index_name}", version.asset_index.id),
        ("${version_type}", version.typ),

        ("${natives_directory}", format!("{minecraft_path}/natives")),
        ("${launcher_name}", "yamcl".to_string()),
        ("${launcher_version}", "323".to_string()),
        ("${game_directory}", minecraft_path),
        ("${user_type}", "msa".to_string()),
        ("${resolution_width}", 1200.to_string()),
        ("${resolution_height}", 800.to_string()),
    ];

    let to_remove = vec![
        "quickPlay",
        "--demo"
    ];

    let args_final: (Vec<String>, Vec<String>) = [args_struct.jvm, args_struct.game].map(|args| {
        args.into_iter().map(|mut arg| {
            for replacement in &replacements {
                arg = arg.replace(replacement.0, &replacement.1)
            }
            arg
        }).filter(|arg| {
            !to_remove.iter().any(|remover| arg.contains(remover))
        }).collect()
    }).into();

    Args {
        jvm: args_final.0,
        game: args_final.1,
        main_class: args_struct.main_class
    }
}