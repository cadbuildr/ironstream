// FILE: geom_eval_paraboloid_surface.rs
// occt: GeomEval_ParaboloidSurface

pub struct GeomEvalParaboloidSurface {
    a: f64,
    b: f64,
}

impl GeomEvalParaboloidSurface {
    pub fn new(a: f64, b: f64) -> Self {
        GeomEvalParaboloidSurface { a, b }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        (
            u,
            v,
            (u * u) / (self.a * self.a) + (v * v) / (self.b * self.b),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_paraboloid_new() {
        let surface = GeomEvalParaboloidSurface::new(2.0, 1.0);
        assert!((surface.a - 2.0).abs() < PRECISION);
        assert!((surface.b - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_paraboloid_evaluate() {
        let surface = GeomEvalParaboloidSurface::new(1.0, 1.0);
        let (x, y, z) = surface.evaluate(1.0, 1.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 1.0).abs() < PRECISION);
        assert!((z - 2.0).abs() < PRECISION);
    }
}
