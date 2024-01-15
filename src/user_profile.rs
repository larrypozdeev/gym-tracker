use crate::utils::save_user_profile;
use crate::workout_session::WorkoutSession;
use serde::{Deserialize, Serialize};

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
    pub fn get_users(&self) -> &Vec<UserProfile> {
        &self.users
    }
}
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

//create user user_profile
pub fn create_profile(name: String) {
    println!("Creating user profile");
    let mut user_profile = UserProfile::new(name);
    save_user_profile(&user_profile).expect("Unable to save user profile");
    let dummy_workout = WorkoutSession::new(Vec::new());
    user_profile.add_workout(dummy_workout);
    save_user_profile(&user_profile).expect("Unable to save user profile");
}
