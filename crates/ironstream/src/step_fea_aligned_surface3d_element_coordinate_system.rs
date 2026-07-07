// FILE: step_fea_aligned_surface3d_element_coordinate_system.rs
// occt: StepFEA_AlignedSurface3dElementCoordinateSystem

/// Representation of STEP entity AlignedSurface3dElementCoordinateSystem.
#[derive(Clone)]
pub struct AlignedSurface3dElementCoordinateSystem {
    name: Option<String>,
    coordinate_system: Option<String>,
}

impl AlignedSurface3dElementCoordinateSystem {
    /// Creates a new AlignedSurface3dElementCoordinateSystem.
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

impl Default for AlignedSurface3dElementCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let sys = AlignedSurface3dElementCoordinateSystem::new();
        assert!(sys.name().is_none());
        assert!(sys.coordinate_system().is_none());
    }

    #[test]
    fn test_init() {
        let mut sys = AlignedSurface3dElementCoordinateSystem::new();
        sys.init(
            Some("SurfaceSystem".to_string()),
            Some("Aligned".to_string()),
        );

        assert_eq!(sys.name(), Some("SurfaceSystem"));
        assert_eq!(sys.coordinate_system(), Some("Aligned"));
    }

    #[test]
    fn test_setters() {
        let mut sys = AlignedSurface3dElementCoordinateSystem::new();
        sys.set_name(Some("System3d".to_string()));
        sys.set_coordinate_system(Some("Aligned3d".to_string()));

        assert_eq!(sys.name(), Some("System3d"));
        assert_eq!(sys.coordinate_system(), Some("Aligned3d"));
    }
}
