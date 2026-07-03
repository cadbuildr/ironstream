// FILE: geom_grid_eval_ellipse.rs
// occt: GeomGridEval_Ellipse

pub struct GeomGridEvalEllipse {
    major: f64,
    minor: f64,
}

impl GeomGridEvalEllipse {
    pub fn new(major: f64, minor: f64) -> Self {
        GeomGridEvalEllipse {
            major: major.abs(),
            minor: minor.abs(),
        }
    }

    pub fn evaluate(&self, u: f64) -> (f64, f64, f64) {
        (
            self.major * u.cos(),
            self.minor * u.sin(),
            0.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_ellipse_new() {
        let ellipse = GeomGridEvalEllipse::new(2.0, 1.0);
        assert!((ellipse.major - 2.0).abs() < PRECISION);
        assert!((ellipse.minor - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_ellipse_evaluate() {
        let ellipse = GeomGridEvalEllipse::new(2.0, 1.0);
        let (x, y, z) = ellipse.evaluate(0.0);
        assert!((x - 2.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
