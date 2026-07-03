// FILE: geom_eval_circular_helix_curve.rs
// occt: GeomEval_CircularHelixCurve

use std::f64::consts::PI;

pub struct GeomEvalCircularHelixCurve {
    radius: f64,
    pitch: f64,
}

impl GeomEvalCircularHelixCurve {
    pub fn new(radius: f64, pitch: f64) -> Self {
        GeomEvalCircularHelixCurve { radius, pitch }
    }

    pub fn evaluate(&self, t: f64) -> (f64, f64, f64) {
        (
            self.radius * t.cos(),
            self.radius * t.sin(),
            self.pitch * t / (2.0 * PI),
        )
    }

    pub fn evaluate_d1(&self, t: f64) -> ((f64, f64, f64), (f64, f64, f64)) {
        (
            (
                self.radius * t.cos(),
                self.radius * t.sin(),
                self.pitch * t / (2.0 * PI),
            ),
            (
                -self.radius * t.sin(),
                self.radius * t.cos(),
                self.pitch / (2.0 * PI),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_helix_new() {
        let helix = GeomEvalCircularHelixCurve::new(1.0, 1.0);
        assert_eq!(helix.radius, 1.0);
        assert_eq!(helix.pitch, 1.0);
    }

    #[test]
    fn test_helix_evaluate_at_0() {
        let helix = GeomEvalCircularHelixCurve::new(1.0, 1.0);
        let (x, y, z) = helix.evaluate(0.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }

    #[test]
    fn test_helix_evaluate_d1() {
        let helix = GeomEvalCircularHelixCurve::new(1.0, 1.0);
        let ((_x, _y, _z), (dx, dy, dz)) = helix.evaluate_d1(0.0);
        assert!(dx.abs() < PRECISION);
        assert!((dy - 1.0).abs() < PRECISION);
        assert!((dz - 1.0 / (2.0 * PI)).abs() < PRECISION);
    }
}
