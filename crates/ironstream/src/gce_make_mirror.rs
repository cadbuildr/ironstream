// FILE: gce_make_mirror.rs
// occt: gce_MakeMirror

/// Make a 3D mirror transformation.
pub struct GceMakeMirror {
    plane_position: (f64, f64, f64),
    plane_normal: (f64, f64, f64),
    status: i32,
}

impl GceMakeMirror {
    /// Create a mirror transformation about a plane.
    pub fn new(position: (f64, f64, f64), normal: (f64, f64, f64)) -> Self {
        let norm_len = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
        let status = if norm_len < 1e-10 {
            12 // NullVector
        } else {
            0 // Done
        };

        let norm_normal = if norm_len > 1e-10 {
            (
                normal.0 / norm_len,
                normal.1 / norm_len,
                normal.2 / norm_len,
            )
        } else {
            (0.0, 0.0, 1.0)
        };

        GceMakeMirror {
            plane_position: position,
            plane_normal: norm_normal,
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

    /// Get the plane position.
    pub fn plane_position(&self) -> (f64, f64, f64) {
        self.plane_position
    }

    /// Get the plane normal.
    pub fn plane_normal(&self) -> (f64, f64, f64) {
        self.plane_normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_mirror_valid() {
        let mirror = GceMakeMirror::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        assert!(mirror.is_done());
        assert_eq!(mirror.plane_position(), (0.0, 0.0, 0.0));
        assert_eq!(mirror.plane_normal(), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_make_mirror_normalized() {
        let mirror = GceMakeMirror::new((1.0, 2.0, 3.0), (3.0, 4.0, 0.0));
        assert!(mirror.is_done());
        let (nx, ny, nz) = mirror.plane_normal();
        let mag = (nx * nx + ny * ny + nz * nz).sqrt();
        assert!((mag - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_make_mirror_zero_normal() {
        let mirror = GceMakeMirror::new((0.0, 0.0, 0.0), (0.0, 0.0, 0.0));
        assert!(!mirror.is_done());
        assert_eq!(mirror.status(), 12);
    }
}
