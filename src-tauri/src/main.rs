use std::{fs::{self}, ffi::OsString};


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_instances])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}



#[tauri::command]
fn get_instances(path: String) {

    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if path.as_ref().unwrap().file_type().unwrap().is_dir() {
            let instance_folder = path.unwrap();

            let instance_contents = fs::read_dir(instance_folder.path()).unwrap();

            for file in instance_contents {
                match file.unwrap().file_name().into_string().unwrap().as_ref() {
                    ".curseclient" => {handle_instance_curseforge(instance_folder.file_name()); break;},
                    "instance.cfg" => {handle_instance_multimc(instance_folder.file_name()); break;},
                    _ => continue
                }   
            }
        }
    }
}

fn handle_instance_curseforge(dir: OsString) {
    println!("{:?} is a CurseForge instance!", dir)
}

fn handle_instance_multimc(dir: OsString) {
    println!("{:?} is a MultiMC instance!", dir)
}