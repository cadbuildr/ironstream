// FILE: top_ope_b_rep_p_the_point_of_intersection.rs
// occt: TopOpeBRep_PThePointOfIntersection

/// Pointer type alias for ThePointOfIntersection (IntPatch_Point).
pub type PThePointOfIntersection = Option<Box<IntPatchPoint>>;

/// Intersection point structure from IntPatch.
pub struct IntPatchPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl IntPatchPoint {
    /// Create a new intersection point.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        IntPatchPoint { x, y, z }
    }

    /// Get x coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get z coordinate.
    pub fn z(&self) -> f64 {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pt = IntPatchPoint::new(1.0, 2.0, 3.0);
        assert_eq!(pt.x(), 1.0);
        assert_eq!(pt.y(), 2.0);
        assert_eq!(pt.z(), 3.0);
    }

    #[test]
    fn test_pointer_type() {
        let p: PThePointOfIntersection = Some(Box::new(IntPatchPoint::new(1.0, 2.0, 3.0)));
        assert!(p.is_some());
        if let Some(pt) = p {
            assert_eq!(pt.x(), 1.0);
        }
    }

    #[test]
    fn test_null_pointer() {
        let p: PThePointOfIntersection = None;
        assert!(p.is_none());
    }
}
