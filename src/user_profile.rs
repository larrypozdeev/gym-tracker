use crate::errors::Result;
use crate::errors::ResultError::OtherError;
use crate::utils::{read_file, update_file, FileContents};
use crate::workout_session::WorkoutSession;
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "user_profile.json";
const CURRENT_USER_FILE_NAME: &str = "current_user.json";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserProfile {
    name: String,
    workouts: Vec<WorkoutSession>,
}

impl UserProfile {
    pub fn new(name: String) -> UserProfile {
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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
        if let Some(index) = self.users.iter().position(|user| user.get_name() == &name) {
            self.users.remove(index);
            Ok(())
        } else {
            Err(OtherError("User does not exist".to_string()))
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
    match contents {
        FileContents::Users(users) => Ok(users),
        _ => Err(OtherError("Unable to read user profiles".to_string())),
    }
}

pub fn save_user_profile(user_profile: &UserProfile) -> Result<()> {
    let mut users = match read_profiles() {
        Ok(users) => users,
        Err(e) => Err(e)?,
    };

    let user_exists = users.user_exists(user_profile.get_name());

    if !user_exists {
        users.add_user(user_profile.clone());
    }

    let file_contents = FileContents::Users(users);
    update_file(FILE_NAME, &file_contents)
}

pub fn create_profile(name: String) -> Result<()> {
    let user_profile = UserProfile::new(name);
    save_user_profile(&user_profile)
}

pub fn delete_profile(name: String) -> Result<()> {
    let mut users = read_profiles().expect("Unable to read user profiles");
    users
        .delete_user(name)
        .expect("Unable to delete user profile");

    let file_contents = FileContents::Users(users);
    update_file(FILE_NAME, &file_contents)
}

pub fn read_current_user() -> Result<String> {
    let contents = read_file(CURRENT_USER_FILE_NAME)?;
    let user_profile: UserProfile = match contents {
        FileContents::UserProfile(user_profile) => user_profile,
        _ => panic!("Unable to read current user"),
    };

    Ok(user_profile.get_name().to_string())
}

pub fn choose_profile(name: String) -> Result<()> {
    let users = read_profiles().expect("Unable to read user profiles");

    let user = users
        .get_user(&name)
        .ok_or(OtherError("User does not exist".to_string()))?;

    let file_contents = FileContents::UserProfile(user.clone());
    update_file(CURRENT_USER_FILE_NAME, &file_contents)
}
