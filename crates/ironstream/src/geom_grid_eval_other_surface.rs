// FILE: geom_grid_eval_other_surface.rs
// occt: GeomGridEval_OtherSurface

//! Evaluates other types of surfaces on a parameter grid.

pub struct OtherSurfaceEval;

impl OtherSurfaceEval {
    pub fn evaluate(_surface_type: &str, _u: f64, _v: f64) -> Option<(f64, f64, f64)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let result = OtherSurfaceEval::evaluate("unknown", 0.5, 0.5);
        assert_eq!(result, None);
    }
}
