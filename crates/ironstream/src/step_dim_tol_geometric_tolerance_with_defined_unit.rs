// FILE: step_dim_tol_geometric_tolerance_with_defined_unit.rs
// occt: StepDimTol_GeometricToleranceWithDefinedUnit

pub struct GeometricToleranceWithDefinedUnit {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub unit: Option<String>,
}

impl GeometricToleranceWithDefinedUnit {
    pub fn new() -> Self {
        GeometricToleranceWithDefinedUnit {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            unit: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_magnitude(&mut self, magnitude: String) {
        self.magnitude = Some(magnitude);
    }

    pub fn get_magnitude(&self) -> Option<&str> {
        self.magnitude.as_deref()
    }

    pub fn set_unit(&mut self, unit: String) {
        self.unit = Some(unit);
    }

    pub fn get_unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = GeometricToleranceWithDefinedUnit::new();
        assert!(tol.unit.is_none());
    }

    #[test]
    fn test_set_unit() {
        let mut tol = GeometricToleranceWithDefinedUnit::new();
        tol.set_unit("mm".to_string());
        assert_eq!(tol.get_unit(), Some("mm"));
    }

    #[test]
    fn test_set_magnitude() {
        let mut tol = GeometricToleranceWithDefinedUnit::new();
        tol.set_magnitude("2.5".to_string());
        assert_eq!(tol.get_magnitude(), Some("2.5"));
    }
}
