// FILE: step_geom_cartesian_point.rs
// occt: StepGeom_CartesianPoint

use std::sync::Arc;

/// CartesianPoint: A point defined by X, Y, Z coordinates (or 2D equivalent).
#[derive(Clone)]
pub struct CartesianPoint {
    name: Arc<String>,
    nb_coord: i32,
    coordinates: [f64; 3],
}

impl CartesianPoint {
    /// Creates a new CartesianPoint.
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            nb_coord: 0,
            coordinates: [0.0; 3],
        }
    }

    /// Initializes with a name and coordinate array.
    pub fn init(&mut self, name: String, coordinates: [f64; 3]) {
        self.name = Arc::new(name);
        self.coordinates = coordinates;
        self.nb_coord = 3;
    }

    /// Initializes as a 2D point.
    pub fn init_2d(&mut self, name: String, x: f64, y: f64) {
        self.name = Arc::new(name);
        self.coordinates = [x, y, 0.0];
        self.nb_coord = 2;
    }

    /// Initializes as a 3D point.
    pub fn init_3d(&mut self, name: String, x: f64, y: f64, z: f64) {
        self.name = Arc::new(name);
        self.coordinates = [x, y, z];
        self.nb_coord = 3;
    }

    /// Sets all coordinates.
    pub fn set_coordinates(&mut self, coordinates: [f64; 3]) {
        self.coordinates = coordinates;
    }

    /// Returns the coordinate array.
    pub fn coordinates(&self) -> [f64; 3] {
        self.coordinates
    }

    /// Returns a single coordinate by index (1-based).
    pub fn coordinates_value(&self, index: i32) -> Option<f64> {
        match index {
            1 => Some(self.coordinates[0]),
            2 => Some(self.coordinates[1]),
            3 => Some(self.coordinates[2]),
            _ => None,
        }
    }

    /// Sets the number of coordinates (2 or 3).
    pub fn set_nb_coordinates(&mut self, size: i32) {
        self.nb_coord = size;
    }

    /// Returns the number of coordinates.
    pub fn nb_coordinates(&self) -> i32 {
        self.nb_coord
    }

    /// Returns the name.
    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for CartesianPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pt = CartesianPoint::new();
        assert_eq!(pt.name(), "");
        assert_eq!(pt.nb_coordinates(), 0);
    }

    #[test]
    fn test_init_3d() {
        let mut pt = CartesianPoint::new();
        pt.init_3d("origin".to_string(), 0.0, 0.0, 0.0);
        assert_eq!(pt.name(), "origin");
        assert_eq!(pt.nb_coordinates(), 3);
        assert_eq!(pt.coordinates_value(1), Some(0.0));
    }

    #[test]
    fn test_init_2d() {
        let mut pt = CartesianPoint::new();
        pt.init_2d("point2d".to_string(), 1.0, 2.0);
        assert_eq!(pt.nb_coordinates(), 2);
        assert_eq!(pt.coordinates_value(1), Some(1.0));
        assert_eq!(pt.coordinates_value(2), Some(2.0));
    }

    #[test]
    fn test_set_coordinates() {
        let mut pt = CartesianPoint::new();
        pt.set_coordinates([3.0, 4.0, 5.0]);
        assert_eq!(pt.coordinates()[0], 3.0);
        assert_eq!(pt.coordinates()[1], 4.0);
        assert_eq!(pt.coordinates()[2], 5.0);
    }

    #[test]
    fn test_invalid_index() {
        let pt = CartesianPoint::new();
        assert_eq!(pt.coordinates_value(0), None);
        assert_eq!(pt.coordinates_value(4), None);
    }
}
