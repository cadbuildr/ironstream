// FILE: step_fea_fea_shell_shear_stiffness.rs
// occt: StepFEA_FeaShellShearStiffness

/// Representation of STEP entity FeaShellShearStiffness
#[derive(Debug, Clone)]
pub struct StepFeaFeaShellShearStiffness {
    name: String,
    fea_constants: Vec<f64>,
}

impl StepFeaFeaShellShearStiffness {
    /// Creates a new empty FeaShellShearStiffness
    pub fn new() -> Self {
        StepFeaFeaShellShearStiffness {
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

impl Default for StepFeaFeaShellShearStiffness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_shell_shear_stiffness_creation() {
        let stiffness = StepFeaFeaShellShearStiffness::new();
        assert_eq!(stiffness.name(), "");
        assert_eq!(stiffness.fea_constants().len(), 0);
    }

    #[test]
    fn test_fea_shell_shear_stiffness_init() {
        let mut stiffness = StepFeaFeaShellShearStiffness::new();
        let constants = vec![45.0, 55.0];
        stiffness.init("Shear".to_string(), constants);

        assert_eq!(stiffness.name(), "Shear");
        assert_eq!(stiffness.fea_constants(), &[45.0, 55.0]);
    }

    #[test]
    fn test_fea_shell_shear_stiffness_setters() {
        let mut stiffness = StepFeaFeaShellShearStiffness::new();
        stiffness.set_name("Test".to_string());
        stiffness.set_fea_constants(vec![50.0]);

        assert_eq!(stiffness.name(), "Test");
        assert_eq!(stiffness.fea_constants(), &[50.0]);
    }
}
