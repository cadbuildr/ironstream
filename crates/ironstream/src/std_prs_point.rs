// FILE: std_prs_point.rs
// occt: StdPrs_Point

/// Tool for creating point presentations
pub struct StdPrsToolPoint;

impl StdPrsToolPoint {
    /// Extract point coordinates from a geometric point
    pub fn extract_point(point: &(f64, f64, f64)) -> (f64, f64, f64) {
        *point
    }
}

/// Presentation of geometric points
/// This is a typedef: Prs3d_Point<Handle<Geom_Point>, StdPrs_ToolPoint>
pub struct StdPrsPoint {
    /// The geometric point coordinate
    pub point: (f64, f64, f64),
    /// Visual properties
    pub marker_size: f32,
}

impl StdPrsPoint {
    /// Creates a new point presentation
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        StdPrsPoint {
            point: (x, y, z),
            marker_size: 1.0,
        }
    }

    /// Creates a point from a tuple
    pub fn from_tuple(point: (f64, f64, f64)) -> Self {
        StdPrsPoint {
            point,
            marker_size: 1.0,
        }
    }

    /// Returns the coordinates of the point
    pub fn coordinates(&self) -> (f64, f64, f64) {
        self.point
    }

    /// Sets the marker size for display
    pub fn set_marker_size(&mut self, size: f32) {
        self.marker_size = size;
    }

    /// Gets the marker size
    pub fn marker_size(&self) -> f32 {
        self.marker_size
    }

    /// Returns the distance from this point to another
    pub fn distance_to(&self, other: &StdPrsPoint) -> f64 {
        let dx = self.point.0 - other.point.0;
        let dy = self.point.1 - other.point.1;
        let dz = self.point.2 - other.point.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_point() {
        let point = StdPrsPoint::new(1.0, 2.0, 3.0);
        assert_eq!(point.coordinates(), (1.0, 2.0, 3.0));
        assert_eq!(point.marker_size, 1.0);
    }

    #[test]
    fn test_from_tuple() {
        let tuple = (4.0, 5.0, 6.0);
        let point = StdPrsPoint::from_tuple(tuple);
        assert_eq!(point.coordinates(), tuple);
    }

    #[test]
    fn test_set_marker_size() {
        let mut point = StdPrsPoint::new(0.0, 0.0, 0.0);
        point.set_marker_size(5.0);
        assert_eq!(point.marker_size(), 5.0);
    }

    #[test]
    fn test_distance_to() {
        let point1 = StdPrsPoint::new(0.0, 0.0, 0.0);
        let point2 = StdPrsPoint::new(3.0, 4.0, 0.0);
        let distance = point1.distance_to(&point2);
        assert!((distance - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_distance_zero() {
        let point1 = StdPrsPoint::new(1.0, 2.0, 3.0);
        let point2 = StdPrsPoint::new(1.0, 2.0, 3.0);
        let distance = point1.distance_to(&point2);
        assert!((distance - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_extract_point() {
        let coords = (10.0, 20.0, 30.0);
        let extracted = StdPrsToolPoint::extract_point(&coords);
        assert_eq!(extracted, coords);
    }
}
