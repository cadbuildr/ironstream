// FILE: b_rep_mesh_circle_inspector.rs
// occt: BRepMesh_CircleInspector

use crate::b_rep_mesh_circle::Circle;

/// Auxiliary class to find circles shot by the given point.
pub struct CircleInspector {
    sq_tolerance: f64,
    circles: Vec<Circle>,
    shot_circle_indices: Vec<i32>,
    reference_point: (f64, f64),
}

impl CircleInspector {
    /// Constructor.
    /// @param tolerance Tolerance to be used for identification of shot circles.
    /// @param reserved_size Size to be reserved for vector of circles.
    pub fn new(tolerance: f64, reserved_size: usize) -> Self {
        CircleInspector {
            sq_tolerance: tolerance * tolerance,
            circles: Vec::with_capacity(reserved_size),
            shot_circle_indices: Vec::new(),
            reference_point: (0.0, 0.0),
        }
    }

    /// Dimension of the space (2D)
    pub const DIMENSION: i32 = 2;

    /// Adds the circle to vector of circles at the given position.
    /// @param index Position of circle in the vector.
    /// @param circle Circle to be added.
    pub fn bind(&mut self, index: usize, circle: Circle) {
        if index >= self.circles.len() {
            self.circles.resize(index + 1, Circle::new());
        }
        self.circles[index] = circle;
    }

    /// Returns the vector of registered circles.
    pub fn circles(&self) -> &[Circle] {
        &self.circles
    }

    /// Returns circle with the given index.
    pub fn circle(&self, index: usize) -> Option<&Circle> {
        self.circles.get(index)
    }

    /// Set reference point to be checked.
    /// @param point_x X coordinate of reference point
    /// @param point_y Y coordinate of reference point
    pub fn set_point(&mut self, point_x: f64, point_y: f64) {
        self.shot_circle_indices.clear();
        self.reference_point = (point_x, point_y);
    }

    /// Returns list of indices of circles shot by the reference point.
    pub fn shot_circles(&self) -> &[i32] {
        &self.shot_circle_indices
    }

    /// Performs inspection of a circle with the given index.
    /// @param index Index of circle to be checked.
    /// @return true if circle was processed, false if it should be purged
    pub fn inspect(&mut self, index: usize) -> bool {
        if index >= self.circles.len() {
            return false;
        }

        let circle = &self.circles[index];
        let radius = circle.radius();

        // If radius is negative, purge this circle
        if radius < 0.0 {
            return false;
        }

        let dx = self.reference_point.0 - circle.location_x();
        let dy = self.reference_point.1 - circle.location_y();
        let dist_sq = dx * dx + dy * dy;
        let radius_sq = radius * radius;

        // Check if point is within tolerance of circle boundary
        if (dist_sq - radius_sq).abs() <= self.sq_tolerance {
            self.shot_circle_indices.push(index as i32);
        }

        true
    }

    /// Checks if two indices are equal.
    pub fn is_equal(index: i32, target_index: i32) -> bool {
        index == target_index
    }

    /// Coordinate accessor for a point.
    pub fn coord(i: i32, point_x: f64, point_y: f64) -> f64 {
        match i {
            0 => point_x,
            1 => point_y,
            _ => 0.0,
        }
    }

    /// Shift a point by the given tolerance.
    pub fn shift(point_x: f64, point_y: f64, tolerance: f64) -> (f64, f64) {
        (point_x + tolerance, point_y + tolerance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_inspector_new() {
        let inspector = CircleInspector::new(0.01, 10);
        assert_eq!(CircleInspector::DIMENSION, 2);
        assert!(inspector.circles().is_empty());
        assert!(inspector.shot_circles().is_empty());
    }

    #[test]
    fn test_circle_inspector_bind() {
        let mut inspector = CircleInspector::new(0.01, 10);
        let circle = Circle::with_location_and_radius(1.0, 2.0, 3.0);
        inspector.bind(0, circle);

        assert_eq!(inspector.circles().len(), 1);
        let c = inspector.circle(0).unwrap();
        assert_eq!(c.radius(), 3.0);
    }

    #[test]
    fn test_circle_inspector_set_point() {
        let mut inspector = CircleInspector::new(0.01, 10);
        inspector.set_point(0.5, 0.7);
        assert!(inspector.shot_circles().is_empty());
    }

    #[test]
    fn test_circle_inspector_inspect() {
        let mut inspector = CircleInspector::new(0.1, 10);
        let circle = Circle::with_location_and_radius(0.0, 0.0, 1.0);
        inspector.bind(0, circle);
        inspector.set_point(0.0, 0.0);

        let result = inspector.inspect(0);
        assert!(result);
    }

    #[test]
    fn test_circle_inspector_coord() {
        assert_eq!(CircleInspector::coord(0, 1.5, 2.5), 1.5);
        assert_eq!(CircleInspector::coord(1, 1.5, 2.5), 2.5);
    }

    #[test]
    fn test_circle_inspector_shift() {
        let (x, y) = CircleInspector::shift(1.0, 2.0, 0.5);
        assert_eq!(x, 1.5);
        assert_eq!(y, 2.5);
    }

    #[test]
    fn test_circle_inspector_is_equal() {
        assert!(CircleInspector::is_equal(5, 5));
        assert!(!CircleInspector::is_equal(5, 6));
    }
}
