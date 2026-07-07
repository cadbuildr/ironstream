// FILE: step_fea_fea_parametric_point.rs
// occt: StepFEA_FeaParametricPoint

/// Representation of STEP entity FeaParametricPoint
#[derive(Debug, Clone)]
pub struct StepFeaFeaParametricPoint {
    name: String,
    coordinates: Vec<f64>,
}

impl StepFeaFeaParametricPoint {
    /// Creates a new empty FeaParametricPoint
    pub fn new() -> Self {
        StepFeaFeaParametricPoint {
            name: String::new(),
            coordinates: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, coordinates: Vec<f64>) {
        self.name = name;
        self.coordinates = coordinates;
    }

    /// Returns field Coordinates
    pub fn coordinates(&self) -> &[f64] {
        &self.coordinates
    }

    /// Set field Coordinates
    pub fn set_coordinates(&mut self, coordinates: Vec<f64>) {
        self.coordinates = coordinates;
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

impl Default for StepFeaFeaParametricPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_parametric_point_creation() {
        let point = StepFeaFeaParametricPoint::new();
        assert_eq!(point.name(), "");
        assert_eq!(point.coordinates().len(), 0);
    }

    #[test]
    fn test_fea_parametric_point_init() {
        let mut point = StepFeaFeaParametricPoint::new();
        let coords = vec![1.0, 2.0, 3.0];
        point.init("Point".to_string(), coords);

        assert_eq!(point.name(), "Point");
        assert_eq!(point.coordinates(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_fea_parametric_point_setters() {
        let mut point = StepFeaFeaParametricPoint::new();
        point.set_name("Test".to_string());
        point.set_coordinates(vec![4.0, 5.0]);

        assert_eq!(point.name(), "Test");
        assert_eq!(point.coordinates(), &[4.0, 5.0]);
    }
}
