use std::process::Command;

use reqwest::blocking::Client;

use crate::{auth::get_active_account, minecraft::versions::MCExtendedVersion, authentication::auth_structs::MCAccount};

use super::versions::MCCompactVersion;

#[derive(Debug)]
struct Args {
    jvm: Vec<String>,
    game: Vec<String>,
    main_class: String
}

#[tauri::command]
pub fn launch_instance(minecraft_path: String, version_id: String, java_path: String) -> Result<(), String> {
    println!("Launching: {minecraft_path}, Version: {version_id}");

    let args = get_arguments(version_id, minecraft_path.clone())?;

    println!("Args: {:#?}", args);
    println!("Launching NOW!");

    let mut process = Command::new(java_path)
    .current_dir(minecraft_path)
    .args(args.jvm)
    .arg(args.main_class)
    .args(args.game)
    .spawn()
    .or_else(|err| Err(format!("Failed to run Minecraft command: {}", err.to_string())))?;

    process.wait().expect("Failed to wait on Java process! How did this happen?");
    Ok(())
}

fn get_arguments(version_id: String, minecraft_path: String) -> Result<Args, String> {
    let client = Client::new();
    let account = get_active_account()
        .or(Err("Could not get the selected account!".to_string()))?;

    println!("Getting compact version info for {version_id}");
    let compact_version = MCCompactVersion::from_id(version_id, &client)
        .ok_or("Could not get compact Minecraft version details!".to_string())?;

    println!("Got compact version info: {:?}", compact_version);
    println!("Getting extended version info from {}", compact_version.url);

    let version = MCExtendedVersion::from_compact(compact_version, &client)
        .ok_or("Could not get extended Minecraft version details!".to_string())?;

    println!("Got extended version info. (not listed due to length)");

    Ok(
        parse_arguments(
            Args {
                jvm: version.get_jvm_args(&client),
                game: version.get_game_args(),
                main_class: version.get_main_class()
            },
            account,
            version,
            minecraft_path,
            &client
        )
    )
}

fn parse_arguments(args_struct: Args, account: MCAccount, version: MCExtendedVersion, minecraft_path: String, client: &Client) -> Args {
    let replacements = vec![
        ("${auth_player_name}", account.mc_profile.name),
        ("${auth_uuid}", account.mc_profile.id),
        ("${auth_access_token}", account.mc_response.access_token),
        ("${auth_xuid}", account.xsts_response.display_claims.xui[0].uhs.to_string()), // idk what else a "xuid" could be
        ("${user_properties}", "something".to_string()),

        ("${classpath}", version.get_classpath(client)),
        ("${version_name}", version.id.replace(' ', "_").replace(':', "_")),
        ("${assets_index_name}", version.asset_index.id),
        ("${assets_root}", "/home/der/.local/share/PrismLauncher/assets".to_string()),
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
            let mut should_retain = true;
            for remover in to_remove.iter() {
                if arg.contains(remover) {
                    should_retain = false;
                }
            }
            should_retain
        }).collect()
    }).into();

    Args {
        jvm: args_final.0,
        game: args_final.1,
        main_class: args_struct.main_class
    }
}