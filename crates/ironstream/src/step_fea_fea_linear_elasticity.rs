// FILE: step_fea_fea_linear_elasticity.rs
// occt: StepFEA_FeaLinearElasticity

/// Representation of STEP entity FeaLinearElasticity
#[derive(Debug, Clone)]
pub struct StepFeaFeaLinearElasticity {
    name: String,
    fea_constants: Vec<f64>,
}

impl StepFeaFeaLinearElasticity {
    /// Creates a new empty FeaLinearElasticity
    pub fn new() -> Self {
        StepFeaFeaLinearElasticity {
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

impl Default for StepFeaFeaLinearElasticity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_linear_elasticity_creation() {
        let elasticity = StepFeaFeaLinearElasticity::new();
        assert_eq!(elasticity.name(), "");
        assert_eq!(elasticity.fea_constants().len(), 0);
    }

    #[test]
    fn test_fea_linear_elasticity_init() {
        let mut elasticity = StepFeaFeaLinearElasticity::new();
        let constants = vec![1.0, 2.0, 3.0];
        elasticity.init("Elasticity".to_string(), constants.clone());

        assert_eq!(elasticity.name(), "Elasticity");
        assert_eq!(elasticity.fea_constants(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_fea_linear_elasticity_setters() {
        let mut elasticity = StepFeaFeaLinearElasticity::new();
        elasticity.set_name("Test".to_string());
        elasticity.set_fea_constants(vec![4.0, 5.0]);

        assert_eq!(elasticity.name(), "Test");
        assert_eq!(elasticity.fea_constants(), &[4.0, 5.0]);
    }
}
