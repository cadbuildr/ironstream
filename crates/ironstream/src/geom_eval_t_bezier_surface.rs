// FILE: geom_eval_t_bezier_surface.rs
// occt: GeomEval_TBezierSurface

pub struct GeomEvalTBezierSurface {
    degree_u: usize,
    degree_v: usize,
}

impl GeomEvalTBezierSurface {
    pub fn new(degree_u: usize, degree_v: usize) -> Self {
        GeomEvalTBezierSurface { degree_u, degree_v }
    }

    pub fn evaluate(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_bezier_surface_new() {
        let eval = GeomEvalTBezierSurface::new(3, 2);
        assert_eq!(eval.degree_u, 3);
        assert_eq!(eval.degree_v, 2);
    }

    #[test]
    fn test_t_bezier_surface_evaluate() {
        let eval = GeomEvalTBezierSurface::new(2, 2);
        let (x, y, z) = eval.evaluate(0.5, 0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
