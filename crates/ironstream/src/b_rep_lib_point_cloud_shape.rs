// FILE: b_rep_lib_point_cloud_shape.rs
// occt: BRepLib_PointCloudShape

use std::collections::HashMap;

/// Tool for generating point clouds from shapes.
#[derive(Debug, Clone)]
pub struct BRepLibPointCloudShape {
    tolerance: f64,
    distance: f64,
    nb_points: i32,
    face_area: HashMap<String, f64>,
}

impl BRepLibPointCloudShape {
    /// Create with default tolerance.
    pub fn new() -> Self {
        BRepLibPointCloudShape {
            tolerance: 0.0000001,
            distance: 0.0,
            nb_points: 0,
            face_area: HashMap::new(),
        }
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn set_distance(&mut self, dist: f64) {
        if dist >= 0.0 {
            self.distance = dist;
        }
    }

    pub fn nb_points(&self) -> i32 {
        self.nb_points
    }
}

impl Default for BRepLibPointCloudShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_cloud_creation() {
        let pc = BRepLibPointCloudShape::new();
        assert!(pc.tolerance() > 0.0);
        assert_eq!(pc.distance(), 0.0);
        assert_eq!(pc.nb_points(), 0);
    }

    #[test]
    fn test_distance_setter() {
        let mut pc = BRepLibPointCloudShape::new();
        pc.set_distance(10.0);
        assert_eq!(pc.distance(), 10.0);
        pc.set_distance(-5.0);
        assert_eq!(pc.distance(), 10.0); // negative ignored
    }
}
