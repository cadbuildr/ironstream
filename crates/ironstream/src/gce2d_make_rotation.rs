// FILE: gce2d_make_rotation.rs
// occt: GCE2d_MakeRotation

/// Creates rotation transformation in 2D.
pub struct Gce2dMakeRotation {
    done: bool,
    angle: f64,
}

impl Gce2dMakeRotation {
    pub fn new(angle: f64) -> Self {
        Gce2dMakeRotation {
            done: false,
            angle,
        }
    }

    pub fn rotate_point(&mut self, x: f64, y: f64, cx: f64, cy: f64) -> (f64, f64) {
        let cos_a = self.angle.cos();
        let sin_a = self.angle.sin();
        let dx = x - cx;
        let dy = y - cy;
        let rx = dx * cos_a - dy * sin_a;
        let ry = dx * sin_a + dy * cos_a;
        self.done = true;
        (cx + rx, cy + ry)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_new() {
        let rot = Gce2dMakeRotation::new(PI / 4.0);
        assert!(!rot.is_done());
    }

    #[test]
    fn test_rotate_point() {
        let mut rot = Gce2dMakeRotation::new(0.0);
        let (x, y) = rot.rotate_point(1.0, 0.0, 0.0, 0.0);
        assert!((x - 1.0).abs() < 0.01);
        assert!(y.abs() < 0.01);
    }
}
