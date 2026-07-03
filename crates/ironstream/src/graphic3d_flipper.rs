// FILE: graphic3d_flipper.rs
// occt: Graphic3d_Flipper

/// Reference coordinate system for rendering upright geometry.
#[derive(Debug, Clone, Copy)]
pub struct Ax2 {
    location: (f64, f64, f64),
    x_direction: (f64, f64, f64),
    y_direction: (f64, f64, f64),
    z_direction: (f64, f64, f64),
}

impl Ax2 {
    /// Create coordinate system.
    pub fn new(
        location: (f64, f64, f64),
        x_direction: (f64, f64, f64),
        y_direction: (f64, f64, f64),
        z_direction: (f64, f64, f64),
    ) -> Self {
        Ax2 {
            location,
            x_direction,
            y_direction,
            z_direction,
        }
    }

    /// Get location.
    pub fn location(&self) -> (f64, f64, f64) {
        self.location
    }

    /// Get X direction.
    pub fn x_direction(&self) -> (f64, f64, f64) {
        self.x_direction
    }

    /// Get Y direction.
    pub fn y_direction(&self) -> (f64, f64, f64) {
        self.y_direction
    }

    /// Get Z direction.
    pub fn z_direction(&self) -> (f64, f64, f64) {
        self.z_direction
    }
}

/// CPU-side analogue of OpenGl_Flipper used by the selection pipeline.
/// Describes reference coordinate system for mirroring group contents.
pub struct Graphic3dFlipper {
    ref_plane: Ax2,
}

impl Graphic3dFlipper {
    /// Create flipper with reference plane.
    pub fn new(ref_plane: Ax2) -> Self {
        Graphic3dFlipper { ref_plane }
    }

    /// Get reference plane.
    pub fn ref_plane(&self) -> Ax2 {
        self.ref_plane
    }

    /// Set reference plane.
    pub fn set_ref_plane(&mut self, ref_plane: Ax2) {
        self.ref_plane = ref_plane;
    }

    /// Compute flipping transformation matrix.
    /// Returns identity-like bool indicating if transform is identity.
    pub fn compute_is_identity(&self) -> bool {
        // Simplified: returns true if reference plane is at origin with standard axes
        self.ref_plane.location == (0.0, 0.0, 0.0)
            && self.ref_plane.x_direction == (1.0, 0.0, 0.0)
            && self.ref_plane.y_direction == (0.0, 1.0, 0.0)
            && self.ref_plane.z_direction == (0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ax2_new() {
        let ax2 = Ax2::new(
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
        );
        assert_eq!(ax2.location(), (0.0, 0.0, 0.0));
        assert_eq!(ax2.x_direction(), (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_flipper_new() {
        let ax2 = Ax2::new(
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
        );
        let flipper = Graphic3dFlipper::new(ax2);
        assert_eq!(flipper.ref_plane().location(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_compute_is_identity() {
        let ax2 = Ax2::new(
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
        );
        let flipper = Graphic3dFlipper::new(ax2);
        assert!(flipper.compute_is_identity());
    }

    #[test]
    fn test_compute_is_identity_false() {
        let ax2 = Ax2::new(
            (1.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
        );
        let flipper = Graphic3dFlipper::new(ax2);
        assert!(!flipper.compute_is_identity());
    }
}
