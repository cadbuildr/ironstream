// FILE: geom2d_eval_circle_involute_curve.rs
// occt: Geom2dEval_CircleInvoluteCurve

/// Involute of a circle
#[derive(Debug, Clone)]
pub struct Geom2dEvalCircleInvoluteCurve {
    radius: f64,
}

impl Geom2dEvalCircleInvoluteCurve {
    pub fn new(radius: f64) -> Self {
        Geom2dEvalCircleInvoluteCurve { radius }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Evaluate involute at parameter t
    pub fn value(&self, t: f64) -> (f64, f64) {
        let r = self.radius;
        (
            r * (t.cos() + t * t.sin()),
            r * (t.sin() - t * t.cos()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_involute_new() {
        let involute = Geom2dEvalCircleInvoluteCurve::new(1.0);
        assert_eq!(involute.radius(), 1.0);
    }

    #[test]
    fn test_involute_value() {
        let involute = Geom2dEvalCircleInvoluteCurve::new(1.0);
        let (x, y) = involute.value(0.0);
        assert!((x - 1.0).abs() < 0.0001);
        assert!(y.abs() < 0.0001);
    }
}
