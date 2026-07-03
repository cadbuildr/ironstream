// FILE: gce_make_lin2d.rs
// occt: gce_MakeLin2d

/// Make a 2D line.
pub struct GceMakeLin2d {
    point: (f64, f64),
    direction: (f64, f64),
    status: i32,
}

impl GceMakeLin2d {
    /// Create a line from point and direction.
    pub fn new(point: (f64, f64), direction: (f64, f64)) -> Self {
        let dir_mag = (direction.0 * direction.0 + direction.1 * direction.1).sqrt();
        let status = if dir_mag < 1e-10 {
            12 // NullVector
        } else {
            0 // Done
        };

        let norm_dir = if dir_mag > 1e-10 {
            (direction.0 / dir_mag, direction.1 / dir_mag)
        } else {
            (1.0, 0.0)
        };

        GceMakeLin2d {
            point,
            direction: norm_dir,
            status,
        }
    }

    /// Get the status.
    pub fn status(&self) -> i32 {
        self.status
    }

    /// Check if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.status == 0
    }

    /// Get the point.
    pub fn point(&self) -> (f64, f64) {
        self.point
    }

    /// Get the direction.
    pub fn direction(&self) -> (f64, f64) {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_line_valid() {
        let line = GceMakeLin2d::new((1.0, 2.0), (3.0, 4.0));
        assert!(line.is_done());
        assert_eq!(line.point(), (1.0, 2.0));
        let (dx, dy) = line.direction();
        assert!((dx * dx + dy * dy - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_make_line_zero_direction() {
        let line = GceMakeLin2d::new((0.0, 0.0), (0.0, 0.0));
        assert!(!line.is_done());
        assert_eq!(line.status(), 12);
    }
}
