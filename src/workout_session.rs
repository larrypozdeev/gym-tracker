use crate::set::Set;
use crate::user_profile::{self, save_user_profile};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WorkoutSession {
    sets: Vec<crate::set::Set>,
    name: String,
}
impl WorkoutSession {
    pub fn new(sets: Vec<Set>) -> WorkoutSession {
        WorkoutSession {
            sets,
            name: String::from(
                DateTime::<Utc>::from(Utc::now())
                    .format("%m-%d-%Y")
                    .to_string(),
            ),
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_sets(&self) -> &Vec<Set> {
        &self.sets
    }
    pub fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }
}

pub fn start() {
    let current_user = user_profile::read_current_user().unwrap();
    let users = user_profile::read_profiles().unwrap();

    let mut user = users
        .list()
        .iter()
        .find(|x| x.get_name() == &current_user)
        .unwrap()
        .clone();

    let workout_session = WorkoutSession::new(Vec::new());
    user.add_workout(workout_session.clone());
    user.set_chosen_workout_session(workout_session.get_name().clone());

    save_user_profile(&user).unwrap();

    println!("Starting workout session");
}

pub fn choose(name: String) {
    let current_user = user_profile::read_current_user().unwrap();
    let users = user_profile::read_profiles().unwrap();

    let mut user = users
        .list()
        .iter()
        .find(|x| x.get_name() == &current_user)
        .unwrap()
        .clone();

    let workout_session = user
        .get_workouts()
        .iter()
        .find(|x| x.name == name)
        .unwrap()
        .clone();
    user.set_chosen_workout_session(workout_session.get_name().clone());
    save_user_profile(&user).unwrap();

    println!("Chose workout session: {}", workout_session.name);
}
pub fn list() {
    let current_user = user_profile::read_current_user().unwrap();
    let users = user_profile::read_profiles().unwrap();

    let user = users
        .list()
        .iter()
        .find(|x| x.get_name() == &current_user)
        .unwrap();

    for workout in user.get_workouts() {
        println!("{}", workout.get_name());
    }
}
pub fn get_current_session() -> WorkoutSession {
    let current_user = user_profile::read_current_user().unwrap();
    let users = user_profile::read_profiles().unwrap();

    let user = users
        .list()
        .iter()
        .find(|x| x.get_name() == &current_user)
        .unwrap();

    let workout_session = user
        .get_workouts()
        .iter()
        .find(|x| x.get_name() == user.get_chosen_workout_session().clone().unwrap())
        .unwrap();

    workout_session.clone()
}
pub fn save_current_session(workout_session: &WorkoutSession) -> crate::errors::Result<()> {
    let current_user = user_profile::read_current_user().unwrap();
    let users = user_profile::read_profiles().unwrap();

    let mut user = users
        .list()
        .iter()
        .find(|x| x.get_name() == &current_user)
        .unwrap()
        .clone();

    let current_session_name = user.get_chosen_workout_session().clone().unwrap();
    let current_session = user
        .get_workouts()
        .iter()
        .find(|x| x.get_name() == current_session_name)
        .unwrap();

    user.remove_workout(current_session.clone());
    user.add_workout(workout_session.clone());


    save_user_profile(&user)
}
pub fn delete() {
    let current_user = user_profile::read_current_user().unwrap();
    let users = user_profile::read_profiles().unwrap();

    let mut user = users
        .list()
        .iter()
        .find(|x| x.get_name() == &current_user)
        .unwrap()
        .clone();
    let current_session = get_current_session();

    user.remove_workout(current_session.clone());

    save_user_profile(&user).unwrap();
    println!("Deleted workout session: {}", current_session.get_name());
}

pub fn display() {
    let current_session = get_current_session();
    println!("Workout session: {}", current_session.get_name());
    for set in current_session.get_sets() {
        println!("Set: {}", set.get_exercise());
        println!("Reps: {}", set.get_reps());
        println!("Weight: {}", set.get_weight());
        println!("Is dropset: {}", set.get_is_dropset());
        println!();
    }
}
