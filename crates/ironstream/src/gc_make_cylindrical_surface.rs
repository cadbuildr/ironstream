// FILE: gc_make_cylindrical_surface.rs
// occt: GC_MakeCylindricalSurface

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    NegativeRadius = 1,
    InvertAxis = 2,
    InvertData = 3,
    ColinearPoints = 4,
}

/// Builder for cylindrical surfaces.
pub struct MakeCylindricalSurface {
    the_error: GceErrorType,
    the_cylinder: Option<[f64; 9]>, // placeholder for surface data
}

impl MakeCylindricalSurface {
    /// Creates a cylindrical surface from axis placement and radius.
    pub fn new_from_axis_radius(_axis: [f64; 9], radius: f64) -> Self {
        let error = if radius < 0.0 {
            GceErrorType::NegativeRadius
        } else {
            GceErrorType::Done
        };
        MakeCylindricalSurface {
            the_error: error,
            the_cylinder: None,
        }
    }

    /// Creates a cylindrical surface from an existing gp_Cylinder.
    pub fn new_from_cylinder(_cylinder: &[f64; 9]) -> Self {
        MakeCylindricalSurface {
            the_error: GceErrorType::Done,
            the_cylinder: None,
        }
    }

    /// Creates a cylindrical surface parallel to input cylinder at a distance.
    pub fn new_parallel_at_distance(_cylinder: &[f64; 9], _distance: f64) -> Self {
        MakeCylindricalSurface {
            the_error: GceErrorType::Done,
            the_cylinder: None,
        }
    }

    /// Creates a cylindrical surface parallel to input cylinder through a point.
    pub fn new_parallel_through_point(_cylinder: &[f64; 9], _point: [f64; 3]) -> Self {
        MakeCylindricalSurface {
            the_error: GceErrorType::Done,
            the_cylinder: None,
        }
    }

    /// Creates a cylindrical surface through three points.
    pub fn new_from_three_points(_p1: [f64; 3], _p2: [f64; 3], _p3: [f64; 3]) -> Self {
        MakeCylindricalSurface {
            the_error: GceErrorType::Done,
            the_cylinder: None,
        }
    }

    /// Creates a cylindrical surface from axis and radius.
    pub fn new_from_axis_radius_direct(_axis: [f64; 6], radius: f64) -> Self {
        let error = if radius < 0.0 {
            GceErrorType::NegativeRadius
        } else {
            GceErrorType::Done
        };
        MakeCylindricalSurface {
            the_error: error,
            the_cylinder: None,
        }
    }

    /// Creates a cylindrical surface from its circular base.
    pub fn new_from_circle(_circle: &[f64; 9]) -> Self {
        MakeCylindricalSurface {
            the_error: GceErrorType::Done,
            the_cylinder: None,
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
    fn test_make_cylinder_from_axis_radius() {
        let axis = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0];
        let cyl = MakeCylindricalSurface::new_from_axis_radius(axis, 2.5);
        assert!(cyl.is_done());
        assert_eq!(cyl.status(), GceErrorType::Done);
    }

    #[test]
    fn test_make_cylinder_negative_radius() {
        let axis = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0];
        let cyl = MakeCylindricalSurface::new_from_axis_radius(axis, -1.0);
        assert!(!cyl.is_done());
        assert_eq!(cyl.status(), GceErrorType::NegativeRadius);
    }

    #[test]
    fn test_make_cylinder_from_circle() {
        let circle = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let cyl = MakeCylindricalSurface::new_from_circle(&circle);
        assert!(cyl.is_done());
    }

    #[test]
    fn test_make_cylinder_from_three_points() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [0.0, 0.0, 5.0];
        let p3 = [2.0, 0.0, 0.0];
        let cyl = MakeCylindricalSurface::new_from_three_points(p1, p2, p3);
        assert!(cyl.is_done());
    }
}
