// FILE: step_dim_tol_geometric_tolerance_with_maximum_tolerance.rs
// occt: StepDimTol_GeometricToleranceWithMaximumTolerance

pub struct GeometricToleranceWithMaximumTolerance {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub maximum_tolerance: Option<f64>,
}

impl GeometricToleranceWithMaximumTolerance {
    pub fn new() -> Self {
        GeometricToleranceWithMaximumTolerance {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            maximum_tolerance: None,
        }
    }

    pub fn set_maximum_tolerance(&mut self, val: f64) {
        self.maximum_tolerance = Some(val);
    }

    pub fn get_maximum_tolerance(&self) -> Option<f64> {
        self.maximum_tolerance
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_magnitude(&mut self, magnitude: String) {
        self.magnitude = Some(magnitude);
    }

    pub fn get_magnitude(&self) -> Option<&str> {
        self.magnitude.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = GeometricToleranceWithMaximumTolerance::new();
        assert!(tol.maximum_tolerance.is_none());
    }

    #[test]
    fn test_set_maximum_tolerance() {
        let mut tol = GeometricToleranceWithMaximumTolerance::new();
        tol.set_maximum_tolerance(10.5);
        assert_eq!(tol.get_maximum_tolerance(), Some(10.5));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = GeometricToleranceWithMaximumTolerance::new();
        tol.set_name("max_tol".to_string());
        assert_eq!(tol.get_name(), Some("max_tol"));
    }
}
