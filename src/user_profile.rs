use crate::utils::{read_file, update_file, FileContents};
use crate::workout_session::WorkoutSession;
use serde_json::Result;
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "user_profile.json";
const CURRENT_USER_FILE_NAME: &str = "current_user.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfile {
    name: String,
    workouts: Vec<WorkoutSession>,
}

impl UserProfile {
    fn new(name: String) -> UserProfile {
        UserProfile {
            name,
            workouts: Vec::new(),
        }
    }
    fn add_workout(&mut self, workout: WorkoutSession) {
        self.workouts.push(workout);
    }
    fn get_workouts(&self) -> &Vec<WorkoutSession> {
        &self.workouts
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    users: Vec<UserProfile>,
}

impl Users {
    pub fn add_user(&mut self, user: UserProfile) {
        self.users.push(user);
    }
    pub fn new() -> Users {
        Users { users: Vec::new() }
    }
    pub fn list(&self) -> &Vec<UserProfile> {
        &self.users
    }
    pub fn user_exists(&self, name: &String) -> bool {
        for user in self.users.iter() {
            if user.get_name() == name {
                return true;
            }
        }
        false
    }
    pub fn delete_user(&mut self, name: String) -> Result<()> {
        let mut index = 0;
        for user in self.users.iter() {
            if user.get_name() == &name {
                break;
            }
            index += 1;
        }
        if index < self.users.len() {
            self.users.remove(index);
            Ok(())
        } else {
            panic!("User does not exist");
        }
    }
    pub fn get_user(&self, name: &String) -> Option<&UserProfile> {
        for user in self.users.iter() {
            if user.get_name() == name {
                return Some(user);
            }
        }
        None
    }
}

pub fn read_profiles() -> Result<Users> {
    let contents = read_file(FILE_NAME)?;
    let users: Users = match contents {
        FileContents::Users(users) => users,
        _ => panic!("Unable to read user profiles"),
    };
    Ok(users)
}

pub fn save_user_profile(user_profile: &UserProfile) {
    let mut users = read_profiles().expect("Unable to read user profiles");

    let user_exists = users.user_exists(user_profile.get_name());

    if !user_exists {
        users.add_user(user_profile.clone());
    }

    let file_contents = FileContents::Users(users);
    update_file(FILE_NAME, &file_contents)
}

pub fn create_profile(name: String) {
    let user_profile = UserProfile::new(name);
    save_user_profile(&user_profile);
}

pub fn delete_profile(name: String) {
    let mut users = read_profiles().expect("Unable to read user profiles");
    users
        .delete_user(name)
        .expect("Unable to delete user profile");

    let file_contents = FileContents::Users(users);
    update_file(FILE_NAME, &file_contents);
}

pub fn read_current_user() -> Result<String> {
    let contents = read_file(CURRENT_USER_FILE_NAME)?;
    let user_profile: UserProfile = match contents {
        FileContents::UserProfile(user_profile) => user_profile,
        _ => panic!("Unable to read current user"),
    };

    Ok(user_profile.get_name().to_string())
}

pub fn choose_profile(name: String) {
    let users = read_profiles().expect("Unable to read user profiles");
    let mut user_exists = false;
    for user in users.list() {
        if user.get_name() == &name {
            user_exists = true;
            break;
        }
    }
    if !user_exists {
        panic!("User does not exist");
    }

    let user_profile = UserProfile::new(name);

    let file_contents = FileContents::UserProfile(user_profile);
    update_file(CURRENT_USER_FILE_NAME, &file_contents)
}
