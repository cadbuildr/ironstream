// FILE: b_rep_mesh_circle.rs
// occt: BRepMesh_Circle

/// Describes a 2d circle with compact storage (3 doubles).
/// Stores circle center location and radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    location_x: f64,
    location_y: f64,
    radius: f64,
}

impl Circle {
    /// Default constructor.
    pub fn new() -> Self {
        Circle {
            location_x: 0.0,
            location_y: 0.0,
            radius: 0.0,
        }
    }

    /// Constructor.
    /// theLocation: location of a circle (x, y)
    /// theRadius: radius of a circle
    pub fn with_location_and_radius(location_x: f64, location_y: f64, radius: f64) -> Self {
        Circle {
            location_x,
            location_y,
            radius,
        }
    }

    /// Sets location of a circle.
    pub fn set_location(&mut self, location_x: f64, location_y: f64) {
        self.location_x = location_x;
        self.location_y = location_y;
    }

    /// Sets radius of a circle.
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Returns X coordinate of circle location.
    pub fn location_x(&self) -> f64 {
        self.location_x
    }

    /// Returns Y coordinate of circle location.
    pub fn location_y(&self) -> f64 {
        self.location_y
    }

    /// Returns radius of a circle.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_default() {
        let c = Circle::new();
        assert_eq!(c.location_x(), 0.0);
        assert_eq!(c.location_y(), 0.0);
        assert_eq!(c.radius(), 0.0);
    }

    #[test]
    fn test_circle_with_location_and_radius() {
        let c = Circle::with_location_and_radius(1.0, 2.0, 3.0);
        assert_eq!(c.location_x(), 1.0);
        assert_eq!(c.location_y(), 2.0);
        assert_eq!(c.radius(), 3.0);
    }

    #[test]
    fn test_circle_set_location() {
        let mut c = Circle::new();
        c.set_location(5.0, 6.0);
        assert_eq!(c.location_x(), 5.0);
        assert_eq!(c.location_y(), 6.0);
    }

    #[test]
    fn test_circle_set_radius() {
        let mut c = Circle::new();
        c.set_radius(7.5);
        assert_eq!(c.radius(), 7.5);
    }

    #[test]
    fn test_circle_equality() {
        let c1 = Circle::with_location_and_radius(1.0, 2.0, 3.0);
        let c2 = Circle::with_location_and_radius(1.0, 2.0, 3.0);
        assert_eq!(c1, c2);
    }
}
