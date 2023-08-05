use std::fs::DirEntry;
use std::fs::File;
use std::fs::read_to_string;
use std::io::BufRead;
use std::io::BufReader;

use configparser::ini::Ini;
use serde_json::Value;


pub fn get_instance_name_cf(dir: DirEntry) {
    let data = read_to_string(dir.path().join("minecraftinstance.json")).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let name = json["name"].as_str().unwrap_or("Name not found!");

    println!("{}", name);
}

fn get_instance_name_mmc(dir: DirEntry) {
    // let mut reader = BufReader::new(File::open(dir.path().join("instance.cfg")).expect("Cannot read"));
    // let mut buf = String::new();
    let mut name = String::new();
    // reader.read_line(&mut buf);
    // match buf.as_ref() {
    //     "[General]" => { name = reader.lines().nth(32).unwrap().unwrap();},
    //     _ => { name = reader.lines().nth(31).unwrap().unwrap();}
    // }

    let data = read_to_string(dir.path().join("instance.cfg")).unwrap();
    let mut contents = Ini::new();
    contents.read(data);
    
    if contents.sections().contains(&"General".to_string()) {
        name = contents.get("General", "name").unwrap();
    } else {
        name = contents.get("default", "name").unwrap();
    }

    println!("{}", name.replace("name=", ""))
}
