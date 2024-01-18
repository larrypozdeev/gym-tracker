use crate::errors::Result;
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

pub fn create_exercise(
    name: String,
    description: Option<String>,
    muscle_groups: Vec<String>,
    equipment: String,
) -> Result<()> {
    Exercise::new(name.clone(), description.clone(), muscle_groups.clone(), equipment.clone());
    let mut user_profile = get_current_user()?;
    user_profile.add_exercise(Exercise::new(name, description, muscle_groups, equipment));
    user_profile.save()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exercise() {
        let name = "test".to_string();
        let description = Some("test description".to_string());
        let muscle_groups = vec!["test".to_string()];
        let equipment = "test".to_string();
        let result = create_exercise(name, description, muscle_groups, equipment);
        assert!(result.is_ok());
    }
}
