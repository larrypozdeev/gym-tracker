use crate::user_profile::{UserProfile, Users};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


#[derive(serde::Serialize, serde::Deserialize)]
pub enum FileContents {
    Users(Users),
    UserProfile(UserProfile),
}

pub fn read_file(path_str: &str) -> Result<FileContents> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Ok(FileContents::Users(Users::new()));
    }

    let mut file = File::open(&path).expect("Unable to open file");

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            let file_contents: FileContents = serde_json::from_str(&contents)?;
            Ok(file_contents)
        },
        Err(_) => {panic!("Unable to read file")},
    }
}

pub fn update_file(path: &str, contents: &FileContents) -> () {
    let path = Path::new(path);
    let display = path.display();

    let contents = serde_json::to_string(&contents).unwrap();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}
