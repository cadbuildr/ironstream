// FILE: geom_int_b_sp_gradient_bfgs_of_my_b_spl_gradient_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_BSpGradient_BFGSOfMyBSplGradientOfTheComputeLineOfWLApprox

/// BFGS optimization for B-spline gradient computation in WLApprox
pub struct GeomIntBSpGradientBFGSOfMyBSplGradient {
    iter_count: i32,
    tolerance: f64,
}

impl GeomIntBSpGradientBFGSOfMyBSplGradient {
    pub fn new() -> Self {
        GeomIntBSpGradientBFGSOfMyBSplGradient {
            iter_count: 0,
            tolerance: 1e-7,
        }
    }

    pub fn iter_count(&self) -> i32 {
        self.iter_count
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

impl Default for GeomIntBSpGradientBFGSOfMyBSplGradient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let opt = GeomIntBSpGradientBFGSOfMyBSplGradient::new();
        assert_eq!(opt.iter_count(), 0);
        assert!((opt.tolerance() - 1e-7).abs() < 1e-10);
    }
}
