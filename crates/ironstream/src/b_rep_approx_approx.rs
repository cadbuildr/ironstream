// FILE: b_rep_approx_approx.rs
// occt: BRepApprox_Approx

/// BRepApprox_Approx: Approximation framework
pub struct BRepApproxApprox {
    tolerance: f64,
    max_degree: i32,
    max_segments: i32,
}

impl BRepApproxApprox {
    pub fn new() -> Self {
        BRepApproxApprox {
            tolerance: 1e-7,
            max_degree: 8,
            max_segments: 9,
        }
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    pub fn set_max_degree(&mut self, deg: i32) {
        self.max_degree = deg;
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn max_degree(&self) -> i32 {
        self.max_degree
    }
}

impl Default for BRepApproxApprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let app = BRepApproxApprox::new();
        assert!(app.tolerance() > 0.0);
        assert!(app.max_degree() > 0);
    }

    #[test]
    fn test_set_tolerance() {
        let mut app = BRepApproxApprox::new();
        app.set_tolerance(1e-5);
        assert_eq!(app.tolerance(), 1e-5);
    }

    #[test]
    fn test_set_max_degree() {
        let mut app = BRepApproxApprox::new();
        app.set_max_degree(10);
        assert_eq!(app.max_degree(), 10);
    }
}
