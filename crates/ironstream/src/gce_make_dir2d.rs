// FILE: gce_make_dir2d.rs
// occt: gce_MakeDir2d

/// Make a 2D direction.
pub struct GceMakeDir2d {
    dir: (f64, f64),
    status: i32,
}

impl GceMakeDir2d {
    /// Create a 2D direction from vector.
    pub fn new(x: f64, y: f64) -> Self {
        let mag = (x * x + y * y).sqrt();
        if mag < 1e-10 {
            GceMakeDir2d {
                dir: (0.0, 1.0),
                status: 12, // NullVector
            }
        } else {
            GceMakeDir2d {
                dir: (x / mag, y / mag),
                status: 0, // Done
            }
        }
    }

    /// Get the direction.
    pub fn value(&self) -> (f64, f64) {
        self.dir
    }

    /// Get the status.
    pub fn status(&self) -> i32 {
        self.status
    }

    /// Check if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.status == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dir2d_normalized() {
        let dir_maker = GceMakeDir2d::new(3.0, 4.0);
        assert!(dir_maker.is_done());
        let (x, y) = dir_maker.value();
        assert!((x - 0.6).abs() < 1e-6);
        assert!((y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_make_dir2d_zero_vector() {
        let dir_maker = GceMakeDir2d::new(0.0, 0.0);
        assert!(!dir_maker.is_done());
        assert_eq!(dir_maker.status(), 12);
    }

    #[test]
    fn test_make_dir2d_unit() {
        let dir_maker = GceMakeDir2d::new(1.0, 0.0);
        assert!(dir_maker.is_done());
        let (x, y) = dir_maker.value();
        assert!((x - 1.0).abs() < 1e-6);
        assert!((y - 0.0).abs() < 1e-6);
    }
}
