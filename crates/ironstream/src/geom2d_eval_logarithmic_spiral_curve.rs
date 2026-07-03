// FILE: geom2d_eval_logarithmic_spiral_curve.rs
// occt: Geom2dEval_LogarithmicSpiralCurve

/// Logarithmic spiral curve r = a * e^(b*theta)
#[derive(Debug, Clone)]
pub struct Geom2dEvalLogarithmicSpiralCurve {
    a: f64, // Scaling factor
    b: f64, // Growth rate
}

impl Geom2dEvalLogarithmicSpiralCurve {
    pub fn new(a: f64, b: f64) -> Self {
        Geom2dEvalLogarithmicSpiralCurve { a, b }
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn value(&self, theta: f64) -> (f64, f64) {
        let r = self.a * (self.b * theta).exp();
        (r * theta.cos(), r * theta.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_spiral_new() {
        let spiral = Geom2dEvalLogarithmicSpiralCurve::new(1.0, 0.1);
        assert_eq!(spiral.a(), 1.0);
        assert_eq!(spiral.b(), 0.1);
    }

    #[test]
    fn test_log_spiral_value() {
        let spiral = Geom2dEvalLogarithmicSpiralCurve::new(1.0, 0.0);
        let (x, y) = spiral.value(0.0);
        assert!((x - 1.0).abs() < 0.0001);
        assert!(y.abs() < 0.0001);
    }
}
