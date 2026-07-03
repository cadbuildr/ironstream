// FILE: geom_grid_eval_surface_of_extrusion.rs
// occt: GeomGridEval_SurfaceOfExtrusion

pub struct GeomGridEvalSurfaceOfExtrusion {
    dir_x: f64,
    dir_y: f64,
    dir_z: f64,
}

impl GeomGridEvalSurfaceOfExtrusion {
    pub fn new(dir_x: f64, dir_y: f64, dir_z: f64) -> Self {
        let len = (dir_x * dir_x + dir_y * dir_y + dir_z * dir_z).sqrt();
        let (dx, dy, dz) = if len > 1e-15 {
            (dir_x / len, dir_y / len, dir_z / len)
        } else {
            (0.0, 0.0, 1.0)
        };
        GeomGridEvalSurfaceOfExtrusion {
            dir_x: dx,
            dir_y: dy,
            dir_z: dz,
        }
    }

    pub fn evaluate(&self, _u: f64, v: f64) -> (f64, f64, f64) {
        (
            v * self.dir_x,
            v * self.dir_y,
            v * self.dir_z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_extrusion_new() {
        let surf = GeomGridEvalSurfaceOfExtrusion::new(0.0, 0.0, 1.0);
        assert!(surf.dir_z.abs() - 1.0 < PRECISION);
    }

    #[test]
    fn test_extrusion_evaluate() {
        let surf = GeomGridEvalSurfaceOfExtrusion::new(0.0, 0.0, 1.0);
        let (x, y, z) = surf.evaluate(0.0, 2.0);
        assert!(x.abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!((z - 2.0).abs() < PRECISION);
    }
}
