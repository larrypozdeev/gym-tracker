use crate::exercise::Exercise;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkoutSession {
    exercises: Vec<Exercise>,
}
impl WorkoutSession {
    pub fn new(exercises: Vec<Exercise>) -> WorkoutSession {
        WorkoutSession {
            exercises,
        }
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
    println!("Starting workout session");
}
pub fn end() {
    println!("Ending workout session");
}
