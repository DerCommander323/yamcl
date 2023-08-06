use std::fs::{self, DirEntry};

use tauri::{Manager, AppHandle};
mod instances;

#[derive(Clone, serde::Serialize)]
struct InstanceData {
  name: String
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_instances])
        .run(tauri::generate_context!())
        .expect("failed to run app");
        
}

#[tauri::command(async)]
fn get_instances(path: String, app_handle: AppHandle) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if path.as_ref().unwrap().file_type().unwrap().is_dir() {
            let instance_folder = path.unwrap();

            let instance_contents = fs::read_dir(instance_folder.path()).unwrap();

            for file in instance_contents {
                match file.unwrap().file_name().into_string().unwrap().as_ref() {
                    "minecraftinstance.json" => {handle_instance_cf(instance_folder, app_handle.clone()); break;},
                    "instance.cfg" => {handle_instance_mmc(instance_folder, app_handle.clone()); break;},
                    _ => continue
                }   
            }
        }
    }
}

fn handle_instance_cf(dir: DirEntry, app_handle: AppHandle) {
    let name = instances::get_instance_name_cf(dir);
    println!("Curseforge: {}", &name);
    emit_instance_create(InstanceData { name }.into(), app_handle)
}

fn handle_instance_mmc(dir: DirEntry, app_handle: AppHandle) {
    let name = instances::get_instance_name_mmc(dir);
    println!("MultiMC: {}", &name);
    emit_instance_create(InstanceData { name }.into(), app_handle)
}

fn emit_instance_create(data: InstanceData, app_handle: AppHandle) {
    app_handle.emit_all("instance_create", data).unwrap()
}