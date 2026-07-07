// FILE: step_repr_definitional_representation.rs
// occt: StepRepr_DefinitionalRepresentation

/// StepRepr_DefinitionalRepresentation: A definitional representation
/// Inherits from StepRepr_Representation
#[derive(Clone, Debug)]
pub struct StepReprDefinitionalRepresentation {
    name: String,
}

impl StepReprDefinitionalRepresentation {
    /// Returns a DefinitionalRepresentation
    pub fn new() -> Self {
        StepReprDefinitionalRepresentation {
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

impl Default for StepReprDefinitionalRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let dr = StepReprDefinitionalRepresentation::new();
        assert_eq!(dr.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut dr = StepReprDefinitionalRepresentation::new();
        dr.set_name("test_def".to_string());
        assert_eq!(dr.name(), "test_def");
    }
}
