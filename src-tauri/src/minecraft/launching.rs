use std::process::Command;

use crate::auth::get_active_account;


#[tauri::command]
pub fn launch_instance(minecraft_path: String, java_path: String) {
    println!("Launching: {}", minecraft_path);

    let mut _process = Command::new(java_path).arg("--version").spawn().unwrap();
}

fn get_arguments() {
    let account = get_active_account();
}