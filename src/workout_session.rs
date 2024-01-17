use crate::exercise::Exercise;
use crate::user_profile::{self, save_user_profile};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WorkoutSession {
    exercises: Vec<Exercise>,
    name: String,
}
impl WorkoutSession {
    pub fn new(exercises: Vec<Exercise>) -> WorkoutSession {
        WorkoutSession {
            exercises,
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
    pub fn get_exercises(&self) -> &Vec<Exercise> {
        &self.exercises
    }
    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }
    pub fn remove_exercise(&mut self, exercise: Exercise) {
        self.exercises.retain(|x| x != &exercise);
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
