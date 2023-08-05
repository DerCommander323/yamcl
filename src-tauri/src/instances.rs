use std::fs::DirEntry;
use std::fs::File;
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

fn get_instance_name_mmc(dir: DirEntry) {
    let mut reader = BufReader::new(File::open(dir.path().join("instance.cfg")).unwrap());
    let mut buf = String::new();
    let mut name = String::new();
    reader.read_line(&mut buf);
    match buf.as_ref() {
        "[General]" => { name = reader.lines().nth(32).unwrap().unwrap();},
        _ => { name = reader.lines().nth(31).unwrap().unwrap();}
    }



    println!("{}", name.replace("name=", ""))
}
