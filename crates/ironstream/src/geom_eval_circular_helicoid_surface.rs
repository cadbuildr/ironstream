// FILE: geom_eval_circular_helicoid_surface.rs
// occt: GeomEval_CircularHelicoidSurface

use std::f64::consts::PI;

pub struct GeomEvalCircularHelicoidSurface {
    radius: f64,
    pitch: f64,
}

impl GeomEvalCircularHelicoidSurface {
    pub fn new(radius: f64, pitch: f64) -> Self {
        GeomEvalCircularHelicoidSurface { radius, pitch }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        (
            v * u.cos(),
            v * u.sin(),
            self.pitch * u / (2.0 * PI),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_helicoid_new() {
        let helicoid = GeomEvalCircularHelicoidSurface::new(1.0, 1.0);
        assert_eq!(helicoid.radius, 1.0);
        assert_eq!(helicoid.pitch, 1.0);
    }

    #[test]
    fn test_helicoid_evaluate() {
        let helicoid = GeomEvalCircularHelicoidSurface::new(1.0, 1.0);
        let (x, y, z) = helicoid.evaluate(0.0, 1.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
