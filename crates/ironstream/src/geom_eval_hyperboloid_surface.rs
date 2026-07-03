// FILE: geom_eval_hyperboloid_surface.rs
// occt: GeomEval_HyperboloidSurface

pub struct GeomEvalHyperboloidSurface {
    a: f64,
    b: f64,
    c: f64,
}

impl GeomEvalHyperboloidSurface {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        GeomEvalHyperboloidSurface { a, b, c }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let cos_v = v.cos();
        let sin_v = v.sin();
        let cosh_u = u.cosh();
        (
            self.a * cosh_u * cos_v,
            self.b * cosh_u * sin_v,
            self.c * u.sinh(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_hyperboloid_new() {
        let surface = GeomEvalHyperboloidSurface::new(1.0, 1.0, 1.0);
        assert!((surface.a - 1.0).abs() < PRECISION);
        assert!((surface.b - 1.0).abs() < PRECISION);
        assert!((surface.c - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_hyperboloid_evaluate() {
        let surface = GeomEvalHyperboloidSurface::new(1.0, 1.0, 1.0);
        let (x, y, z) = surface.evaluate(0.0, 0.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
