// FILE: hlrb_rep_shape_bounds.rs
// occt: HLRBRep_ShapeBounds

pub struct HlrbrépShapeBounds {
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
}

impl HlrbrépShapeBounds {
    pub fn new() -> Self {
        HlrbrépShapeBounds {
            xmin: 0.0, ymin: 0.0, zmin: 0.0,
            xmax: 0.0, ymax: 0.0, zmax: 0.0,
        }
    }

    pub fn set_bounds(&mut self, xmin: f64, ymin: f64, zmin: f64, xmax: f64, ymax: f64, zmax: f64) {
        self.xmin = xmin;
        self.ymin = ymin;
        self.zmin = zmin;
        self.xmax = xmax;
        self.ymax = ymax;
        self.zmax = zmax;
    }

    pub fn bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        (self.xmin, self.ymin, self.zmin, self.xmax, self.ymax, self.zmax)
    }
}

impl Default for HlrbrépShapeBounds {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let sb = HlrbrépShapeBounds::new();
        assert_eq!(sb.bounds(), (0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    }
}
