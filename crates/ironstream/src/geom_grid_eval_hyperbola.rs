// FILE: geom_grid_eval_hyperbola.rs
// occt: GeomGridEval_Hyperbola

pub struct GeomGridEvalHyperbola {
    major: f64,
    minor: f64,
}

impl GeomGridEvalHyperbola {
    pub fn new(major: f64, minor: f64) -> Self {
        GeomGridEvalHyperbola {
            major: major.abs(),
            minor: minor.abs(),
        }
    }

    pub fn evaluate(&self, u: f64) -> (f64, f64, f64) {
        (
            self.major * u.cosh(),
            self.minor * u.sinh(),
            0.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_hyperbola_new() {
        let hyp = GeomGridEvalHyperbola::new(2.0, 1.0);
        assert!((hyp.major - 2.0).abs() < PRECISION);
        assert!((hyp.minor - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_hyperbola_evaluate() {
        let hyp = GeomGridEvalHyperbola::new(1.0, 1.0);
        let (x, y, z) = hyp.evaluate(0.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
