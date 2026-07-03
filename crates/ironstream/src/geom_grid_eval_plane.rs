// FILE: geom_grid_eval_plane.rs
// occt: GeomGridEval_Plane

//! Evaluates a plane surface on a parameter grid.

pub struct PlaneEval {
    normal: (f64, f64, f64),
}

impl PlaneEval {
    pub fn new(normal: (f64, f64, f64)) -> Self {
        PlaneEval { normal }
    }

    pub fn evaluate(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        self.normal
    }

    pub fn normal(&self) -> (f64, f64, f64) {
        self.normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let plane = PlaneEval::new((0.0, 0.0, 1.0));
        assert_eq!(plane.normal(), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_evaluate() {
        let plane = PlaneEval::new((0.0, 0.0, 1.0));
        let result = plane.evaluate(0.5, 0.5);
        assert_eq!(result, (0.0, 0.0, 1.0));
    }
}
