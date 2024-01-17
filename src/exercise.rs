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
    fn set_exercise_name(&mut self, name: String) {
        self.name = name;
    }
    fn set_exercise_description(&mut self, description: String) {
        self.description = description;
    }
    fn set_exercise_muscle_groups(&mut self, muscle_groups: Vec<String>) {
        self.muscle_groups = muscle_groups;
    }
    fn set_exercise_equipment(&mut self, equipment: String) {
        self.equipment = equipment;
    }
}
