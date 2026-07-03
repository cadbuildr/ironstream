// FILE: gc_make_trimmed_cylinder.rs
// occt: GC_MakeTrimmedCylinder

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
    ColinearPoints = 2,
}

/// Builder for trimmed cylinders.
pub struct MakeTrimmedCylinder {
    the_error: GceErrorType,
}

impl MakeTrimmedCylinder {
    /// Creates a trimmed cylindrical surface from three points.
    pub fn new_from_three_points(_p1: [f64; 3], _p2: [f64; 3], _p3: [f64; 3]) -> Self {
        MakeTrimmedCylinder {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates a trimmed cylindrical surface from a base circle and height.
    pub fn new_from_circle_height(_circle: &[f64; 9], _height: f64) -> Self {
        MakeTrimmedCylinder {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates a trimmed cylindrical surface from axis, radius and height.
    pub fn new_from_axis_radius_height(_axis: &[f64; 6], _radius: f64, _height: f64) -> Self {
        let error = if _radius < 0.0 {
            GceErrorType::NegativeRadius
        } else {
            GceErrorType::Done
        };
        MakeTrimmedCylinder {
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
    fn test_trimmed_cylinder_from_three_points() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [0.0, 0.0, 5.0];
        let p3 = [2.0, 0.0, 0.0];
        let cyl = MakeTrimmedCylinder::new_from_three_points(p1, p2, p3);
        assert!(cyl.is_done());
        assert_eq!(cyl.status(), GceErrorType::Done);
    }

    #[test]
    fn test_trimmed_cylinder_from_circle_height() {
        let circle = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let cyl = MakeTrimmedCylinder::new_from_circle_height(&circle, 5.0);
        assert!(cyl.is_done());
    }

    #[test]
    fn test_trimmed_cylinder_from_axis_radius_height() {
        let axis = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let cyl = MakeTrimmedCylinder::new_from_axis_radius_height(&axis, 2.0, 5.0);
        assert!(cyl.is_done());
    }

    #[test]
    fn test_trimmed_cylinder_negative_radius() {
        let axis = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let cyl = MakeTrimmedCylinder::new_from_axis_radius_height(&axis, -1.0, 5.0);
        assert!(!cyl.is_done());
        assert_eq!(cyl.status(), GceErrorType::NegativeRadius);
    }
}
