// FILE: step_dim_tol_unequally_disposed_geometric_tolerance.rs
// occt: StepDimTol_UnequallyDisposedGeometricTolerance

pub struct UnequallyDisposedGeometricTolerance {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub lower_displacement: Option<f64>,
    pub upper_displacement: Option<f64>,
}

impl UnequallyDisposedGeometricTolerance {
    pub fn new() -> Self {
        UnequallyDisposedGeometricTolerance {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            lower_displacement: None,
            upper_displacement: None,
        }
    }

    pub fn set_lower_displacement(&mut self, val: f64) {
        self.lower_displacement = Some(val);
    }

    pub fn get_lower_displacement(&self) -> Option<f64> {
        self.lower_displacement
    }

    pub fn set_upper_displacement(&mut self, val: f64) {
        self.upper_displacement = Some(val);
    }

    pub fn get_upper_displacement(&self) -> Option<f64> {
        self.upper_displacement
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = UnequallyDisposedGeometricTolerance::new();
        assert!(tol.lower_displacement.is_none());
        assert!(tol.upper_displacement.is_none());
    }

    #[test]
    fn test_set_displacements() {
        let mut tol = UnequallyDisposedGeometricTolerance::new();
        tol.set_lower_displacement(1.0);
        tol.set_upper_displacement(2.0);
        assert_eq!(tol.get_lower_displacement(), Some(1.0));
        assert_eq!(tol.get_upper_displacement(), Some(2.0));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = UnequallyDisposedGeometricTolerance::new();
        tol.set_name("uneq_disp".to_string());
        assert_eq!(tol.get_name(), Some("uneq_disp"));
    }
}
