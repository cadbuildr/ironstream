// FILE: gce_make_mirror2d.rs
// occt: gce_MakeMirror2d

/// Make a 2D mirror transformation.
pub struct GceMakeMirror2d {
    line_point: (f64, f64),
    line_direction: (f64, f64),
    status: i32,
}

impl GceMakeMirror2d {
    /// Create a 2D mirror transformation about a line.
    pub fn new(point: (f64, f64), direction: (f64, f64)) -> Self {
        let dir_len = (direction.0 * direction.0 + direction.1 * direction.1).sqrt();
        let status = if dir_len < 1e-10 {
            12 // NullVector
        } else {
            0 // Done
        };

        let norm_direction = if dir_len > 1e-10 {
            (direction.0 / dir_len, direction.1 / dir_len)
        } else {
            (1.0, 0.0)
        };

        GceMakeMirror2d {
            line_point: point,
            line_direction: norm_direction,
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

    /// Get the line point.
    pub fn line_point(&self) -> (f64, f64) {
        self.line_point
    }

    /// Get the line direction.
    pub fn line_direction(&self) -> (f64, f64) {
        self.line_direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_mirror2d_valid() {
        let mirror = GceMakeMirror2d::new((0.0, 0.0), (1.0, 0.0));
        assert!(mirror.is_done());
        assert_eq!(mirror.line_point(), (0.0, 0.0));
        assert_eq!(mirror.line_direction(), (1.0, 0.0));
    }

    #[test]
    fn test_make_mirror2d_normalized() {
        let mirror = GceMakeMirror2d::new((1.0, 1.0), (3.0, 4.0));
        assert!(mirror.is_done());
        let (dx, dy) = mirror.line_direction();
        let mag = (dx * dx + dy * dy).sqrt();
        assert!((mag - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_make_mirror2d_zero_direction() {
        let mirror = GceMakeMirror2d::new((0.0, 0.0), (0.0, 0.0));
        assert!(!mirror.is_done());
        assert_eq!(mirror.status(), 12);
    }
}
