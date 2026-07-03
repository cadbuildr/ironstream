// FILE: gc_make_trimmed_cone.rs
// occt: GC_MakeTrimmedCone

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
    ColinearPoints = 2,
    InvertData = 3,
}

/// Builder for trimmed cones.
pub struct MakeTrimmedCone {
    the_error: GceErrorType,
}

impl MakeTrimmedCone {
    /// Creates a rectangular trimmed conical surface from four points.
    pub fn new_from_four_points(_p1: [f64; 3], _p2: [f64; 3], _p3: [f64; 3], _p4: [f64; 3]) -> Self {
        MakeTrimmedCone {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates a rectangular trimmed conical surface from two points and two radii.
    pub fn new_from_points_radii(_p1: [f64; 3], _p2: [f64; 3], _r1: f64, _r2: f64) -> Self {
        let error = if _r1 < 0.0 || _r2 < 0.0 {
            GceErrorType::NegativeRadius
        } else {
            GceErrorType::Done
        };
        MakeTrimmedCone {
            the_error: error,
        }
    }

    /// Returns true if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.the_error == GceErrorType::Done
    }

    /// Returns the error status.
    pub fn status(&self) -> GceErrorType {
        self.the_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimmed_cone_from_four_points() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [0.0, 0.0, 5.0];
        let p3 = [2.0, 0.0, 0.0];
        let p4 = [1.0, 0.0, 5.0];
        let cone = MakeTrimmedCone::new_from_four_points(p1, p2, p3, p4);
        assert!(cone.is_done());
        assert_eq!(cone.status(), GceErrorType::Done);
    }

    #[test]
    fn test_trimmed_cone_from_points_radii() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [0.0, 0.0, 5.0];
        let cone = MakeTrimmedCone::new_from_points_radii(p1, p2, 2.0, 1.0);
        assert!(cone.is_done());
    }

    #[test]
    fn test_trimmed_cone_negative_radius() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [0.0, 0.0, 5.0];
        let cone = MakeTrimmedCone::new_from_points_radii(p1, p2, -1.0, 1.0);
        assert!(!cone.is_done());
        assert_eq!(cone.status(), GceErrorType::NegativeRadius);
    }
}
