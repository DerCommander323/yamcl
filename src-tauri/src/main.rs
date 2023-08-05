use std::fs::{self, DirEntry};
mod instances;

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
                    "minecraftinstance.json" => {handle_instance_cf(instance_folder); break;},
                    "instance.cfg" => {handle_instance_mmc(instance_folder); break;},
                    _ => continue
                }   
            }
        }
    }
}

fn handle_instance_cf(dir: DirEntry) {
    println!("{:?} is a CurseForge instance!", dir.file_name());
    instances::get_instance_name_cf(dir)

}

fn handle_instance_mmc(dir: DirEntry) {
    println!("{:?} is a MultiMC instance!", dir.file_name());
    instances::get_instance_name_mmc(dir)
}