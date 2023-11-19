use std::process::Command;

use log::{info, warn};

#[tauri::command(async)]
pub fn get_java_version(path: String, args: String) -> Result<String, String> {
    info!("Getting Java version for: {} using args: {}", path, args);

    let java_process = Command::new(path).args(args.split_whitespace()).arg("-version").output();

    if java_process.is_err() {
        warn!("Java test failed!");
        Err(String::from("Executing Java Command failed! Is the java path correct?"))
    } else {
        if java_process.as_ref().unwrap().status.success() {
            let output = java_process.unwrap().stderr;
            info!("Java test succeeded:\n{}", String::from_utf8(output.clone()).unwrap());
            Ok(String::from_utf8(output).unwrap())
        } else {
            let output = java_process.unwrap().stderr;
            warn!("Java command failed:\n{}", String::from_utf8(output.clone()).unwrap());
            Err(String::from_utf8(output).unwrap())
        }
    }
}