// FILE: step_geom_plane.rs
// occt: StepGeom_Plane

/// Represents a plane in STEP format
pub struct StepGeomPlane {
    name: String,
    /// Position [x, y, z]
    position: [f64; 3],
}

impl StepGeomPlane {
    pub fn new(name: String) -> Self {
        StepGeomPlane {
            name,
            position: [0.0; 3],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> [f64; 3] {
        self.position
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = [x, y, z];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_plane() {
        let plane = StepGeomPlane::new("Plane1".to_string());
        assert_eq!(plane.name(), "Plane1");
    }

    #[test]
    fn test_set_position() {
        let mut plane = StepGeomPlane::new("Plane1".to_string());
        plane.set_position(1.0, 2.0, 3.0);
        assert_eq!(plane.position(), [1.0, 2.0, 3.0]);
    }
}
