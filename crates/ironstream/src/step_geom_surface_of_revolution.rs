// FILE: step_geom_surface_of_revolution.rs
// occt: StepGeom_SurfaceOfRevolution

/// Represents a surface of revolution (created by rotating a curve around an axis)
pub struct StepGeomSurfaceOfRevolution {
    name: String,
    curve_id: i32,
    /// Rotation axis position [x, y, z]
    axis_position: [f64; 3],
    /// Rotation axis direction [x, y, z]
    axis_direction: [f64; 3],
}

impl StepGeomSurfaceOfRevolution {
    pub fn new(
        name: String,
        curve_id: i32,
        axis_pos: [f64; 3],
        axis_dir: [f64; 3],
    ) -> Self {
        StepGeomSurfaceOfRevolution {
            name,
            curve_id,
            axis_position: axis_pos,
            axis_direction: axis_dir,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn curve_id(&self) -> i32 {
        self.curve_id
    }

    pub fn axis_position(&self) -> [f64; 3] {
        self.axis_position
    }

    pub fn axis_direction(&self) -> [f64; 3] {
        self.axis_direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface() {
        let surface = StepGeomSurfaceOfRevolution::new(
            "Surface1".to_string(),
            1,
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        );
        assert_eq!(surface.name(), "Surface1");
        assert_eq!(surface.curve_id(), 1);
    }
}
