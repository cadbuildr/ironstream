// FILE: step_geom_surface_of_linear_extrusion.rs
// occt: StepGeom_SurfaceOfLinearExtrusion

/// Represents a surface of linear extrusion (sweep along a direction)
pub struct StepGeomSurfaceOfLinearExtrusion {
    name: String,
    curve_id: i32,
    /// Extrusion direction [x, y, z]
    extrusion_direction: [f64; 3],
}

impl StepGeomSurfaceOfLinearExtrusion {
    pub fn new(name: String, curve_id: i32, direction: [f64; 3]) -> Self {
        StepGeomSurfaceOfLinearExtrusion {
            name,
            curve_id,
            extrusion_direction: direction,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn curve_id(&self) -> i32 {
        self.curve_id
    }

    pub fn extrusion_direction(&self) -> [f64; 3] {
        self.extrusion_direction
    }

    pub fn set_extrusion_direction(&mut self, direction: [f64; 3]) {
        self.extrusion_direction = direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface() {
        let surface = StepGeomSurfaceOfLinearExtrusion::new(
            "Surface1".to_string(),
            1,
            [0.0, 0.0, 1.0],
        );
        assert_eq!(surface.name(), "Surface1");
        assert_eq!(surface.curve_id(), 1);
        assert_eq!(surface.extrusion_direction(), [0.0, 0.0, 1.0]);
    }
}
