// FILE: step_data_step_reader_data.rs
// occt: StepData_StepReaderData

use std::collections::HashMap;

//! Data container for STEP reader
pub struct StepDataStepReaderData {
    entities: HashMap<usize, String>,
}

impl StepDataStepReaderData {
    //! Creates a StepReaderData
    pub fn new() -> Self {
        StepDataStepReaderData {
            entities: HashMap::new(),
        }
    }

    //! Adds an entity
    pub fn add_entity(&mut self, id: usize, data: String) {
        self.entities.insert(id, data);
    }

    //! Returns the number of entities
    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    //! Returns an entity
    pub fn entity(&self, id: usize) -> Option<&str> {
        self.entities.get(&id).map(|s| s.as_str())
    }
}

impl Default for StepDataStepReaderData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_reader_data_new() {
        let data = StepDataStepReaderData::new();
        assert_eq!(data.nb_entities(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut data = StepDataStepReaderData::new();
        data.add_entity(1, "entity1".to_string());
        assert_eq!(data.nb_entities(), 1);
        assert_eq!(data.entity(1), Some("entity1"));
    }
}
