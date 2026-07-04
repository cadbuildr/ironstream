// FILE: step_fea_fea_secant_coefficient_of_linear_thermal_expansion.rs
// occt: StepFEA_FeaSecantCoefficientOfLinearThermalExpansion

/// Representation of STEP entity FeaSecantCoefficientOfLinearThermalExpansion
#[derive(Debug, Clone)]
pub struct StepFeaFeaSecantCoefficientOfLinearThermalExpansion {
    name: String,
    fea_constants: Vec<f64>,
    reference_temperature: f64,
}

impl StepFeaFeaSecantCoefficientOfLinearThermalExpansion {
    /// Creates a new empty FeaSecantCoefficientOfLinearThermalExpansion
    pub fn new() -> Self {
        StepFeaFeaSecantCoefficientOfLinearThermalExpansion {
            name: String::new(),
            fea_constants: Vec::new(),
            reference_temperature: 0.0,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, fea_constants: Vec<f64>, reference_temperature: f64) {
        self.name = name;
        self.fea_constants = fea_constants;
        self.reference_temperature = reference_temperature;
    }

    /// Returns field FeaConstants
    pub fn fea_constants(&self) -> &[f64] {
        &self.fea_constants
    }

    /// Set field FeaConstants
    pub fn set_fea_constants(&mut self, fea_constants: Vec<f64>) {
        self.fea_constants = fea_constants;
    }

    /// Returns field ReferenceTemperature
    pub fn reference_temperature(&self) -> f64 {
        self.reference_temperature
    }

    /// Set field ReferenceTemperature
    pub fn set_reference_temperature(&mut self, reference_temperature: f64) {
        self.reference_temperature = reference_temperature;
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

impl Default for StepFeaFeaSecantCoefficientOfLinearThermalExpansion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_secant_coefficient_creation() {
        let coeff = StepFeaFeaSecantCoefficientOfLinearThermalExpansion::new();
        assert_eq!(coeff.name(), "");
        assert_eq!(coeff.fea_constants().len(), 0);
        assert_eq!(coeff.reference_temperature(), 0.0);
    }

    #[test]
    fn test_fea_secant_coefficient_init() {
        let mut coeff = StepFeaFeaSecantCoefficientOfLinearThermalExpansion::new();
        let constants = vec![1.0e-6, 2.0e-6];
        coeff.init("Secant".to_string(), constants, 20.0);

        assert_eq!(coeff.name(), "Secant");
        assert_eq!(coeff.fea_constants(), &[1.0e-6, 2.0e-6]);
        assert_eq!(coeff.reference_temperature(), 20.0);
    }

    #[test]
    fn test_fea_secant_coefficient_setters() {
        let mut coeff = StepFeaFeaSecantCoefficientOfLinearThermalExpansion::new();
        coeff.set_name("Test".to_string());
        coeff.set_fea_constants(vec![0.5e-6]);
        coeff.set_reference_temperature(25.0);

        assert_eq!(coeff.name(), "Test");
        assert_eq!(coeff.fea_constants(), &[0.5e-6]);
        assert_eq!(coeff.reference_temperature(), 25.0);
    }
}
