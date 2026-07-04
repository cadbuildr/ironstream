// FILE: step_fea_parametric_curve3d_element_coordinate_direction.rs
// occt: StepFEA_ParametricCurve3dElementCoordinateDirection

/// Representation of STEP entity ParametricCurve3dElementCoordinateDirection
#[derive(Debug, Clone)]
pub struct StepFeaParametricCurve3dElementCoordinateDirection {
    name: String,
    orientation: Option<i32>,
}

impl StepFeaParametricCurve3dElementCoordinateDirection {
    /// Creates a new empty ParametricCurve3dElementCoordinateDirection
    pub fn new() -> Self {
        StepFeaParametricCurve3dElementCoordinateDirection {
            name: String::new(),
            orientation: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, orientation: Option<i32>) {
        self.name = name;
        self.orientation = orientation;
    }

    /// Returns field Orientation
    pub fn orientation(&self) -> Option<i32> {
        self.orientation
    }

    /// Set field Orientation
    pub fn set_orientation(&mut self, orientation: Option<i32>) {
        self.orientation = orientation;
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

impl Default for StepFeaParametricCurve3dElementCoordinateDirection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parametric_curve3d_element_coordinate_direction_creation() {
        let dir = StepFeaParametricCurve3dElementCoordinateDirection::new();
        assert_eq!(dir.name(), "");
        assert_eq!(dir.orientation(), None);
    }

    #[test]
    fn test_parametric_curve3d_element_coordinate_direction_init() {
        let mut dir = StepFeaParametricCurve3dElementCoordinateDirection::new();
        dir.init("Direction".to_string(), Some(1));

        assert_eq!(dir.name(), "Direction");
        assert_eq!(dir.orientation(), Some(1));
    }

    #[test]
    fn test_parametric_curve3d_element_coordinate_direction_setters() {
        let mut dir = StepFeaParametricCurve3dElementCoordinateDirection::new();
        dir.set_name("Test".to_string());
        dir.set_orientation(Some(2));

        assert_eq!(dir.name(), "Test");
        assert_eq!(dir.orientation(), Some(2));
    }
}
