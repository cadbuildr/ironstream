// FILE: step_dim_tol_symmetry_tolerance.rs
// occt: StepDimTol_SymmetryTolerance

pub struct SymmetryTolerance {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
}

impl SymmetryTolerance {
    pub fn new() -> Self {
        SymmetryTolerance {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
        }
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
        let tol = SymmetryTolerance::new();
        assert!(tol.name.is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = SymmetryTolerance::new();
        tol.set_name("symmetry".to_string());
        assert_eq!(tol.get_name(), Some("symmetry"));
    }

    #[test]
    fn test_set_magnitude() {
        let mut tol = SymmetryTolerance::new();
        tol.set_magnitude("0.3".to_string());
        assert_eq!(tol.get_magnitude(), Some("0.3"));
    }
}
