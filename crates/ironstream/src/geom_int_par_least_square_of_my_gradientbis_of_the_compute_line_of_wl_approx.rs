// FILE: geom_int_par_least_square_of_my_gradientbis_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_ParLeastSquareOfMyGradientbisOfTheComputeLineOfWLApprox

pub struct GeomIntParLeastSquareGradientbis {
    error: f64,
}

impl GeomIntParLeastSquareGradientbis {
    pub fn new() -> Self {
        GeomIntParLeastSquareGradientbis { error: 0.0 }
    }

    pub fn error(&self) -> f64 {
        self.error
    }

    pub fn set_error(&mut self, e: f64) {
        self.error = e;
    }
}

impl Default for GeomIntParLeastSquareGradientbis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let ls = GeomIntParLeastSquareGradientbis::new();
        assert_eq!(ls.error(), 0.0);
    }

    #[test]
    fn test_set_error() {
        let mut ls = GeomIntParLeastSquareGradientbis::new();
        ls.set_error(0.0005);
        assert_eq!(ls.error(), 0.0005);
    }
}
