// FILE: geom2d_eval_archimedean_spiral_curve.rs
// occt: Geom2dEval_ArchimedeanSpiralCurve

/// Archimedean spiral curve r = a + b*theta
#[derive(Debug, Clone)]
pub struct Geom2dEvalArchimedeanSpiralCurve {
    a: f64, // Start radius
    b: f64, // Pitch
}

impl Geom2dEvalArchimedeanSpiralCurve {
    pub fn new(a: f64, b: f64) -> Self {
        Geom2dEvalArchimedeanSpiralCurve { a, b }
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn value(&self, theta: f64) -> (f64, f64) {
        let r = self.a + self.b * theta;
        (r * theta.cos(), r * theta.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_archimedean_spiral_new() {
        let spiral = Geom2dEvalArchimedeanSpiralCurve::new(1.0, 0.5);
        assert_eq!(spiral.a(), 1.0);
        assert_eq!(spiral.b(), 0.5);
    }

    #[test]
    fn test_archimedean_spiral_value() {
        let spiral = Geom2dEvalArchimedeanSpiralCurve::new(1.0, 0.1);
        let (x, y) = spiral.value(0.0);
        assert!((x - 1.0).abs() < 0.0001);
        assert!(y.abs() < 0.0001);
    }
}
