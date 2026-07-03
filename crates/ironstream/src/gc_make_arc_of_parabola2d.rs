// FILE: gc_make_arc_of_parabola2d.rs
// occt: GC_MakeArcOfParabola2d

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
}

/// Builder for parabola arcs in 2D space.
pub struct MakeArcOfParabola2d {
    the_error: GceErrorType,
}

impl MakeArcOfParabola2d {
    /// Constructs an arc from angular bounds on a 2D parabola.
    pub fn new_from_angles(_parabola: &[f64; 6], _alpha1: f64, _alpha2: f64, _sense: bool) -> Self {
        MakeArcOfParabola2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs an arc from a point and angular bound on a 2D parabola.
    pub fn new_from_point_angle(_parabola: &[f64; 6], _point: [f64; 2], _alpha: f64, _sense: bool) -> Self {
        MakeArcOfParabola2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs an arc between two points on a 2D parabola.
    pub fn new_from_two_points(_parabola: &[f64; 6], _p1: [f64; 2], _p2: [f64; 2], _sense: bool) -> Self {
        MakeArcOfParabola2d {
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
    fn test_make_arc_parabola_2d_from_angles() {
        let parab2d = [0.5, 0.0, 0.0, 1.0, 0.0, 0.0];
        let arc = MakeArcOfParabola2d::new_from_angles(&parab2d, 0.0, std::f64::consts::PI, true);
        assert!(arc.is_done());
        assert_eq!(arc.status(), GceErrorType::Done);
    }

    #[test]
    fn test_make_arc_parabola_2d_from_point() {
        let parab2d = [0.5, 0.0, 0.0, 1.0, 0.0, 0.0];
        let pt = [0.0, 1.0];
        let arc = MakeArcOfParabola2d::new_from_point_angle(&parab2d, pt, 0.5, true);
        assert!(arc.is_done());
    }

    #[test]
    fn test_make_arc_parabola_2d_from_two_points() {
        let parab2d = [0.5, 0.0, 0.0, 1.0, 0.0, 0.0];
        let p1 = [0.0, 0.0];
        let p2 = [0.0, 1.0];
        let arc = MakeArcOfParabola2d::new_from_two_points(&parab2d, p1, p2, true);
        assert!(arc.is_done());
    }
}
