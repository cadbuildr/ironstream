// FILE: step_fea_parametric_curve3d_element_coordinate_system.rs
// occt: StepFEA_ParametricCurve3dElementCoordinateSystem

/// Representation of STEP entity ParametricCurve3dElementCoordinateSystem
#[derive(Debug, Clone)]
pub struct StepFeaParametricCurve3dElementCoordinateSystem {
    name: String,
    direction: Option<i32>,
}

impl StepFeaParametricCurve3dElementCoordinateSystem {
    /// Creates a new empty ParametricCurve3dElementCoordinateSystem
    pub fn new() -> Self {
        StepFeaParametricCurve3dElementCoordinateSystem {
            name: String::new(),
            direction: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, direction: Option<i32>) {
        self.name = name;
        self.direction = direction;
    }

    /// Returns field Direction
    pub fn direction(&self) -> Option<i32> {
        self.direction
    }

    /// Set field Direction
    pub fn set_direction(&mut self, direction: Option<i32>) {
        self.direction = direction;
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

impl Default for StepFeaParametricCurve3dElementCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parametric_curve3d_element_coordinate_system_creation() {
        let sys = StepFeaParametricCurve3dElementCoordinateSystem::new();
        assert_eq!(sys.name(), "");
        assert_eq!(sys.direction(), None);
    }

    #[test]
    fn test_parametric_curve3d_element_coordinate_system_init() {
        let mut sys = StepFeaParametricCurve3dElementCoordinateSystem::new();
        sys.init("System".to_string(), Some(1));

        assert_eq!(sys.name(), "System");
        assert_eq!(sys.direction(), Some(1));
    }

    #[test]
    fn test_parametric_curve3d_element_coordinate_system_setters() {
        let mut sys = StepFeaParametricCurve3dElementCoordinateSystem::new();
        sys.set_name("Test".to_string());
        sys.set_direction(Some(2));

        assert_eq!(sys.name(), "Test");
        assert_eq!(sys.direction(), Some(2));
    }
}
