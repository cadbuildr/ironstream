// FILE: step_fea_constant_surface3d_element_coordinate_system.rs
// occt: StepFEA_ConstantSurface3dElementCoordinateSystem

/// Representation of STEP entity ConstantSurface3dElementCoordinateSystem.
#[derive(Clone)]
pub struct ConstantSurface3dElementCoordinateSystem {
    name: Option<String>,
    axis: i32,
    angle: f64,
}

impl ConstantSurface3dElementCoordinateSystem {
    /// Creates a new ConstantSurface3dElementCoordinateSystem.
    pub fn new() -> Self {
        Self {
            name: None,
            axis: 0,
            angle: 0.0,
        }
    }

    /// Initializes all fields.
    pub fn init(&mut self, name: Option<String>, axis: i32, angle: f64) {
        self.name = name;
        self.axis = axis;
        self.angle = angle;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, n: Option<String>) {
        self.name = n;
    }

    pub fn axis(&self) -> i32 {
        self.axis
    }

    pub fn set_axis(&mut self, a: i32) {
        self.axis = a;
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, a: f64) {
        self.angle = a;
    }
}

impl Default for ConstantSurface3dElementCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let sys = ConstantSurface3dElementCoordinateSystem::new();
        assert!(sys.name().is_none());
        assert_eq!(sys.axis(), 0);
        assert_eq!(sys.angle(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut sys = ConstantSurface3dElementCoordinateSystem::new();
        sys.init(Some("ConstSurf".to_string()), 2, 45.0);

        assert_eq!(sys.name(), Some("ConstSurf"));
        assert_eq!(sys.axis(), 2);
        assert_eq!(sys.angle(), 45.0);
    }

    #[test]
    fn test_setters() {
        let mut sys = ConstantSurface3dElementCoordinateSystem::new();
        sys.set_name(Some("System".to_string()));
        sys.set_axis(1);
        sys.set_angle(90.0);

        assert_eq!(sys.name(), Some("System"));
        assert_eq!(sys.axis(), 1);
        assert_eq!(sys.angle(), 90.0);
    }
}
