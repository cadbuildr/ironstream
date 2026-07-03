// FILE: geom_int_wl_approx.rs
// occt: GeomInt_WLApprox

/// Wire-line approximation for surface intersection.
pub struct GeomIntWLApprox {
    tolerance: f64,
    max_degree: i32,
}

impl GeomIntWLApprox {
    pub fn new() -> Self {
        GeomIntWLApprox {
            tolerance: 1e-6,
            max_degree: 8,
        }
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn max_degree(&self) -> i32 {
        self.max_degree
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    pub fn set_max_degree(&mut self, deg: i32) {
        self.max_degree = deg;
    }
}

impl Default for GeomIntWLApprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let approx = GeomIntWLApprox::new();
        assert!((approx.tolerance() - 1e-6).abs() < 1e-10);
        assert_eq!(approx.max_degree(), 8);
    }

    #[test]
    fn test_setters() {
        let mut approx = GeomIntWLApprox::new();
        approx.set_tolerance(1e-5);
        approx.set_max_degree(10);

        assert!((approx.tolerance() - 1e-5).abs() < 1e-10);
        assert_eq!(approx.max_degree(), 10);
    }
}
