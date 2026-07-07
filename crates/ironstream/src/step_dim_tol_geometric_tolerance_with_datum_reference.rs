// FILE: step_dim_tol_geometric_tolerance_with_datum_reference.rs
// occt: StepDimTol_GeometricToleranceWithDatumReference

pub struct GeometricToleranceWithDatumReference {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub datum_system: Vec<String>,
}

impl GeometricToleranceWithDatumReference {
    pub fn new() -> Self {
        GeometricToleranceWithDatumReference {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            datum_system: Vec::new(),
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

    pub fn set_toleranced_shape_aspect(&mut self, aspect: String) {
        self.toleranced_shape_aspect = Some(aspect);
    }

    pub fn get_toleranced_shape_aspect(&self) -> Option<&str> {
        self.toleranced_shape_aspect.as_deref()
    }

    pub fn add_datum(&mut self, datum: String) {
        self.datum_system.push(datum);
    }

    pub fn get_datum_system(&self) -> &[String] {
        &self.datum_system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = GeometricToleranceWithDatumReference::new();
        assert!(tol.name.is_none());
        assert_eq!(tol.datum_system.len(), 0);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = GeometricToleranceWithDatumReference::new();
        tol.set_name("test".to_string());
        assert_eq!(tol.get_name(), Some("test"));
    }

    #[test]
    fn test_add_datum() {
        let mut tol = GeometricToleranceWithDatumReference::new();
        tol.add_datum("datum1".to_string());
        tol.add_datum("datum2".to_string());
        assert_eq!(tol.get_datum_system().len(), 2);
    }

    #[test]
    fn test_magnitude() {
        let mut tol = GeometricToleranceWithDatumReference::new();
        tol.set_magnitude("5.0".to_string());
        assert_eq!(tol.get_magnitude(), Some("5.0"));
    }
}
