// FILE: geom_grid_eval_offset_surface.rs
// occt: GeomGridEval_OffsetSurface

pub struct GeomGridEvalOffsetSurface {
    offset: f64,
}

impl GeomGridEvalOffsetSurface {
    pub fn new(offset: f64) -> Self {
        GeomGridEvalOffsetSurface { offset }
    }

    pub fn evaluate(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        (self.offset, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_offset_surface_new() {
        let surf = GeomGridEvalOffsetSurface::new(1.5);
        assert!((surf.offset - 1.5).abs() < PRECISION);
    }

    #[test]
    fn test_offset_surface_evaluate() {
        let surf = GeomGridEvalOffsetSurface::new(2.0);
        let (x, y, z) = surf.evaluate(0.5, 0.5);
        assert!((x - 2.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }
}
