// FILE: step_fea_parametric_surface3d_element_coordinate_system.rs
// occt: StepFEA_ParametricSurface3dElementCoordinateSystem

/// Representation of STEP entity ParametricSurface3dElementCoordinateSystem
#[derive(Debug, Clone)]
pub struct StepFeaParametricSurface3dElementCoordinateSystem {
    name: String,
    axis: i32,
    angle: f64,
}

impl StepFeaParametricSurface3dElementCoordinateSystem {
    /// Creates a new empty ParametricSurface3dElementCoordinateSystem
    pub fn new() -> Self {
        StepFeaParametricSurface3dElementCoordinateSystem {
            name: String::new(),
            axis: 0,
            angle: 0.0,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, axis: i32, angle: f64) {
        self.name = name;
        self.axis = axis;
        self.angle = angle;
    }

    /// Returns field Axis
    pub fn axis(&self) -> i32 {
        self.axis
    }

    /// Set field Axis
    pub fn set_axis(&mut self, axis: i32) {
        self.axis = axis;
    }

    /// Returns field Angle
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Set field Angle
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    /// Returns field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepFeaParametricSurface3dElementCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parametric_surface3d_element_coordinate_system_creation() {
        let sys = StepFeaParametricSurface3dElementCoordinateSystem::new();
        assert_eq!(sys.name(), "");
        assert_eq!(sys.axis(), 0);
        assert_eq!(sys.angle(), 0.0);
    }

    #[test]
    fn test_parametric_surface3d_element_coordinate_system_init() {
        let mut sys = StepFeaParametricSurface3dElementCoordinateSystem::new();
        sys.init("System".to_string(), 1, 45.0);

        assert_eq!(sys.name(), "System");
        assert_eq!(sys.axis(), 1);
        assert_eq!(sys.angle(), 45.0);
    }

    #[test]
    fn test_parametric_surface3d_element_coordinate_system_setters() {
        let mut sys = StepFeaParametricSurface3dElementCoordinateSystem::new();
        sys.set_name("Test".to_string());
        sys.set_axis(2);
        sys.set_angle(90.0);

        assert_eq!(sys.name(), "Test");
        assert_eq!(sys.axis(), 2);
        assert_eq!(sys.angle(), 90.0);
    }
}
