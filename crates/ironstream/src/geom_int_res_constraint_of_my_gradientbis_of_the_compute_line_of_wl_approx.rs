// FILE: geom_int_res_constraint_of_my_gradientbis_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_ResConstraintOfMyGradientbisOfTheComputeLineOfWLApprox

pub struct GeomIntResConstraintGradientbis {
    residual: f64,
}

impl GeomIntResConstraintGradientbis {
    pub fn new() -> Self {
        GeomIntResConstraintGradientbis { residual: 0.0 }
    }

    pub fn residual(&self) -> f64 {
        self.residual
    }

    pub fn set_residual(&mut self, r: f64) {
        self.residual = r;
    }
}

impl Default for GeomIntResConstraintGradientbis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let res = GeomIntResConstraintGradientbis::new();
        assert_eq!(res.residual(), 0.0);
    }

    #[test]
    fn test_set_residual() {
        let mut res = GeomIntResConstraintGradientbis::new();
        res.set_residual(0.005);
        assert_eq!(res.residual(), 0.005);
    }
}
