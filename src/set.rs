use crate::user_profile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Set {
    exercise: String,
    reps: u32,
    weight: f32,
    is_dropset: bool,
}

impl Set {
    pub fn new(exercise: String, reps: u32, weight: f32, is_dropset: bool) -> Set {
        Set {
            exercise,
            reps,
            weight,
            is_dropset,
        }
    }
    pub fn get_exercise(&self) -> &String {
        &self.exercise
    }
    pub fn get_reps(&self) -> &u32 {
        &self.reps
    }
    pub fn get_weight(&self) -> &f32 {
        &self.weight
    }
    pub fn get_is_dropset(&self) -> &bool {
        &self.is_dropset
    }
}

pub fn create_set(exercise: String, reps: u32, weight: f32, is_dropset: bool) -> () {
    let new_set = Set::new(exercise, reps, weight, is_dropset);
    let mut workout_s = crate::workout_session::get_current_session();
    workout_s.add_set(new_set);

    crate::workout_session::save_current_session(&workout_s).unwrap();
    crate::user_profile::save_user_profile(&user_profile::get_current_user().unwrap()).unwrap();
}
