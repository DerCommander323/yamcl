use std::fs::ReadDir;
use std::fs::read_to_string;


fn get_instance_name_cf(dir: ReadDir) {
    let mut name = "";
    
    for file in dir {
        if file.unwrap().file_name() == "minecraftinstance.json" {
            let data = read_to_string(file.unwrap().path()).unwrap();
            let json = serde_json::json!(data);

            name = json["name"].as_str().unwrap()
            
        }
    }
    println!("{}", name)
}
