// FILE: geom_grid_eval_sphere.rs
// occt: GeomGridEval_Sphere

pub struct GeomGridEvalSphere {
    radius: f64,
}

impl GeomGridEvalSphere {
    pub fn new(radius: f64) -> Self {
        GeomGridEvalSphere {
            radius: radius.abs(),
        }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let cos_u = u.cos();
        let sin_u = u.sin();
        let cos_v = v.cos();
        let sin_v = v.sin();
        (
            self.radius * cos_u * cos_v,
            self.radius * sin_u * cos_v,
            self.radius * sin_v,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_sphere_new() {
        let sphere = GeomGridEvalSphere::new(2.0);
        assert!((sphere.radius - 2.0).abs() < PRECISION);
    }

    #[test]
    fn test_sphere_evaluate() {
        let sphere = GeomGridEvalSphere::new(1.0);
        let (x, y, z) = sphere.evaluate(0.0, 0.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
