// FILE: gce_make_dir.rs
// occt: gce_MakeDir

/// Make a 3D direction.
pub struct GceMakeDir {
    dir: (f64, f64, f64),
    status: i32,
}

impl GceMakeDir {
    /// Create a direction from vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let mag = (x * x + y * y + z * z).sqrt();
        if mag < 1e-10 {
            GceMakeDir {
                dir: (0.0, 0.0, 1.0),
                status: 7, // NullVector
            }
        } else {
            GceMakeDir {
                dir: (x / mag, y / mag, z / mag),
                status: 0, // Done
            }
        }
    }

    /// Get the direction.
    pub fn value(&self) -> (f64, f64, f64) {
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
    fn test_make_dir_normalized() {
        let dir_maker = GceMakeDir::new(3.0, 4.0, 0.0);
        assert!(dir_maker.is_done());
        let (x, y, z) = dir_maker.value();
        assert!((x - 0.6).abs() < 1e-6);
        assert!((y - 0.8).abs() < 1e-6);
        assert!((z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_make_dir_zero_vector() {
        let dir_maker = GceMakeDir::new(0.0, 0.0, 0.0);
        assert!(!dir_maker.is_done());
        assert_eq!(dir_maker.status(), 7);
    }

    #[test]
    fn test_make_dir_unit() {
        let dir_maker = GceMakeDir::new(1.0, 0.0, 0.0);
        assert!(dir_maker.is_done());
        let (x, y, z) = dir_maker.value();
        assert!((x - 1.0).abs() < 1e-6);
        assert!((y - 0.0).abs() < 1e-6);
        assert!((z - 0.0).abs() < 1e-6);
    }
}
