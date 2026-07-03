// FILE: top_ope_b_rep_ds_point.rs
// occt: TopOpeBRepDS_Point

/// A 3D point with tolerance
#[derive(Debug, Clone)]
pub struct Point {
    point: [f64; 3],     // x, y, z
    tolerance: f64,
    keep: bool,
}

impl Point {
    /// Create a new Point with default values
    pub fn new() -> Self {
        Point {
            point: [0.0, 0.0, 0.0],
            tolerance: 0.0,
            keep: false,
        }
    }

    /// Create a Point from coordinates and tolerance
    pub fn from_coords(x: f64, y: f64, z: f64, tolerance: f64) -> Self {
        Point {
            point: [x, y, z],
            tolerance,
            keep: false,
        }
    }

    /// Create a Point from array and tolerance
    pub fn from_array(coords: [f64; 3], tolerance: f64) -> Self {
        Point {
            point: coords,
            tolerance,
            keep: false,
        }
    }

    /// Check equality with another Point (considering tolerance)
    pub fn is_equal(&self, other: &Point) -> bool {
        let dx = self.point[0] - other.point[0];
        let dy = self.point[1] - other.point[1];
        let dz = self.point[2] - other.point[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;

        // Tolerance is compared as maximum tolerance
        let max_tol = self.tolerance.max(other.tolerance);
        dist_sq <= max_tol * max_tol
    }

    /// Get point coordinates as array
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Get X coordinate
    pub fn x(&self) -> f64 {
        self.point[0]
    }

    /// Get Y coordinate
    pub fn y(&self) -> f64 {
        self.point[1]
    }

    /// Get Z coordinate
    pub fn z(&self) -> f64 {
        self.point[2]
    }

    /// Mutably access the point
    pub fn point_mut(&mut self) -> &mut [f64; 3] {
        &mut self.point
    }

    /// Get tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set tolerance
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Check if point should be kept
    pub fn keep(&self) -> bool {
        self.keep
    }

    /// Set keep flag
    pub fn set_keep(&mut self, b: bool) {
        self.keep = b;
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point::new();
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.0);
        assert_eq!(p.z(), 0.0);
        assert_eq!(p.tolerance(), 0.0);
        assert!(!p.keep());
    }

    #[test]
    fn test_point_from_coords() {
        let p = Point::from_coords(1.0, 2.0, 3.0, 0.5);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
        assert_eq!(p.tolerance(), 0.5);
    }

    #[test]
    fn test_point_from_array() {
        let p = Point::from_array([1.0, 2.0, 3.0], 0.25);
        assert_eq!(p.point(), [1.0, 2.0, 3.0]);
        assert_eq!(p.tolerance(), 0.25);
    }

    #[test]
    fn test_point_is_equal() {
        let p1 = Point::from_coords(0.0, 0.0, 0.0, 1.0);
        let p2 = Point::from_coords(0.5, 0.5, 0.0, 1.0);
        assert!(p1.is_equal(&p2));
    }

    #[test]
    fn test_point_is_equal_false() {
        let p1 = Point::from_coords(0.0, 0.0, 0.0, 0.1);
        let p2 = Point::from_coords(1.0, 1.0, 1.0, 0.1);
        assert!(!p1.is_equal(&p2));
    }

    #[test]
    fn test_point_tolerance_operations() {
        let mut p = Point::new();
        p.set_tolerance(0.75);
        assert_eq!(p.tolerance(), 0.75);
    }

    #[test]
    fn test_point_keep_flag() {
        let mut p = Point::new();
        assert!(!p.keep());
        p.set_keep(true);
        assert!(p.keep());
        p.set_keep(false);
        assert!(!p.keep());
    }

    #[test]
    fn test_point_mut_access() {
        let mut p = Point::new();
        {
            let coords = p.point_mut();
            coords[0] = 5.0;
            coords[1] = 6.0;
            coords[2] = 7.0;
        }
        assert_eq!(p.x(), 5.0);
        assert_eq!(p.y(), 6.0);
        assert_eq!(p.z(), 7.0);
    }

    #[test]
    fn test_point_default() {
        let p = Point::default();
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.tolerance(), 0.0);
    }
}
