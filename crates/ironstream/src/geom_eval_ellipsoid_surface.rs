// FILE: geom_eval_ellipsoid_surface.rs
// occt: GeomEval_EllipsoidSurface

pub struct GeomEvalEllipsoidSurface {
    major: f64,
    minor: f64,
    height: f64,
}

impl GeomEvalEllipsoidSurface {
    pub fn new(major: f64, minor: f64) -> Self {
        GeomEvalEllipsoidSurface {
            major,
            minor,
            height: minor,
        }
    }

    pub fn evaluate(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let cos_u = u.cos();
        let sin_u = u.sin();
        let cos_v = v.cos();
        let sin_v = v.sin();
        (
            self.major * cos_u * cos_v,
            self.minor * sin_u * cos_v,
            self.height * sin_v,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_ellipsoid_new() {
        let ellipsoid = GeomEvalEllipsoidSurface::new(2.0, 1.0);
        assert!((ellipsoid.major - 2.0).abs() < PRECISION);
        assert!((ellipsoid.minor - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_ellipsoid_evaluate() {
        let ellipsoid = GeomEvalEllipsoidSurface::new(2.0, 1.0);
        let (x, y, z) = ellipsoid.evaluate(0.0, 0.0);
        assert!((x - 2.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
