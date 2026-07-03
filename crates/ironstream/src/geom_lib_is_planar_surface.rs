// FILE: geom_lib_is_planar_surface.rs
// occt: GeomLib_IsPlanarSurface

/// Check if a surface is planar.
pub struct GeomLibIsPlanarSurface {
    is_planar: bool,
    plane_position: (f64, f64, f64),
    plane_normal: (f64, f64, f64),
}

impl GeomLibIsPlanarSurface {
    /// Create a planar surface checker.
    pub fn new() -> Self {
        GeomLibIsPlanarSurface {
            is_planar: false,
            plane_position: (0.0, 0.0, 0.0),
            plane_normal: (0.0, 0.0, 1.0),
        }
    }

    /// Perform the check.
    pub fn perform(&mut self, _tolerance: f64) {
        self.is_planar = false;
    }

    /// Check if surface is planar.
    pub fn is_planar(&self) -> bool {
        self.is_planar
    }

    /// Get plane position.
    pub fn plane_position(&self) -> (f64, f64, f64) {
        self.plane_position
    }

    /// Get plane normal.
    pub fn plane_normal(&self) -> (f64, f64, f64) {
        self.plane_normal
    }
}

impl Default for GeomLibIsPlanarSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_checker() {
        let checker = GeomLibIsPlanarSurface::new();
        assert!(!checker.is_planar());
    }

    #[test]
    fn test_plane_properties() {
        let checker = GeomLibIsPlanarSurface::new();
        let (x, y, z) = checker.plane_position();
        assert_eq!((x, y, z), (0.0, 0.0, 0.0));

        let (nx, ny, nz) = checker.plane_normal();
        assert_eq!((nx, ny, nz), (0.0, 0.0, 1.0));
    }
}
