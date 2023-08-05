use std::{fs::{self, ReadDir}, ffi::OsStr};


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
            /*
            if instance_contents.into_iter().any(|f| f.unwrap().file_name() == OsStr::new("instance.cfg"))  {
                println!("{:?} is a MultiMC instance", instance_folder.file_name())
            } else {
                println!("{:?} is not a MultiMC instance", instance_folder.file_name());
            }
            */
            if contains_file(&instance_contents, "instance.cfg") {
                println!("A: {:?} is a MultiMC instance", instance_folder.file_name())
            } else if contains_file(&instance_contents, ".curseclient") {
                println!("B: {:?} is a CurseForge instance", instance_folder.file_name())
            } else {
                println!("C: {:?} is not a MC instance", instance_folder.file_name())
            }
            

            /*
            for instance_files in instance_contents {
                println!(" -Contents: {:?}", instance_files.unwrap().file_name())
            }
            */
        }
    }

}

fn contains_file(dir: &ReadDir, file: &str) -> bool {
    let mut ret = false;
    dir.for_each(|e| if e.unwrap().file_name() == OsStr::new(file) {ret = true;});
    ret

    //dir.any(|f| f.unwrap().file_name() == OsStr::new(file))
}