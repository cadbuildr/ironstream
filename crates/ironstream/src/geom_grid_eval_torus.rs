// FILE: geom_grid_eval_torus.rs
// occt: GeomGridEval_Torus

pub struct GeomGridEvalTorus {
    major_radius: f64,
    minor_radius: f64,
}

impl GeomGridEvalTorus {
    pub fn new(major_radius: f64, minor_radius: f64) -> Self {
        GeomGridEvalTorus {
            major_radius: major_radius.abs(),
            minor_radius: minor_radius.abs(),
        }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let cos_u = u.cos();
        let sin_u = u.sin();
        let cos_v = v.cos();
        let sin_v = v.sin();
        (
            (self.major_radius + self.minor_radius * cos_v) * cos_u,
            (self.major_radius + self.minor_radius * cos_v) * sin_u,
            self.minor_radius * sin_v,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_torus_new() {
        let torus = GeomGridEvalTorus::new(3.0, 1.0);
        assert!((torus.major_radius - 3.0).abs() < PRECISION);
        assert!((torus.minor_radius - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_torus_evaluate() {
        let torus = GeomGridEvalTorus::new(2.0, 1.0);
        let (x, y, z) = torus.evaluate(0.0, 0.0);
        assert!((x - 3.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
