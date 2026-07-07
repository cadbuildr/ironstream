// FILE: step_fea_fea_moisture_absorption.rs
// occt: StepFEA_FeaMoistureAbsorption

/// Representation of STEP entity FeaMoistureAbsorption
#[derive(Debug, Clone)]
pub struct StepFeaFeaMoistureAbsorption {
    name: String,
    fea_constants: Vec<f64>,
}

impl StepFeaFeaMoistureAbsorption {
    /// Creates a new empty FeaMoistureAbsorption
    pub fn new() -> Self {
        StepFeaFeaMoistureAbsorption {
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

impl Default for StepFeaFeaMoistureAbsorption {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_moisture_absorption_creation() {
        let absorption = StepFeaFeaMoistureAbsorption::new();
        assert_eq!(absorption.name(), "");
        assert_eq!(absorption.fea_constants().len(), 0);
    }

    #[test]
    fn test_fea_moisture_absorption_init() {
        let mut absorption = StepFeaFeaMoistureAbsorption::new();
        let constants = vec![0.1, 0.2];
        absorption.init("Moisture".to_string(), constants);

        assert_eq!(absorption.name(), "Moisture");
        assert_eq!(absorption.fea_constants(), &[0.1, 0.2]);
    }

    #[test]
    fn test_fea_moisture_absorption_setters() {
        let mut absorption = StepFeaFeaMoistureAbsorption::new();
        absorption.set_name("Test".to_string());
        absorption.set_fea_constants(vec![0.5]);

        assert_eq!(absorption.name(), "Test");
        assert_eq!(absorption.fea_constants(), &[0.5]);
    }
}
