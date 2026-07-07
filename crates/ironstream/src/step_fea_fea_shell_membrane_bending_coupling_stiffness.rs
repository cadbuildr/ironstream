// FILE: step_fea_fea_shell_membrane_bending_coupling_stiffness.rs
// occt: StepFEA_FeaShellMembraneBendingCouplingStiffness

/// Representation of STEP entity FeaShellMembraneBendingCouplingStiffness
#[derive(Debug, Clone)]
pub struct StepFeaFeaShellMembraneBendingCouplingStiffness {
    name: String,
    fea_constants: Vec<f64>,
}

impl StepFeaFeaShellMembraneBendingCouplingStiffness {
    /// Creates a new empty FeaShellMembraneBendingCouplingStiffness
    pub fn new() -> Self {
        StepFeaFeaShellMembraneBendingCouplingStiffness {
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

impl Default for StepFeaFeaShellMembraneBendingCouplingStiffness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_shell_membrane_bending_coupling_stiffness_creation() {
        let coupling = StepFeaFeaShellMembraneBendingCouplingStiffness::new();
        assert_eq!(coupling.name(), "");
        assert_eq!(coupling.fea_constants().len(), 0);
    }

    #[test]
    fn test_fea_shell_membrane_bending_coupling_stiffness_init() {
        let mut coupling = StepFeaFeaShellMembraneBendingCouplingStiffness::new();
        let constants = vec![50.0, 60.0];
        coupling.init("Coupling".to_string(), constants);

        assert_eq!(coupling.name(), "Coupling");
        assert_eq!(coupling.fea_constants(), &[50.0, 60.0]);
    }

    #[test]
    fn test_fea_shell_membrane_bending_coupling_stiffness_setters() {
        let mut coupling = StepFeaFeaShellMembraneBendingCouplingStiffness::new();
        coupling.set_name("Test".to_string());
        coupling.set_fea_constants(vec![75.0]);

        assert_eq!(coupling.name(), "Test");
        assert_eq!(coupling.fea_constants(), &[75.0]);
    }
}
