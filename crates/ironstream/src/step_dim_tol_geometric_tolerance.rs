// FILE: step_dim_tol_geometric_tolerance.rs
// occt: StepDimTol_GeometricTolerance

pub struct GeometricTolerance {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
}

impl GeometricTolerance {
    pub fn new() -> Self {
        GeometricTolerance {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        magnitude: Option<String>,
        toleranced_shape_aspect: Option<String>,
    ) {
        self.name = name;
        self.description = description;
        self.magnitude = magnitude;
        self.toleranced_shape_aspect = toleranced_shape_aspect;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = GeometricTolerance::new();
        assert!(tol.name.is_none());
        assert!(tol.description.is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = GeometricTolerance::new();
        tol.set_name("test".to_string());
        assert_eq!(tol.get_name(), Some("test"));
    }

    #[test]
    fn test_init() {
        let mut tol = GeometricTolerance::new();
        tol.init(
            Some("name".to_string()),
            Some("desc".to_string()),
            Some("mag".to_string()),
            Some("asp".to_string()),
        );
        assert_eq!(tol.get_name(), Some("name"));
        assert_eq!(tol.get_description(), Some("desc"));
        assert_eq!(tol.get_magnitude(), Some("mag"));
        assert_eq!(tol.get_toleranced_shape_aspect(), Some("asp"));
    }

    #[test]
    fn test_set_description() {
        let mut tol = GeometricTolerance::new();
        tol.set_description("desc".to_string());
        assert_eq!(tol.get_description(), Some("desc"));
    }
}
