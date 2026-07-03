// FILE: geom_eval_hyp_paraboloid_surface.rs
// occt: GeomEval_HypParaboloidSurface

pub struct GeomEvalHypParaboloidSurface {
    a: f64,
    b: f64,
}

impl GeomEvalHypParaboloidSurface {
    pub fn new(a: f64, b: f64) -> Self {
        GeomEvalHypParaboloidSurface { a, b }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        (
            u,
            v,
            (u * u) / (self.a * self.a) - (v * v) / (self.b * self.b),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_hyp_paraboloid_new() {
        let surface = GeomEvalHypParaboloidSurface::new(2.0, 1.0);
        assert!((surface.a - 2.0).abs() < PRECISION);
        assert!((surface.b - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_hyp_paraboloid_evaluate() {
        let surface = GeomEvalHypParaboloidSurface::new(1.0, 1.0);
        let (x, y, z) = surface.evaluate(1.0, 1.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 1.0).abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
