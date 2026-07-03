// FILE: gce2d_make_scale.rs
// occt: GCE2d_MakeScale

/// Creates scaling transformation in 2D.
pub struct Gce2dMakeScale {
    done: bool,
    scale: f64,
}

impl Gce2dMakeScale {
    pub fn new(scale: f64) -> Self {
        Gce2dMakeScale {
            done: false,
            scale,
        }
    }

    pub fn scale_point(&mut self, x: f64, y: f64, cx: f64, cy: f64) -> (f64, f64) {
        let dx = (x - cx) * self.scale;
        let dy = (y - cy) * self.scale;
        self.done = true;
        (cx + dx, cy + dy)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn scale_value(&self) -> f64 {
        self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let scale = Gce2dMakeScale::new(2.0);
        assert!(!scale.is_done());
        assert_eq!(scale.scale_value(), 2.0);
    }

    #[test]
    fn test_scale_point() {
        let mut scale = Gce2dMakeScale::new(2.0);
        let (x, y) = scale.scale_point(2.0, 2.0, 0.0, 0.0);
        assert_eq!(x, 4.0);
        assert_eq!(y, 4.0);
    }
}
