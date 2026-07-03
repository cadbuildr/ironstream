// FILE: top_ope_b_rep_p_int_res2d_intersection_point.rs
// occt: TopOpeBRep_PIntRes2d_IntersectionPoint

/// Pointer type alias for IntRes2d_IntersectionPoint.
pub type PIntRes2dIntersectionPoint = Option<Box<IntRes2dIntersectionPoint>>;

/// 2D intersection point structure.
pub struct IntRes2dIntersectionPoint {
    x: f64,
    y: f64,
}

impl IntRes2dIntersectionPoint {
    /// Create a new intersection point.
    pub fn new(x: f64, y: f64) -> Self {
        IntRes2dIntersectionPoint { x, y }
    }

    /// Get x coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pt = IntRes2dIntersectionPoint::new(1.0, 2.0);
        assert_eq!(pt.x(), 1.0);
        assert_eq!(pt.y(), 2.0);
    }

    #[test]
    fn test_pointer_type() {
        let p: PIntRes2dIntersectionPoint = Some(Box::new(IntRes2dIntersectionPoint::new(1.0, 2.0)));
        assert!(p.is_some());
        if let Some(pt) = p {
            assert_eq!(pt.x(), 1.0);
        }
    }

    #[test]
    fn test_null_pointer() {
        let p: PIntRes2dIntersectionPoint = None;
        assert!(p.is_none());
    }
}
