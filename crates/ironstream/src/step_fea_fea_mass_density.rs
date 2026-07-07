// FILE: step_fea_fea_mass_density.rs
// occt: StepFEA_FeaMassDensity

/// Representation of STEP entity FeaMassDensity
#[derive(Debug, Clone)]
pub struct StepFeaFeaMassDensity {
    name: String,
    fea_constant: f64,
}

impl StepFeaFeaMassDensity {
    /// Creates a new empty FeaMassDensity
    pub fn new() -> Self {
        StepFeaFeaMassDensity {
            name: String::new(),
            fea_constant: 0.0,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, fea_constant: f64) {
        self.name = name;
        self.fea_constant = fea_constant;
    }

    /// Returns field FeaConstant
    pub fn fea_constant(&self) -> f64 {
        self.fea_constant
    }

    /// Set field FeaConstant
    pub fn set_fea_constant(&mut self, fea_constant: f64) {
        self.fea_constant = fea_constant;
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

impl Default for StepFeaFeaMassDensity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_mass_density_creation() {
        let density = StepFeaFeaMassDensity::new();
        assert_eq!(density.name(), "");
        assert_eq!(density.fea_constant(), 0.0);
    }

    #[test]
    fn test_fea_mass_density_init() {
        let mut density = StepFeaFeaMassDensity::new();
        density.init("Mass Density".to_string(), 7.85);

        assert_eq!(density.name(), "Mass Density");
        assert_eq!(density.fea_constant(), 7.85);
    }

    #[test]
    fn test_fea_mass_density_setters() {
        let mut density = StepFeaFeaMassDensity::new();
        density.set_name("Test".to_string());
        density.set_fea_constant(2.7);

        assert_eq!(density.name(), "Test");
        assert_eq!(density.fea_constant(), 2.7);
    }
}
