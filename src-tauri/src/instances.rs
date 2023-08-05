use std::fs::DirEntry;
use std::fs::read_to_string;

use configparser::ini::Ini;
use serde_json::Value;


pub fn get_instance_name_cf(dir: DirEntry) {
    let data = read_to_string(dir.path().join("minecraftinstance.json")).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let name = json["name"].as_str().unwrap_or("Name not found!");

    println!("{}", name);
}

pub fn get_instance_name_mmc(dir: DirEntry) {
    let mut data = read_to_string(dir.path().join("instance.cfg")).unwrap();
    data = data.replace("[General]", "");

    let mut config = Ini::new();
    config.read(data);

    println!("{}", config.get("default","name").unwrap());
}
