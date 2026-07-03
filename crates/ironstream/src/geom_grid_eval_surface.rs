// FILE: geom_grid_eval_surface.rs
// occt: GeomGridEval_Surface

pub struct GeomGridEvalSurface;

impl GeomGridEvalSurface {
    pub fn new() -> Self {
        GeomGridEvalSurface
    }

    pub fn evaluate(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

impl Default for GeomGridEvalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _eval = GeomGridEvalSurface::new();
    }

    #[test]
    fn test_evaluate() {
        let eval = GeomGridEvalSurface::new();
        let (x, y, z) = eval.evaluate(0.5, 0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
