// FILE: geom_int_res_constraint_of_my_gradient_of_the_compute_line_bezier_of_wl_approx.rs
// occt: GeomInt_ResConstraintOfMyGradientOfTheComputeLineBezierOfWLApprox

pub struct GeomIntResConstraintBezier {
    residual: f64,
}

impl GeomIntResConstraintBezier {
    pub fn new() -> Self {
        GeomIntResConstraintBezier { residual: 0.0 }
    }

    pub fn residual(&self) -> f64 {
        self.residual
    }

    pub fn set_residual(&mut self, r: f64) {
        self.residual = r;
    }
}

impl Default for GeomIntResConstraintBezier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let res = GeomIntResConstraintBezier::new();
        assert_eq!(res.residual(), 0.0);
    }

    #[test]
    fn test_set_residual() {
        let mut res = GeomIntResConstraintBezier::new();
        res.set_residual(0.01);
        assert_eq!(res.residual(), 0.01);
    }
}
