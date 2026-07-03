// FILE: geom_int_b_sp_par_least_square_of_my_b_spl_gradient_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_BSpParLeastSquareOfMyBSplGradientOfTheComputeLineOfWLApprox

pub struct GeomIntBSpParLeastSquare {
    error: f64,
}

impl GeomIntBSpParLeastSquare {
    pub fn new() -> Self {
        GeomIntBSpParLeastSquare { error: 0.0 }
    }

    pub fn error(&self) -> f64 {
        self.error
    }
}

impl Default for GeomIntBSpParLeastSquare {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let ls = GeomIntBSpParLeastSquare::new();
        assert_eq!(ls.error(), 0.0);
    }
}
