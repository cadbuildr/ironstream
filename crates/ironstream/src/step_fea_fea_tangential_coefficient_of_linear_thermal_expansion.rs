// FILE: step_fea_fea_tangential_coefficient_of_linear_thermal_expansion.rs
// occt: StepFEA_FeaTangentialCoefficientOfLinearThermalExpansion

/// Representation of STEP entity FeaTangentialCoefficientOfLinearThermalExpansion
#[derive(Debug, Clone)]
pub struct StepFeaFeaTangentialCoefficientOfLinearThermalExpansion {
    name: String,
    fea_constants: Vec<f64>,
}

impl StepFeaFeaTangentialCoefficientOfLinearThermalExpansion {
    /// Creates a new empty FeaTangentialCoefficientOfLinearThermalExpansion
    pub fn new() -> Self {
        StepFeaFeaTangentialCoefficientOfLinearThermalExpansion {
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

impl Default for StepFeaFeaTangentialCoefficientOfLinearThermalExpansion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_tangential_coefficient_creation() {
        let coeff = StepFeaFeaTangentialCoefficientOfLinearThermalExpansion::new();
        assert_eq!(coeff.name(), "");
        assert_eq!(coeff.fea_constants().len(), 0);
    }

    #[test]
    fn test_fea_tangential_coefficient_init() {
        let mut coeff = StepFeaFeaTangentialCoefficientOfLinearThermalExpansion::new();
        let constants = vec![2.0e-6, 3.0e-6];
        coeff.init("Tangential".to_string(), constants);

        assert_eq!(coeff.name(), "Tangential");
        assert_eq!(coeff.fea_constants(), &[2.0e-6, 3.0e-6]);
    }

    #[test]
    fn test_fea_tangential_coefficient_setters() {
        let mut coeff = StepFeaFeaTangentialCoefficientOfLinearThermalExpansion::new();
        coeff.set_name("Test".to_string());
        coeff.set_fea_constants(vec![1.5e-6]);

        assert_eq!(coeff.name(), "Test");
        assert_eq!(coeff.fea_constants(), &[1.5e-6]);
    }
}
