// FILE: geom_grid_eval_cone.rs
// occt: GeomGridEval_Cone

pub struct GeomGridEvalCone {
    semi_angle: f64,
}

impl GeomGridEvalCone {
    pub fn new(semi_angle: f64) -> Self {
        GeomGridEvalCone { semi_angle }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        (
            v * u.cos() * self.semi_angle.tan(),
            v * u.sin() * self.semi_angle.tan(),
            v,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_cone_new() {
        let cone = GeomGridEvalCone::new(0.5);
        assert!((cone.semi_angle - 0.5).abs() < PRECISION);
    }

    #[test]
    fn test_cone_evaluate() {
        let cone = GeomGridEvalCone::new(0.0);
        let (x, y, z) = cone.evaluate(0.0, 1.0);
        assert!(x.abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!((z - 1.0).abs() < PRECISION);
    }
}
