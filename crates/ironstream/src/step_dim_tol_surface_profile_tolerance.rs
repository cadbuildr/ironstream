// FILE: step_dim_tol_surface_profile_tolerance.rs
// occt: StepDimTol_SurfaceProfileTolerance

pub struct SurfaceProfileTolerance {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
}

impl SurfaceProfileTolerance {
    pub fn new() -> Self {
        SurfaceProfileTolerance {
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
        let tol = SurfaceProfileTolerance::new();
        assert!(tol.name.is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = SurfaceProfileTolerance::new();
        tol.set_name("surface_profile".to_string());
        assert_eq!(tol.get_name(), Some("surface_profile"));
    }

    #[test]
    fn test_set_magnitude() {
        let mut tol = SurfaceProfileTolerance::new();
        tol.set_magnitude("1.2".to_string());
        assert_eq!(tol.get_magnitude(), Some("1.2"));
    }
}
