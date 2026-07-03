// FILE: geom_grid_eval_parabola.rs
// occt: GeomGridEval_Parabola

pub struct GeomGridEvalParabola {
    focal_length: f64,
}

impl GeomGridEvalParabola {
    pub fn new(focal_length: f64) -> Self {
        GeomGridEvalParabola {
            focal_length: focal_length.abs(),
        }
    }

    pub fn evaluate(&self, u: f64) -> (f64, f64, f64) {
        (
            u * u / (4.0 * self.focal_length),
            u,
            0.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_parabola_new() {
        let parab = GeomGridEvalParabola::new(1.0);
        assert!((parab.focal_length - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_parabola_evaluate() {
        let parab = GeomGridEvalParabola::new(1.0);
        let (x, y, z) = parab.evaluate(2.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 2.0).abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
