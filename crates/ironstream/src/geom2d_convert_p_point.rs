// FILE: geom2d_convert_p_point.rs
// occt: Geom2dConvert_PPoint

/// Represents a point on a 2D curve with parameter, coordinates, and tangent derivative.
#[derive(Clone, Debug)]
pub struct PPoint {
    parameter: f64,
    point: [f64; 2],
    d1: [f64; 2],
}

impl PPoint {
    /// Creates an empty point at parameter infinity.
    pub fn new() -> Self {
        PPoint {
            parameter: f64::INFINITY,
            point: [0.0, 0.0],
            d1: [0.0, 0.0],
        }
    }

    /// Creates a point with given parameter, coordinates, and derivatives.
    pub fn new_with_values(parameter: f64, point: [f64; 2], d1: [f64; 2]) -> Self {
        PPoint { parameter, point, d1 }
    }

    /// Computes distance to another point.
    pub fn dist(&self, other: &PPoint) -> f64 {
        let dx = self.point[0] - other.point[0];
        let dy = self.point[1] - other.point[1];
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the parameter value.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// Returns the point coordinates.
    pub fn point(&self) -> [f64; 2] {
        self.point
    }

    /// Returns the first derivatives (tangent).
    pub fn d1(&self) -> [f64; 2] {
        self.d1
    }

    /// Sets the derivative values.
    pub fn set_d1(&mut self, d1: [f64; 2]) {
        self.d1 = d1;
    }
}

impl PartialEq for PPoint {
    fn eq(&self, other: &Self) -> bool {
        (self.parameter - other.parameter).abs() < 1e-10
            && (self.point[0] - other.point[0]).abs() < 1e-10
            && (self.point[1] - other.point[1]).abs() < 1e-10
    }
}

impl Default for PPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppoint_new() {
        let pt = PPoint::new();
        assert_eq!(pt.parameter(), f64::INFINITY);
        assert_eq!(pt.point(), [0.0, 0.0]);
        assert_eq!(pt.d1(), [0.0, 0.0]);
    }

    #[test]
    fn test_ppoint_new_with_values() {
        let pt = PPoint::new_with_values(1.5, [2.0, 3.0], [1.0, 0.0]);
        assert_eq!(pt.parameter(), 1.5);
        assert_eq!(pt.point(), [2.0, 3.0]);
        assert_eq!(pt.d1(), [1.0, 0.0]);
    }

    #[test]
    fn test_ppoint_dist() {
        let pt1 = PPoint::new_with_values(0.0, [0.0, 0.0], [0.0, 0.0]);
        let pt2 = PPoint::new_with_values(1.0, [3.0, 4.0], [0.0, 0.0]);
        let dist = pt1.dist(&pt2);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_ppoint_set_d1() {
        let mut pt = PPoint::new_with_values(0.0, [1.0, 2.0], [0.0, 0.0]);
        pt.set_d1([2.0, 1.0]);
        assert_eq!(pt.d1(), [2.0, 1.0]);
    }

    #[test]
    fn test_ppoint_equality() {
        let pt1 = PPoint::new_with_values(1.0, [1.0, 1.0], [0.5, 0.5]);
        let pt2 = PPoint::new_with_values(1.0, [1.0, 1.0], [0.5, 0.5]);
        assert_eq!(pt1, pt2);
    }

    #[test]
    fn test_ppoint_default() {
        let pt = PPoint::default();
        assert_eq!(pt.parameter(), f64::INFINITY);
    }
}
