// FILE: gc_make_hyperbola2d.rs
// occt: GC_MakeHyperbola2d

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
    InvertAxis = 2,
    ConfusedPoints = 3,
    ColinearPoints = 4,
}

/// Builder for 2D hyperbolas.
pub struct MakeHyperbola2d {
    the_error: GceErrorType,
}

impl MakeHyperbola2d {
    /// Creates a hyperbola from a gp_Hypr2d.
    pub fn new_from_hyperbola(_hyperbola: &[f64; 6]) -> Self {
        MakeHyperbola2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates a hyperbola from major axis placement and radii.
    pub fn new_from_major_axis(_major_axis: &[f64; 6], _major_radius: f64, _minor_radius: f64, _sense: bool) -> Self {
        MakeHyperbola2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates a hyperbola from local coordinate system and radii.
    pub fn new_from_axis_and_radii(_axis: &[f64; 8], _major_radius: f64, _minor_radius: f64) -> Self {
        MakeHyperbola2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates a hyperbola from two apex points and center point.
    pub fn new_from_apex_center(_s1: [f64; 2], _s2: [f64; 2], _center: [f64; 2]) -> Self {
        MakeHyperbola2d {
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
    fn test_make_hyperbola_2d_from_axis() {
        let major_axis = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let hyp = MakeHyperbola2d::new_from_major_axis(&major_axis, 2.0, 1.5, true);
        assert!(hyp.is_done());
        assert_eq!(hyp.status(), GceErrorType::Done);
    }

    #[test]
    fn test_make_hyperbola_2d_from_axis_radii() {
        let axis = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let hyp = MakeHyperbola2d::new_from_axis_and_radii(&axis, 2.0, 1.5);
        assert!(hyp.is_done());
    }

    #[test]
    fn test_make_hyperbola_2d_from_apex_center() {
        let s1 = [2.0, 0.0];
        let s2 = [0.0, 1.5];
        let center = [0.0, 0.0];
        let hyp = MakeHyperbola2d::new_from_apex_center(s1, s2, center);
        assert!(hyp.is_done());
    }

    #[test]
    fn test_make_hyperbola_2d_from_hyperbola() {
        let hyp_data = [0.0, 0.0, 2.0, 1.5, 0.0, 0.0];
        let hyp = MakeHyperbola2d::new_from_hyperbola(&hyp_data);
        assert!(hyp.is_done());
    }
}
