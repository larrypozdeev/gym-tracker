use crate::errors::Result;
use crate::errors::ResultError::OtherError;
use crate::utils::{read_file, update_file, FileContents};
use crate::workout_session::WorkoutSession;
use serde::{Deserialize, Serialize};
use crate::exercise::Exercise;
const FILE_NAME: &str = "user_profile.json";
const CURRENT_USER_FILE_NAME: &str = "current_user.json";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserProfile {
    name: String,
    chosen_workout_session: Option<String>,
    workouts: Vec<WorkoutSession>,
    created_exercises: Vec<Exercise>
}

impl UserProfile {
    pub fn new(name: String) -> UserProfile {
        UserProfile {
            name,
            chosen_workout_session: None,
            workouts: Vec::new(),
            created_exercises: Vec::new(),
        }
    }
    pub fn get_chosen_workout_session(&self) -> Option<&String> {
        self.chosen_workout_session.as_ref()
    }
    pub fn set_chosen_workout_session(&mut self, workout_session: String) {
        self.chosen_workout_session = Some(workout_session);
    }
    pub fn add_workout(&mut self, workout: WorkoutSession) {
        self.workouts.push(workout);
    }
    pub fn remove_workout(&mut self, workout: WorkoutSession) {
        self.workouts.retain(|x| x != &workout);
    }
    pub fn get_workouts(&self) -> &Vec<WorkoutSession> {
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
    else {
        let user = users
            .get_user(user_profile.get_name())
            .ok_or(OtherError("User does not exist".to_string()))?;
        let index = users
            .list()
            .iter()
            .position(|x| x == user)
            .ok_or(OtherError("User does not exist".to_string()))?;
        users.users[index] = user_profile.clone();
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

pub fn get_current_user() -> Result<UserProfile> {
    // {CURRENT_USER_FILE_NAME} doesn't store any data other than the name of the current user
    // so, we need to read both both files and find the user profile that matches the name

    let users = read_profiles().expect("Unable to read user profiles");
    let contents = read_file(CURRENT_USER_FILE_NAME)?;

    match contents {
        FileContents::UserProfile(user_profile) => {
            for user in users.list() {
                if user.get_name() == user_profile.get_name() {
                    return Ok(user.clone());
                }
            }

            Err(OtherError("Unable to find current user".to_string()))
        }
        _ => Err(OtherError("Unable to read current user".to_string())),
    }
}

pub fn read_current_user() -> Result<String> {
    let user_profile = get_current_user()?;

    Ok(user_profile.get_name().to_string())
}

pub fn choose_profile(name: String) -> Result<()> {
    let users = read_profiles().expect("Unable to read user profiles");

    if name == "default" {
        for user in users.list() {
            if user.get_name() != "default" {
                let file_contents = FileContents::UserProfile(user.clone());
                return update_file(CURRENT_USER_FILE_NAME, &file_contents);
            }
        }
        create_profile("default".to_string())?;
    }

    let user = users
        .get_user(&name)
        .ok_or(OtherError("User does not exist".to_string()))?;

    let file_contents = FileContents::UserProfile(user.clone());
    update_file(CURRENT_USER_FILE_NAME, &file_contents)
}


#[cfg(test)]
mod tests {
    // unit tests for user_profile.rs
    use super::*;
    use crate::errors::ResultError;

    #[test]
    fn test_new() {
        let users = Users::new();
        assert_eq!(users.list(), &Vec::new());
    }

    #[test]
    fn test_add_user() {
        let mut users = Users::new();
        let user = UserProfile::new("test".to_string());
        users.add_user(user.clone());
        assert_eq!(users.list(), &vec![user]);
    }

    #[test]
    fn test_user_exists() {
        let mut users = Users::new();
        let user = UserProfile::new("test".to_string());
        users.add_user(user.clone());
        assert!(users.user_exists(&user.get_name()));
    }

    #[test]
    fn test_user_does_not_exist() {
        let users = Users::new();
        assert!(!users.user_exists(&"test".to_string()));
    }

    #[test]
    fn test_delete_user() {
        let mut users = Users::new();
        let user = UserProfile::new("test".to_string());
        users.add_user(user.clone());
        users.delete_user(user.get_name().to_string()).unwrap();
        assert_eq!(users.list(), &Vec::new());
    }

    #[test]
    fn test_delete_user_does_not_exist() {
        let mut users = Users::new();
        let user = UserProfile::new("test".to_string());
        users.add_user(user.clone());
        let result = users.delete_user("test2".to_string());
        assert!(matches!(
            result,
            Err(ResultError::OtherError(_))
        ));
    }

    #[test]
    fn test_get_user() {
        let mut users = Users::new();
        let user = UserProfile::new("test".to_string());
        users.add_user(user.clone());
        assert_eq!(users.get_user(&user.get_name()), Some(&user));
    }

    #[test]
    fn test_get_user_does_not_exist() {
        let users = Users::new();
        assert_eq!(users.get_user(&"test".to_string()), None);
    }

}
