// FILE: step_geom_placement.rs
// occt: StepGeom_Placement

/// Represents a 3D placement (position and orientation)
pub struct StepGeomPlacement {
    /// Location [x, y, z]
    location: [f64; 3],
    /// Direction/orientation (3x3 rotation matrix as array)
    axis: [f64; 9],
}

impl StepGeomPlacement {
    pub fn new() -> Self {
        StepGeomPlacement {
            location: [0.0; 3],
            axis: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn with_location(x: f64, y: f64, z: f64) -> Self {
        StepGeomPlacement {
            location: [x, y, z],
            axis: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn location(&self) -> [f64; 3] {
        self.location
    }

    pub fn set_location(&mut self, x: f64, y: f64, z: f64) {
        self.location = [x, y, z];
    }

    pub fn axis(&self) -> [f64; 9] {
        self.axis
    }

    pub fn set_axis(&mut self, axis: [f64; 9]) {
        self.axis = axis;
    }
}

impl Default for StepGeomPlacement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_placement() {
        let placement = StepGeomPlacement::new();
        assert_eq!(placement.location(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_with_location() {
        let placement = StepGeomPlacement::with_location(1.0, 2.0, 3.0);
        assert_eq!(placement.location(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_set_location() {
        let mut placement = StepGeomPlacement::new();
        placement.set_location(5.0, 6.0, 7.0);
        assert_eq!(placement.location(), [5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_default_axis() {
        let placement = StepGeomPlacement::new();
        let axis = placement.axis();
        assert_eq!(axis[0], 1.0);
        assert_eq!(axis[4], 1.0);
        assert_eq!(axis[8], 1.0);
    }
}
