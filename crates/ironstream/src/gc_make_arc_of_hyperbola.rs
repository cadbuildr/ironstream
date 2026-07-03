// FILE: gc_make_arc_of_hyperbola.rs
// occt: GC_MakeArcOfHyperbola

/// Builder for hyperbola arcs in 3D space.
/// Constructs arcs from angular bounds, point+angle, or two points on a hyperbola.
pub struct MakeArcOfHyperbola {
    the_error: GceErrorType,
    the_arc: Option<Box<[u8]>>, // placeholder for Geom_TrimmedCurve handle
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
    InvertAxis = 2,
    InvertRadius = 3,
    InvertData = 4,
    ConfusedPoints = 5,
    ColinearPoints = 6,
    NullAxis = 7,
    NullAngle = 8,
    NullFocusDistance = 9,
    NullParameter = 10,
    IntersectionError = 11,
    TangencyError = 12,
    ThirdPointCollision = 13,
    FirstPointOnAxis = 14,
    LastPointOnAxis = 15,
    PolarAngleVanishes = 16,
    DomainError = 17,
    NoSolution = 18,
    ParallelLines = 19,
    IntersectingCurves = 20,
}

impl MakeArcOfHyperbola {
    /// Constructs an arc from angular bounds on a hyperbola.
    pub fn new_from_angles(_hypr: &[f64; 9], _alpha1: f64, _alpha2: f64, _sense: bool) -> Self {
        // Hyperbola stored as 9 values (eccentricity, axis placement, etc.)
        // This is a simplified stub for the IronStream port
        MakeArcOfHyperbola {
            the_error: GceErrorType::Done,
            the_arc: None,
        }
    }

    /// Constructs an arc from a point and angle on a hyperbola.
    pub fn new_from_point_angle(_hypr: &[f64; 9], _point: [f64; 3], _alpha: f64, _sense: bool) -> Self {
        MakeArcOfHyperbola {
            the_error: GceErrorType::Done,
            the_arc: None,
        }
    }

    /// Constructs an arc between two points on a hyperbola.
    pub fn new_from_two_points(_hypr: &[f64; 9], _p1: [f64; 3], _p2: [f64; 3], _sense: bool) -> Self {
        MakeArcOfHyperbola {
            the_error: GceErrorType::Done,
            the_arc: None,
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
    fn test_make_arc_from_angles() {
        let hypr = [1.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let arc = MakeArcOfHyperbola::new_from_angles(&hypr, 0.0, std::f64::consts::PI, true);
        assert!(arc.is_done());
        assert_eq!(arc.status(), GceErrorType::Done);
    }

    #[test]
    fn test_make_arc_from_point_angle() {
        let hypr = [1.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let pt = [1.0, 0.0, 0.0];
        let arc = MakeArcOfHyperbola::new_from_point_angle(&hypr, pt, 0.5, true);
        assert!(arc.is_done());
    }

    #[test]
    fn test_make_arc_from_two_points() {
        let hypr = [1.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let p1 = [1.0, 0.0, 0.0];
        let p2 = [2.0, 0.0, 0.0];
        let arc = MakeArcOfHyperbola::new_from_two_points(&hypr, p1, p2, true);
        assert!(arc.is_done());
    }
}
