// FILE: step_repr_mechanical_design_and_draughting_relationship.rs
// occt: StepRepr_MechanicalDesignAndDraughtingRelationship

/// StepRepr_MechanicalDesignAndDraughtingRelationship:
/// A representation relationship for mechanical design and draughting
/// Inherits from StepRepr_RepresentationRelationship
#[derive(Clone, Debug)]
pub struct StepReprMechanicalDesignAndDraughtingRelationship {
    name: String,
}

impl StepReprMechanicalDesignAndDraughtingRelationship {
    /// Returns a MechanicalDesignAndDraughtingRelationship
    pub fn new() -> Self {
        StepReprMechanicalDesignAndDraughtingRelationship {
            name: String::new(),
        }
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepReprMechanicalDesignAndDraughtingRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let mddr = StepReprMechanicalDesignAndDraughtingRelationship::new();
        assert_eq!(mddr.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut mddr = StepReprMechanicalDesignAndDraughtingRelationship::new();
        mddr.set_name("design_rel".to_string());
        assert_eq!(mddr.name(), "design_rel");
    }
}
