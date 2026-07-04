// FILE: step_visual_characterized_obj_and_representation_and_draughting_model.rs
// occt: StepVisual_CharacterizedObjAndRepresentationAndDraughtingModel

/// Represents a characterized object with representation and draughting model
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CharacterizedObjAndRepresentationAndDraughtingModel {
    name: Option<String>,
}

impl StepVisual_CharacterizedObjAndRepresentationAndDraughtingModel {
    pub fn new() -> Self {
        StepVisual_CharacterizedObjAndRepresentationAndDraughtingModel { name: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let coard = StepVisual_CharacterizedObjAndRepresentationAndDraughtingModel::new();
        assert!(coard.name().is_none());
    }
}
