use crate::errors::{ResultError, Result};
use crate::user_profile::get_current_user;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Exercise {
    name: String,
    description: String,
    muscle_groups: Vec<String>,
    equipment: String,
}

impl Exercise {
    fn new(
        name: String,
        description: Option<String>,
        muscle_groups: Vec<String>,
        equipment: String,
    ) -> Exercise {
        Exercise {
            name,
            description: description.unwrap_or_default(),
            muscle_groups,
            equipment,
        }
    }
    pub fn set_exercise_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_exercise_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_exercise_muscle_groups(&mut self, muscle_groups: Vec<String>) {
        self.muscle_groups = muscle_groups;
    }
    pub fn set_exercise_equipment(&mut self, equipment: String) {
        self.equipment = equipment;
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

pub fn create_exercise(
    name: String,
    description: Option<String>,
    muscle_groups: Vec<String>,
    equipment: String,
) -> Result<()> {
    Exercise::new(
        name.clone(),
        description.clone(),
        muscle_groups.clone(),
        equipment.clone(),
    );
    let mut user_profile = get_current_user()?;
    user_profile.add_exercise(Exercise::new(name, description, muscle_groups, equipment));
    user_profile.save()
}

pub fn get_exercise(name: String) -> Result<Exercise> {
    let user_profile = get_current_user()?;

    for exercise in user_profile.get_exercises() {
        if exercise.name == name {
            return Result::Ok(exercise.clone());
        }
    }
    Result::Err(ResultError::OtherError("Exercise not found".to_string()))
}

pub fn delete_exercise(name: &str) -> Result<()> {
    let mut user_profile = get_current_user()?;
    user_profile.remove_exercise(name);
    user_profile.save()
}

pub fn choose_exercise(name: String) -> Result<()> {
    let mut user_profile = get_current_user()?;
    user_profile.set_chosen_exercise(name);
    user_profile.save()
}
