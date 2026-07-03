// FILE: gce_make_elips2d.rs
// occt: gce_MakeElips2d

/// Make a 2D ellipse.
pub struct GceMakeElips2d {
    center: (f64, f64),
    major_radius: f64,
    minor_radius: f64,
    status: i32,
}

impl GceMakeElips2d {
    /// Create an ellipse.
    pub fn new(
        center: (f64, f64),
        major_radius: f64,
        minor_radius: f64,
    ) -> Self {
        let status = if major_radius < minor_radius || major_radius < 1e-10 || minor_radius < 1e-10 {
            2 // NegativeRadius
        } else {
            0 // Done
        };

        GceMakeElips2d {
            center,
            major_radius,
            minor_radius,
            status,
        }
    }

    /// Get the status.
    pub fn status(&self) -> i32 {
        self.status
    }

    /// Check if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.status == 0
    }

    /// Get the center.
    pub fn center(&self) -> (f64, f64) {
        self.center
    }

    /// Get the major radius.
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Get the minor radius.
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_ellipse_valid() {
        let ellipse = GceMakeElips2d::new((0.0, 0.0), 5.0, 3.0);
        assert!(ellipse.is_done());
        assert_eq!(ellipse.center(), (0.0, 0.0));
        assert_eq!(ellipse.major_radius(), 5.0);
        assert_eq!(ellipse.minor_radius(), 3.0);
    }

    #[test]
    fn test_make_ellipse_negative_radius() {
        let ellipse = GceMakeElips2d::new((0.0, 0.0), 3.0, 5.0);
        assert!(!ellipse.is_done());
        assert_eq!(ellipse.status(), 2);
    }

    #[test]
    fn test_make_ellipse_zero_radius() {
        let ellipse = GceMakeElips2d::new((0.0, 0.0), 0.0, 3.0);
        assert!(!ellipse.is_done());
    }
}
