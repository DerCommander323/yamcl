
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

#[tauri::command]
fn my_custom_command(path: String) -> String {
  println!("Returning {}", path);
  return "Path is: ".to_owned()+&path
}