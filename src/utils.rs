use crate::user_profile::UserProfile;
use crate::user_profile::Users;
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILE_NAME: &str = "user_profile.json";
const CURRENT_USER_FILE_NAME: &str = "current_user.json";

pub fn read_user_profiles() -> Result<Users> {
    let path = Path::new(FILE_NAME);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => serde_json::from_str(&contents),
    }
}

// saves all user profiles
pub fn update_users(users: &Users) -> Result<()> {
    let path = Path::new(FILE_NAME);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let serialized = serde_json::to_string(&users)?;
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };

    Ok(())
}

//save a single user_profile
pub fn save_user_profile(user_profile: &UserProfile) -> Result<()> {
    let contents = match read_user_profiles() {
        Ok(users) => serde_json::to_string(&users)?,
        Err(_) => String::from(""),
    };

    let mut users: Users = serde_json::from_str(&contents).unwrap_or(Users::new());
    let mut user_exists = false;
    for user in users.list() {
        if user.get_name() == user_profile.get_name() {
            user_exists = true;
            break;
        }
    }

    if !user_exists {
        users.add_user(user_profile.clone());
    }

    update_users(&users).expect("Unable to update user profiles");
    Ok(())
}

pub fn read_current_user() -> Result<UserProfile> {
    let path = Path::new(CURRENT_USER_FILE_NAME);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => serde_json::from_str(&contents),
    }
}

pub fn update_current_user(user_profile: &UserProfile) -> Result<()> {
    let path = Path::new(CURRENT_USER_FILE_NAME);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let serialized = serde_json::to_string(&user_profile)?;
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };

    Ok(())
}
