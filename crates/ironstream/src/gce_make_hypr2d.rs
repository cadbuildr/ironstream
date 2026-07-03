// FILE: gce_make_hypr2d.rs
// occt: gce_MakeHypr2d

/// Make a 2D hyperbola.
pub struct GceMakeHypr2d {
    center: (f64, f64),
    major_radius: f64,
    minor_radius: f64,
    status: i32,
}

impl GceMakeHypr2d {
    /// Create a hyperbola.
    pub fn new(
        center: (f64, f64),
        major_radius: f64,
        minor_radius: f64,
    ) -> Self {
        let status = if major_radius < 1e-10 || minor_radius < 1e-10 {
            7 // NullRadius
        } else {
            0 // Done
        };

        GceMakeHypr2d {
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
    fn test_make_hyperbola_valid() {
        let hypr = GceMakeHypr2d::new((0.0, 0.0), 5.0, 3.0);
        assert!(hypr.is_done());
        assert_eq!(hypr.center(), (0.0, 0.0));
        assert_eq!(hypr.major_radius(), 5.0);
    }

    #[test]
    fn test_make_hyperbola_zero_radius() {
        let hypr = GceMakeHypr2d::new((0.0, 0.0), 0.0, 3.0);
        assert!(!hypr.is_done());
        assert_eq!(hypr.status(), 7);
    }
}
