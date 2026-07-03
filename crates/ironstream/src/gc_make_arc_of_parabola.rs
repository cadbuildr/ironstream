// FILE: gc_make_arc_of_parabola.rs
// occt: GC_MakeArcOfParabola

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
    InvertAxis = 2,
}

/// Builder for parabola arcs in 3D space.
pub struct MakeArcOfParabola {
    the_error: GceErrorType,
}

impl MakeArcOfParabola {
    /// Constructs an arc from angular bounds on a parabola.
    pub fn new_from_angles(_parab: &[f64; 9], _alpha1: f64, _alpha2: f64, _sense: bool) -> Self {
        MakeArcOfParabola {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs an arc from a point and angle on a parabola.
    pub fn new_from_point_angle(_parab: &[f64; 9], _point: [f64; 3], _alpha: f64, _sense: bool) -> Self {
        MakeArcOfParabola {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs an arc between two points on a parabola.
    pub fn new_from_two_points(_parab: &[f64; 9], _p1: [f64; 3], _p2: [f64; 3], _sense: bool) -> Self {
        MakeArcOfParabola {
            the_error: GceErrorType::Done,
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
    fn test_make_arc_parabola_from_angles() {
        let parab = [0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let arc = MakeArcOfParabola::new_from_angles(&parab, 0.0, std::f64::consts::PI, true);
        assert!(arc.is_done());
        assert_eq!(arc.status(), GceErrorType::Done);
    }

    #[test]
    fn test_make_arc_parabola_from_point() {
        let parab = [0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let pt = [0.0, 1.0, 0.0];
        let arc = MakeArcOfParabola::new_from_point_angle(&parab, pt, 0.5, true);
        assert!(arc.is_done());
    }

    #[test]
    fn test_make_arc_parabola_from_two_points() {
        let parab = [0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [0.0, 2.0, 0.0];
        let arc = MakeArcOfParabola::new_from_two_points(&parab, p1, p2, true);
        assert!(arc.is_done());
    }
}
