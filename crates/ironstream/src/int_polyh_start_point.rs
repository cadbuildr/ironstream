// FILE: int_polyh_start_point.rs
// occt: IntPolyh_StartPoint

//! Starting point for polyhedron intersection search.

/// Start point for intersection tracing
#[derive(Clone)]
pub struct IntPolyhStartPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u1: f64,
    pub v1: f64,
    pub u2: f64,
    pub v2: f64,
    pub tri1: i32,
    pub tri2: i32,
}

impl IntPolyhStartPoint {
    /// Creates new start point
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        u1: f64,
        v1: f64,
        u2: f64,
        v2: f64,
        tri1: i32,
        tri2: i32,
    ) -> Self {
        IntPolyhStartPoint {
            x,
            y,
            z,
            u1,
            v1,
            u2,
            v2,
            tri1,
            tri2,
        }
    }

    /// Returns 3D point coordinates
    pub fn point(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Returns first surface parameters
    pub fn params1(&self) -> (f64, f64) {
        (self.u1, self.v1)
    }

    /// Returns second surface parameters
    pub fn params2(&self) -> (f64, f64) {
        (self.u2, self.v2)
    }

    /// Returns triangle indices
    pub fn triangles(&self) -> (i32, i32) {
        (self.tri1, self.tri2)
    }
}

impl Default for IntPolyhStartPoint {
    fn default() -> Self {
        IntPolyhStartPoint {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            u1: 0.0,
            v1: 0.0,
            u2: 0.0,
            v2: 0.0,
            tri1: -1,
            tri2: -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_point_new() {
        let sp = IntPolyhStartPoint::new(1.0, 2.0, 3.0, 0.5, 0.5, 0.5, 0.5, 0, 1);
        let (x, y, z) = sp.point();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_start_point_params() {
        let sp = IntPolyhStartPoint::new(0.0, 0.0, 0.0, 0.3, 0.7, 0.2, 0.8, 0, 1);
        let (u1, v1) = sp.params1();
        let (u2, v2) = sp.params2();
        assert_eq!(u1, 0.3);
        assert_eq!(v1, 0.7);
        assert_eq!(u2, 0.2);
        assert_eq!(v2, 0.8);
    }

    #[test]
    fn test_start_point_triangles() {
        let sp = IntPolyhStartPoint::new(0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 5, 10);
        let (tri1, tri2) = sp.triangles();
        assert_eq!(tri1, 5);
        assert_eq!(tri2, 10);
    }
}
