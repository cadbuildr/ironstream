// FILE: int_res2d_domain.rs
// occt: IntRes2d_Domain

/// Represents a domain on a 2D curve with optional first and last bounds.
#[derive(Clone, Debug, Copy)]
pub struct Domain {
    status: i32,
    first_param: f64,
    last_param: f64,
    first_tol: f64,
    last_tol: f64,
    first_point_x: f64,
    first_point_y: f64,
    last_point_x: f64,
    last_point_y: f64,
    periodfirst: f64,
    periodlast: f64,
}

impl Domain {
    /// Create an infinite domain (no first or last point)
    pub fn new() -> Self {
        Domain {
            status: 0, // No bounds
            first_param: 0.0,
            last_param: 0.0,
            first_tol: 0.0,
            last_tol: 0.0,
            first_point_x: 0.0,
            first_point_y: 0.0,
            last_point_x: 0.0,
            last_point_y: 0.0,
            periodfirst: 0.0,
            periodlast: 0.0,
        }
    }

    /// Create a bounded domain with two extremities
    pub fn with_bounds(
        pnt1_x: f64,
        pnt1_y: f64,
        par1: f64,
        tol1: f64,
        pnt2_x: f64,
        pnt2_y: f64,
        par2: f64,
        tol2: f64,
    ) -> Self {
        Domain {
            status: 3, // Both bounds
            first_param: par1,
            last_param: par2,
            first_tol: tol1,
            last_tol: tol2,
            first_point_x: pnt1_x,
            first_point_y: pnt1_y,
            last_point_x: pnt2_x,
            last_point_y: pnt2_y,
            periodfirst: 0.0,
            periodlast: 0.0,
        }
    }

    /// Create a semi-infinite domain
    pub fn semi_infinite(pnt_x: f64, pnt_y: f64, par: f64, tol: f64, is_first: bool) -> Self {
        let status = if is_first { 1 } else { 2 };
        let (fp_x, fp_y, lp_x, lp_y) = if is_first {
            (pnt_x, pnt_y, 0.0, 0.0)
        } else {
            (0.0, 0.0, pnt_x, pnt_y)
        };
        let (first_p, last_p, first_t, last_t) = if is_first {
            (par, 0.0, tol, 0.0)
        } else {
            (0.0, par, 0.0, tol)
        };

        Domain {
            status,
            first_param: first_p,
            last_param: last_p,
            first_tol: first_t,
            last_tol: last_t,
            first_point_x: fp_x,
            first_point_y: fp_y,
            last_point_x: lp_x,
            last_point_y: lp_y,
            periodfirst: 0.0,
            periodlast: 0.0,
        }
    }

    /// Set domain to infinite
    pub fn set_infinite(&mut self) {
        self.status = 0;
    }

    /// Set domain bounds
    pub fn set_bounds(
        &mut self,
        pnt1_x: f64,
        pnt1_y: f64,
        par1: f64,
        tol1: f64,
        pnt2_x: f64,
        pnt2_y: f64,
        par2: f64,
        tol2: f64,
    ) {
        self.status = 3;
        self.first_param = par1;
        self.last_param = par2;
        self.first_tol = tol1;
        self.last_tol = tol2;
        self.first_point_x = pnt1_x;
        self.first_point_y = pnt1_y;
        self.last_point_x = pnt2_x;
        self.last_point_y = pnt2_y;
    }

    /// Set equivalent parameters for closed domain
    pub fn set_equivalent_parameters(&mut self, zero: f64, period: f64) {
        self.periodfirst = zero;
        self.periodlast = zero + period;
    }

    /// Check if domain has a first point
    pub fn has_first_point(&self) -> bool {
        (self.status & 1) != 0
    }

    /// Check if domain has a last point
    pub fn has_last_point(&self) -> bool {
        (self.status & 2) != 0
    }

    /// Get first parameter
    pub fn first_parameter(&self) -> Option<f64> {
        if self.has_first_point() {
            Some(self.first_param)
        } else {
            None
        }
    }

    /// Get last parameter
    pub fn last_parameter(&self) -> Option<f64> {
        if self.has_last_point() {
            Some(self.last_param)
        } else {
            None
        }
    }

    /// Get first point (x, y)
    pub fn first_point(&self) -> Option<(f64, f64)> {
        if self.has_first_point() {
            Some((self.first_point_x, self.first_point_y))
        } else {
            None
        }
    }

    /// Get last point (x, y)
    pub fn last_point(&self) -> Option<(f64, f64)> {
        if self.has_last_point() {
            Some((self.last_point_x, self.last_point_y))
        } else {
            None
        }
    }

    /// Get first tolerance
    pub fn first_tolerance(&self) -> Option<f64> {
        if self.has_first_point() {
            Some(self.first_tol)
        } else {
            None
        }
    }

    /// Get last tolerance
    pub fn last_tolerance(&self) -> Option<f64> {
        if self.has_last_point() {
            Some(self.last_tol)
        } else {
            None
        }
    }

    /// Check if domain is closed
    pub fn is_closed(&self) -> bool {
        (self.periodfirst - self.periodlast).abs() > 1e-10
    }
}

impl Default for Domain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite() {
        let d = Domain::new();
        assert!(!d.has_first_point());
        assert!(!d.has_last_point());
    }

    #[test]
    fn test_bounded() {
        let d = Domain::with_bounds(0.0, 0.0, 0.5, 0.1, 1.0, 1.0, 1.5, 0.1);
        assert!(d.has_first_point());
        assert!(d.has_last_point());
        assert_eq!(d.first_parameter(), Some(0.5));
        assert_eq!(d.last_parameter(), Some(1.5));
        assert_eq!(d.first_tolerance(), Some(0.1));
        assert_eq!(d.last_tolerance(), Some(0.1));
    }

    #[test]
    fn test_semi_infinite_first() {
        let d = Domain::semi_infinite(0.5, 0.5, 0.5, 0.1, true);
        assert!(d.has_first_point());
        assert!(!d.has_last_point());
    }

    #[test]
    fn test_semi_infinite_last() {
        let d = Domain::semi_infinite(1.5, 1.5, 1.5, 0.1, false);
        assert!(!d.has_first_point());
        assert!(d.has_last_point());
    }

    #[test]
    fn test_set_infinite() {
        let mut d = Domain::with_bounds(0.0, 0.0, 0.5, 0.1, 1.0, 1.0, 1.5, 0.1);
        d.set_infinite();
        assert!(!d.has_first_point());
        assert!(!d.has_last_point());
    }
}
