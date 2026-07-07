// FILE: step_geom_point.rs
// occt: StepGeom_Point

/// Represents a point in STEP format
pub struct StepGeomPoint {
    name: String,
    /// Coordinates [x, y, z]
    coords: [f64; 3],
}

impl StepGeomPoint {
    pub fn new(name: String, x: f64, y: f64, z: f64) -> Self {
        StepGeomPoint {
            name,
            coords: [x, y, z],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn coords(&self) -> [f64; 3] {
        self.coords
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    pub fn z(&self) -> f64 {
        self.coords[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point() {
        let point = StepGeomPoint::new("Point1".to_string(), 1.0, 2.0, 3.0);
        assert_eq!(point.name(), "Point1");
        assert_eq!(point.coords(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_coordinates() {
        let point = StepGeomPoint::new("Point1".to_string(), 5.5, 6.5, 7.5);
        assert_eq!(point.x(), 5.5);
        assert_eq!(point.y(), 6.5);
        assert_eq!(point.z(), 7.5);
    }
}
