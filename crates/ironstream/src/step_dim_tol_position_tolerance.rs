// FILE: step_dim_tol_position_tolerance.rs
// occt: StepDimTol_PositionTolerance

pub struct PositionTolerance {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
}

impl PositionTolerance {
    pub fn new() -> Self {
        PositionTolerance {
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
        let tol = PositionTolerance::new();
        assert!(tol.name.is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = PositionTolerance::new();
        tol.set_name("position".to_string());
        assert_eq!(tol.get_name(), Some("position"));
    }

    #[test]
    fn test_set_magnitude() {
        let mut tol = PositionTolerance::new();
        tol.set_magnitude("0.1".to_string());
        assert_eq!(tol.get_magnitude(), Some("0.1"));
    }
}
