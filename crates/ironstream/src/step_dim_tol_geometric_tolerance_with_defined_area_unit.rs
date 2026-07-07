// FILE: step_dim_tol_geometric_tolerance_with_defined_area_unit.rs
// occt: StepDimTol_GeometricToleranceWithDefinedAreaUnit

pub struct GeometricToleranceWithDefinedAreaUnit {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub area_unit: Option<String>,
}

impl GeometricToleranceWithDefinedAreaUnit {
    pub fn new() -> Self {
        GeometricToleranceWithDefinedAreaUnit {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            area_unit: None,
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

    pub fn set_area_unit(&mut self, unit: String) {
        self.area_unit = Some(unit);
    }

    pub fn get_area_unit(&self) -> Option<&str> {
        self.area_unit.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = GeometricToleranceWithDefinedAreaUnit::new();
        assert!(tol.area_unit.is_none());
    }

    #[test]
    fn test_set_area_unit() {
        let mut tol = GeometricToleranceWithDefinedAreaUnit::new();
        tol.set_area_unit("mm2".to_string());
        assert_eq!(tol.get_area_unit(), Some("mm2"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = GeometricToleranceWithDefinedAreaUnit::new();
        tol.set_name("tol".to_string());
        assert_eq!(tol.get_name(), Some("tol"));
    }
}
