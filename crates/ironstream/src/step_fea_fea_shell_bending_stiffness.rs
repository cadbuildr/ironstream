// FILE: step_fea_fea_shell_bending_stiffness.rs
// occt: StepFEA_FeaShellBendingStiffness

/// Representation of STEP entity FeaShellBendingStiffness
#[derive(Debug, Clone)]
pub struct StepFeaFeaShellBendingStiffness {
    name: String,
    fea_constants: Vec<f64>,
}

impl StepFeaFeaShellBendingStiffness {
    /// Creates a new empty FeaShellBendingStiffness
    pub fn new() -> Self {
        StepFeaFeaShellBendingStiffness {
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

impl Default for StepFeaFeaShellBendingStiffness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_shell_bending_stiffness_creation() {
        let stiffness = StepFeaFeaShellBendingStiffness::new();
        assert_eq!(stiffness.name(), "");
        assert_eq!(stiffness.fea_constants().len(), 0);
    }

    #[test]
    fn test_fea_shell_bending_stiffness_init() {
        let mut stiffness = StepFeaFeaShellBendingStiffness::new();
        let constants = vec![100.0, 200.0];
        stiffness.init("Bending".to_string(), constants);

        assert_eq!(stiffness.name(), "Bending");
        assert_eq!(stiffness.fea_constants(), &[100.0, 200.0]);
    }

    #[test]
    fn test_fea_shell_bending_stiffness_setters() {
        let mut stiffness = StepFeaFeaShellBendingStiffness::new();
        stiffness.set_name("Test".to_string());
        stiffness.set_fea_constants(vec![150.0]);

        assert_eq!(stiffness.name(), "Test");
        assert_eq!(stiffness.fea_constants(), &[150.0]);
    }
}
