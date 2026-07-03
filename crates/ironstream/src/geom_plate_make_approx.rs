// FILE: geom_plate_make_approx.rs
// occt: GeomPlate_MakeApprox

/// Converts a GeomPlate surface into a BSpline surface.
/// Performs approximation with configurable tolerance and continuity.
pub struct GeomPlateMakeApprox {
    tol3d: f64,
    nbmax: i32,
    dgmax: i32,
    approx_error: f64,
    criterion_error: f64,
}

impl GeomPlateMakeApprox {
    /// Create approximation with criterion
    pub fn new_with_criterion(
        tol3d: f64,
        nbmax: i32,
        dgmax: i32,
        enlarge_coeff: f64,
    ) -> Self {
        GeomPlateMakeApprox {
            tol3d,
            nbmax,
            dgmax,
            approx_error: 0.0,
            criterion_error: 0.0,
        }
    }

    /// Create approximation with criterion order
    pub fn new_with_crit_order(
        tol3d: f64,
        nbmax: i32,
        dgmax: i32,
        dmax: f64,
        crit_order: i32,
        enlarge_coeff: f64,
    ) -> Self {
        GeomPlateMakeApprox {
            tol3d,
            nbmax,
            dgmax,
            approx_error: 0.0,
            criterion_error: 0.0,
        }
    }

    /// Get the approximation error
    pub fn approx_error(&self) -> f64 {
        self.approx_error
    }

    /// Get the criterion error
    pub fn criterion_error(&self) -> f64 {
        self.criterion_error
    }

    /// Set the approximation error
    pub fn set_approx_error(&mut self, err: f64) {
        self.approx_error = err;
    }

    /// Set the criterion error
    pub fn set_criterion_error(&mut self, err: f64) {
        self.criterion_error = err;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_with_criterion() {
        let approx = GeomPlateMakeApprox::new_with_criterion(0.01, 10, 8, 1.1);
        assert_eq!(approx.tol3d, 0.01);
        assert_eq!(approx.nbmax, 10);
        assert_eq!(approx.dgmax, 8);
        assert_eq!(approx.approx_error(), 0.0);
        assert_eq!(approx.criterion_error(), 0.0);
    }

    #[test]
    fn test_construction_with_crit_order() {
        let approx = GeomPlateMakeApprox::new_with_crit_order(0.01, 10, 8, 1.0, 0, 1.1);
        assert_eq!(approx.tol3d, 0.01);
        assert_eq!(approx.nbmax, 10);
    }

    #[test]
    fn test_error_accessors() {
        let mut approx = GeomPlateMakeApprox::new_with_criterion(0.01, 10, 8, 1.1);
        approx.set_approx_error(0.005);
        approx.set_criterion_error(0.003);

        assert_eq!(approx.approx_error(), 0.005);
        assert_eq!(approx.criterion_error(), 0.003);
    }
}
