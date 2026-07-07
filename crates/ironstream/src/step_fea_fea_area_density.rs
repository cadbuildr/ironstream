// FILE: step_fea_fea_area_density.rs
// occt: StepFEA_FeaAreaDensity

/// Representation of STEP entity FeaAreaDensity
#[derive(Debug, Clone)]
pub struct StepFeaFeaAreaDensity {
    name: String,
    fea_constant: f64,
}

impl StepFeaFeaAreaDensity {
    /// Creates a new empty FeaAreaDensity
    pub fn new() -> Self {
        StepFeaFeaAreaDensity {
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

impl Default for StepFeaFeaAreaDensity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_area_density_creation() {
        let density = StepFeaFeaAreaDensity::new();
        assert_eq!(density.name(), "");
        assert_eq!(density.fea_constant(), 0.0);
    }

    #[test]
    fn test_fea_area_density_init() {
        let mut density = StepFeaFeaAreaDensity::new();
        density.init("Area Density".to_string(), 2.5);

        assert_eq!(density.name(), "Area Density");
        assert_eq!(density.fea_constant(), 2.5);
    }

    #[test]
    fn test_fea_area_density_setters() {
        let mut density = StepFeaFeaAreaDensity::new();
        density.set_name("Test".to_string());
        density.set_fea_constant(3.14);

        assert_eq!(density.name(), "Test");
        assert_eq!(density.fea_constant(), 3.14);
    }

    #[test]
    fn test_fea_area_density_clone() {
        let mut density = StepFeaFeaAreaDensity::new();
        density.init("Test".to_string(), 1.5);
        let cloned = density.clone();

        assert_eq!(cloned.name(), "Test");
        assert_eq!(cloned.fea_constant(), 1.5);
    }

    #[test]
    fn test_fea_area_density_default() {
        let density = StepFeaFeaAreaDensity::default();
        assert_eq!(density.name(), "");
        assert_eq!(density.fea_constant(), 0.0);
    }
}
