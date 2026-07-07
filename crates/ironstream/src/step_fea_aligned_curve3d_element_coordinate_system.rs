// FILE: step_fea_aligned_curve3d_element_coordinate_system.rs
// occt: StepFEA_AlignedCurve3dElementCoordinateSystem

/// Representation of STEP entity AlignedCurve3dElementCoordinateSystem.
#[derive(Clone)]
pub struct AlignedCurve3dElementCoordinateSystem {
    name: Option<String>,
    coordinate_system: Option<String>,
}

impl AlignedCurve3dElementCoordinateSystem {
    /// Creates a new AlignedCurve3dElementCoordinateSystem.
    pub fn new() -> Self {
        Self {
            name: None,
            coordinate_system: None,
        }
    }

    /// Initializes all fields.
    pub fn init(&mut self, name: Option<String>, coordinate_system: Option<String>) {
        self.name = name;
        self.coordinate_system = coordinate_system;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, n: Option<String>) {
        self.name = n;
    }

    pub fn coordinate_system(&self) -> Option<&str> {
        self.coordinate_system.as_deref()
    }

    pub fn set_coordinate_system(&mut self, cs: Option<String>) {
        self.coordinate_system = cs;
    }
}

impl Default for AlignedCurve3dElementCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let sys = AlignedCurve3dElementCoordinateSystem::new();
        assert!(sys.name().is_none());
        assert!(sys.coordinate_system().is_none());
    }

    #[test]
    fn test_init() {
        let mut sys = AlignedCurve3dElementCoordinateSystem::new();
        sys.init(
            Some("System1".to_string()),
            Some("Cartesian".to_string()),
        );

        assert_eq!(sys.name(), Some("System1"));
        assert_eq!(sys.coordinate_system(), Some("Cartesian"));
    }

    #[test]
    fn test_setters() {
        let mut sys = AlignedCurve3dElementCoordinateSystem::new();
        sys.set_name(Some("CurveSystem".to_string()));
        sys.set_coordinate_system(Some("Aligned".to_string()));

        assert_eq!(sys.name(), Some("CurveSystem"));
        assert_eq!(sys.coordinate_system(), Some("Aligned"));
    }
}
