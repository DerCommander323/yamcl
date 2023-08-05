use std::fs::DirEntry;
use std::fs::ReadDir;
use std::fs::read_to_string;
use std::io::BufRead;
use std::io::BufReader;


pub fn get_instance_name_cf(dir: DirEntry) {
    let data = read_to_string(dir.path().join("minecraftinstance.json")).unwrap();
    let json = serde_json::json!(data);
    let name = json["name"].as_str().unwrap_or("Name not found!");

    println!("{}", name);
}

fn get_instance_name_mmc(dir: ReadDir) {
    let mut reader: BufReader<std::fs::File>;
    let mut name = String::new();

    for file in dir {
        if file.as_ref().unwrap().file_name() == "instance.cfg" {
            reader = BufReader::new(std::fs::File::open(file.unwrap().path()).unwrap());
            let mut lines = reader.lines();
            if lines.nth(0).unwrap().unwrap() == "[General]".to_string() {
                name = lines.nth(32).unwrap().unwrap()
            } else {
                name = lines.nth(31).unwrap().unwrap()
            }
            
        }
    }

    println!("{}", name.replace("name=", ""))
}
