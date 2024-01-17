use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Exercise {
    exercise_id: i32,
    exercise_name: String,
    exercise_description: String,
    exercise_muscle_groups: Vec<String>,
    exercise_equipment: String,
}

impl Exercise {
    fn new(
        exercise_id: i32,
        exercise_name: String,
        exercise_description: Option<String>,
        exercise_muscle_groups: Vec<String>,
        exercise_equipment: String,
    ) -> Exercise {
        Exercise {
            exercise_id,
            exercise_name,
            exercise_description: exercise_description.unwrap_or_default(),
            exercise_muscle_groups,
            exercise_equipment,
        }
    }
    fn set_exercise_name(&mut self, exercise_name: String) {
        self.exercise_name = exercise_name;
    }
    fn set_exercise_description(&mut self, exercise_description: String) {
        self.exercise_description = exercise_description;
    }
    fn set_exercise_muscle_groups(&mut self, exercise_muscle_groups: Vec<String>) {
        self.exercise_muscle_groups = exercise_muscle_groups;
    }
    fn set_exercise_equipment(&mut self, exercise_equipment: String) {
        self.exercise_equipment = exercise_equipment;
    }
    fn get_exercise_id(&self) -> i32 {
        self.exercise_id
    }
}


