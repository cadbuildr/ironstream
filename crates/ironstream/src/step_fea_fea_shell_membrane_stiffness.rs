// FILE: step_fea_fea_shell_membrane_stiffness.rs
// occt: StepFEA_FeaShellMembraneStiffness

/// Representation of STEP entity FeaShellMembraneStiffness
#[derive(Debug, Clone)]
pub struct StepFeaFeaShellMembraneStiffness {
    name: String,
    fea_constants: Vec<f64>,
}

impl StepFeaFeaShellMembraneStiffness {
    /// Creates a new empty FeaShellMembraneStiffness
    pub fn new() -> Self {
        StepFeaFeaShellMembraneStiffness {
            name: String::new(),
            fea_constants: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, fea_constants: Vec<f64>) {
        self.name = name;
        self.fea_constants = fea_constants;
    }

    /// Returns field FeaConstants
    pub fn fea_constants(&self) -> &[f64] {
        &self.fea_constants
    }

    /// Set field FeaConstants
    pub fn set_fea_constants(&mut self, fea_constants: Vec<f64>) {
        self.fea_constants = fea_constants;
    }

    /// Returns field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepFeaFeaShellMembraneStiffness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_shell_membrane_stiffness_creation() {
        let stiffness = StepFeaFeaShellMembraneStiffness::new();
        assert_eq!(stiffness.name(), "");
        assert_eq!(stiffness.fea_constants().len(), 0);
    }

    #[test]
    fn test_fea_shell_membrane_stiffness_init() {
        let mut stiffness = StepFeaFeaShellMembraneStiffness::new();
        let constants = vec![80.0, 90.0];
        stiffness.init("Membrane".to_string(), constants);

        assert_eq!(stiffness.name(), "Membrane");
        assert_eq!(stiffness.fea_constants(), &[80.0, 90.0]);
    }

    #[test]
    fn test_fea_shell_membrane_stiffness_setters() {
        let mut stiffness = StepFeaFeaShellMembraneStiffness::new();
        stiffness.set_name("Test".to_string());
        stiffness.set_fea_constants(vec![85.0]);

        assert_eq!(stiffness.name(), "Test");
        assert_eq!(stiffness.fea_constants(), &[85.0]);
    }
}
