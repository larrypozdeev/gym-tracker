use crate::utils::{read_user_profiles, save_user_profile, update_current_user, update_users};
use crate::workout_session::WorkoutSession;
use serde::{Deserialize, Serialize};
use serde_json::Result;

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
pub struct CurrentUser {
    user: UserProfile,
}
impl CurrentUser {
    pub fn new(user: UserProfile) -> CurrentUser {
        CurrentUser { user }
    }
    pub fn get_user(&self) -> &UserProfile {
        &self.user
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
        } else {
            panic!("User does not exist");
        }
        Ok(())
    }
}

pub fn create_profile(name: String) -> Result<()> {
    let mut user_profile = UserProfile::new(name);
    save_user_profile(&user_profile).expect("Unable to save user profile");
    let dummy_workout = WorkoutSession::new(Vec::new());
    user_profile.add_workout(dummy_workout);
    save_user_profile(&user_profile).expect("Unable to save user profile");
    update_current_user(&user_profile).expect("Unable to update current user");
    Ok(())
}

pub fn delete_profile(name: String) -> Result<()> {
    let mut users = read_user_profiles().expect("Unable to read user profiles");
    users
        .delete_user(name)
        .expect("Unable to delete user profile");
    update_users(&users).expect("Unable to update user profiles");
    Ok(())
}

pub fn read_profiles() -> Result<Users> {
    let users = read_user_profiles().expect("Unable to read user profiles");
    Ok(users)
}

pub fn choose_profile(name: String) -> Result<()> {
    let users = read_user_profiles().expect("Unable to read user profiles");
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
    update_current_user(&user_profile)
}
