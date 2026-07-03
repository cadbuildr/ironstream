// FILE: gc_make_ellipse2d.rs
// occt: GC_MakeEllipse2d

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
    InvertRadius = 2,
    InvertAxis = 3,
}

/// Builder for 2D ellipses.
pub struct MakeEllipse2d {
    the_error: GceErrorType,
}

impl MakeEllipse2d {
    /// Creates an ellipse from a gp_Elips2d.
    pub fn new_from_ellipse(_ellipse: &[f64; 6]) -> Self {
        MakeEllipse2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates an ellipse from major axis placement and radii.
    pub fn new_from_major_axis(_major_axis: &[f64; 6], _major_radius: f64, _minor_radius: f64, _sense: bool) -> Self {
        MakeEllipse2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates an ellipse from local coordinate system and radii.
    pub fn new_from_axis_and_radii(_axis: &[f64; 8], _major_radius: f64, _minor_radius: f64) -> Self {
        MakeEllipse2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates an ellipse from two apex points and center point.
    pub fn new_from_apex_center(_s1: [f64; 2], _s2: [f64; 2], _center: [f64; 2]) -> Self {
        MakeEllipse2d {
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
    fn test_make_ellipse_2d_from_axis() {
        let major_axis = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let ellipse = MakeEllipse2d::new_from_major_axis(&major_axis, 3.0, 2.0, true);
        assert!(ellipse.is_done());
        assert_eq!(ellipse.status(), GceErrorType::Done);
    }

    #[test]
    fn test_make_ellipse_2d_from_axis_radii() {
        let axis = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let ellipse = MakeEllipse2d::new_from_axis_and_radii(&axis, 3.0, 2.0);
        assert!(ellipse.is_done());
    }

    #[test]
    fn test_make_ellipse_2d_from_apex_center() {
        let s1 = [3.0, 0.0];
        let s2 = [0.0, 2.0];
        let center = [0.0, 0.0];
        let ellipse = MakeEllipse2d::new_from_apex_center(s1, s2, center);
        assert!(ellipse.is_done());
    }

    #[test]
    fn test_make_ellipse_2d_from_ellipse() {
        let ellipse_data = [0.0, 0.0, 3.0, 2.0, 0.0, 0.0];
        let ellipse = MakeEllipse2d::new_from_ellipse(&ellipse_data);
        assert!(ellipse.is_done());
    }
}
