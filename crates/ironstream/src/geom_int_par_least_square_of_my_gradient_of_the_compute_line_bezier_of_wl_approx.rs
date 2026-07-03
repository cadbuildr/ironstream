// FILE: geom_int_par_least_square_of_my_gradient_of_the_compute_line_bezier_of_wl_approx.rs
// occt: GeomInt_ParLeastSquareOfMyGradientOfTheComputeLineBezierOfWLApprox

pub struct GeomIntParLeastSquareBezier {
    error: f64,
}

impl GeomIntParLeastSquareBezier {
    pub fn new() -> Self {
        GeomIntParLeastSquareBezier { error: 0.0 }
    }

    pub fn error(&self) -> f64 {
        self.error
    }

    pub fn set_error(&mut self, e: f64) {
        self.error = e;
    }
}

impl Default for GeomIntParLeastSquareBezier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let ls = GeomIntParLeastSquareBezier::new();
        assert_eq!(ls.error(), 0.0);
    }

    #[test]
    fn test_set_error() {
        let mut ls = GeomIntParLeastSquareBezier::new();
        ls.set_error(0.001);
        assert_eq!(ls.error(), 0.001);
    }
}
