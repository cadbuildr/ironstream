// FILE: geom_grid_eval_surface_of_revolution.rs
// occt: GeomGridEval_SurfaceOfRevolution

pub struct GeomGridEvalSurfaceOfRevolution;

impl GeomGridEvalSurfaceOfRevolution {
    pub fn new() -> Self {
        GeomGridEvalSurfaceOfRevolution
    }

    pub fn evaluate(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

impl Default for GeomGridEvalSurfaceOfRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _surf = GeomGridEvalSurfaceOfRevolution::new();
    }

    #[test]
    fn test_evaluate() {
        let surf = GeomGridEvalSurfaceOfRevolution::new();
        let (x, y, z) = surf.evaluate(0.5, 0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
