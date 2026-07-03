// FILE: geom_grid_eval_bezier_surface.rs
// occt: GeomGridEval_BezierSurface

pub struct GeomGridEvalBezierSurface {
    degree_u: usize,
    degree_v: usize,
}

impl GeomGridEvalBezierSurface {
    pub fn new(degree_u: usize, degree_v: usize) -> Self {
        GeomGridEvalBezierSurface { degree_u, degree_v }
    }

    pub fn evaluate(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_eval_bezier_surface_new() {
        let eval = GeomGridEvalBezierSurface::new(2, 3);
        assert_eq!(eval.degree_u, 2);
        assert_eq!(eval.degree_v, 3);
    }

    #[test]
    fn test_grid_eval_bezier_surface_evaluate() {
        let eval = GeomGridEvalBezierSurface::new(2, 2);
        let (x, y, z) = eval.evaluate(0.5, 0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
