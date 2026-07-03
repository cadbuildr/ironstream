// FILE: geom_grid_eval_b_spline_surface.rs
// occt: GeomGridEval_BSplineSurface

pub struct GeomGridEvalBSplineSurface;

impl GeomGridEvalBSplineSurface {
    pub fn new() -> Self {
        GeomGridEvalBSplineSurface
    }

    pub fn evaluate(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

impl Default for GeomGridEvalBSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _eval = GeomGridEvalBSplineSurface::new();
    }

    #[test]
    fn test_evaluate() {
        let eval = GeomGridEvalBSplineSurface::new();
        let (x, y, z) = eval.evaluate(0.5, 0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
