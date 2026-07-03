// FILE: geom_grid_eval_cylinder.rs
// occt: GeomGridEval_Cylinder

pub struct GeomGridEvalCylinder {
    radius: f64,
}

impl GeomGridEvalCylinder {
    pub fn new(radius: f64) -> Self {
        GeomGridEvalCylinder {
            radius: radius.abs(),
        }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        (
            self.radius * u.cos(),
            self.radius * u.sin(),
            v,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_cylinder_new() {
        let cyl = GeomGridEvalCylinder::new(2.0);
        assert!((cyl.radius - 2.0).abs() < PRECISION);
    }

    #[test]
    fn test_cylinder_evaluate() {
        let cyl = GeomGridEvalCylinder::new(1.0);
        let (x, y, z) = cyl.evaluate(0.0, 5.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!((z - 5.0).abs() < PRECISION);
    }
}
