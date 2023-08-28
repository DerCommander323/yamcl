use std::process::Command;



#[tauri::command]
pub fn get_java_version(path: String, args: String) -> Option<String> {
    println!("Getting Java version for: {} using args: {}", path, args);

    let java_process = Command::new(path).args(args.split_whitespace()).arg("-version").output();

    if java_process.is_err() {
        println!("Java test failed!");
        None
    } else {
        if java_process.as_ref().unwrap().status.success() {
            let output = java_process.unwrap().stderr;
            println!("Java test succeeded:\n{}", String::from_utf8(output.clone()).unwrap());
            Some(String::from_utf8(output).unwrap())
        } else {
            println!("Java command failed:\n{}", String::from_utf8(java_process.unwrap().stderr).unwrap());
            None
        }

    }
}


#[tauri::command]
pub fn launch_instance(minecraft_path: String, java_path: String) {
    println!("Launching: {}", minecraft_path);

    let mut _process = Command::new(java_path).arg("--version").spawn().unwrap();
}