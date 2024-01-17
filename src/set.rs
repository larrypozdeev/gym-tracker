use crate::exercise::Exercise;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Set {
    exercise: Exercise,
    reps: u8,
    weight: u16,
    is_dropset: bool,
}

impl Set {
    pub fn new(exercise: Exercise, reps: u8, weight: u16, is_dropset: bool) -> Set {
        Set {
            exercise,
            reps,
            weight,
            is_dropset,
        }
    }
    pub fn get_exercise(&self) -> &Exercise {
        &self.exercise
    }
    pub fn get_reps(&self) -> &u8 {
        &self.reps
    }
    pub fn get_weight(&self) -> &u16 {
        &self.weight
    }
    pub fn get_is_dropset(&self) -> &bool {
        &self.is_dropset
    }
    pub fn edit(&mut self, reps: u8, weight: u16, is_dropset: bool) {
        self.reps = reps;
        self.weight = weight;
        self.is_dropset = is_dropset;
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Sets {
    sets: Vec<Set>,
}
impl Sets {
    pub fn new(sets: Vec<Set>) -> Sets {
        Sets { sets }
    }
    pub fn get_sets(&self) -> &Vec<Set> {
        &self.sets
    }
    pub fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }
    pub fn remove_set(&mut self, set: Set) {
        self.sets.retain(|x| x != &set);
    }
}


