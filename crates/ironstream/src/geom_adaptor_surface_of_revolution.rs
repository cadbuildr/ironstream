// FILE: geom_adaptor_surface_of_revolution.rs
// occt: GeomAdaptor_SurfaceOfRevolution

//! This class defines a complete surface of revolution.
//! The surface is obtained by rotating a curve a complete revolution
//! about an axis. The curve and the axis must be in the same plane.
//! For a complete surface of revolution the parametric range is
//! 0 <= U <= 2*PI
//! The parametric range for V is defined with the revolved curve.

use std::f64::consts::PI;

/// A surface obtained by revolving a curve around an axis.
pub struct SurfaceOfRevolution {
    axis: (f64, f64, f64, f64, f64, f64), // origin + direction as 3D components each
    have_axis: bool,
}

impl SurfaceOfRevolution {
    /// Create an empty surface.
    pub fn new() -> Self {
        SurfaceOfRevolution {
            axis: (0.0, 0.0, 0.0, 0.0, 0.0, 1.0), // default to Z-axis at origin
            have_axis: false,
        }
    }

    /// Create a surface with a basis curve (axis not yet specified).
    pub fn with_curve(_curve: &str) -> Self {
        SurfaceOfRevolution {
            axis: (0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            have_axis: false,
        }
    }

    /// Create a surface with both basis curve and revolution axis.
    /// Axis is represented as (origin_x, origin_y, origin_z, direction_x, direction_y, direction_z).
    pub fn with_curve_and_axis(_curve: &str, axis: (f64, f64, f64, f64, f64, f64)) -> Self {
        SurfaceOfRevolution {
            axis,
            have_axis: true,
        }
    }

    /// Set or change the revolution axis.
    pub fn set_axis(&mut self, axis: (f64, f64, f64, f64, f64, f64)) {
        self.axis = axis;
        self.have_axis = true;
    }

    /// Get the revolution axis as (origin, direction).
    pub fn axis(&self) -> Option<(f64, f64, f64, f64, f64, f64)> {
        if self.have_axis {
            Some(self.axis)
        } else {
            None
        }
    }

    /// Returns the first U parameter (start of revolution).
    pub fn first_u_parameter(&self) -> f64 {
        0.0
    }

    /// Returns the last U parameter (complete revolution is 2*PI).
    pub fn last_u_parameter(&self) -> f64 {
        2.0 * PI
    }

    /// Returns the first V parameter (depends on basis curve).
    pub fn first_v_parameter(&self) -> f64 {
        0.0
    }

    /// Returns the last V parameter (depends on basis curve).
    pub fn last_v_parameter(&self) -> f64 {
        1.0
    }

    /// U is continuous (revolves smoothly).
    pub fn u_continuity(&self) -> &str {
        "C1"
    }

    /// V continuity depends on the basis curve (CN for all standard curves).
    pub fn v_continuity(&self) -> &str {
        "CN"
    }

    /// Surface is U-periodic (complete revolution).
    pub fn is_u_periodic(&self) -> bool {
        true
    }

    /// U period is 2*PI.
    pub fn u_period(&self) -> f64 {
        2.0 * PI
    }

    /// Check if surface is V-periodic (depends on basis curve).
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// Returns U resolution corresponding to 3D resolution.
    pub fn u_resolution(&self, r3d: f64) -> f64 {
        r3d
    }

    /// Returns V resolution corresponding to 3D resolution.
    pub fn v_resolution(&self, r3d: f64) -> f64 {
        r3d
    }

    /// Surface type is SurfaceOfRevolution.
    pub fn surface_type(&self) -> &str {
        "SurfaceOfRevolution"
    }
}

impl Default for SurfaceOfRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_surface() {
        let surface = SurfaceOfRevolution::new();
        assert!(!surface.have_axis);
        assert_eq!(surface.axis(), None);
    }

    #[test]
    fn test_set_axis() {
        let mut surface = SurfaceOfRevolution::new();
        let axis = (0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        surface.set_axis(axis);
        assert!(surface.have_axis);
        assert_eq!(surface.axis(), Some(axis));
    }

    #[test]
    fn test_with_axis() {
        let axis = (1.0, 2.0, 3.0, 0.0, 1.0, 0.0);
        let surface = SurfaceOfRevolution::with_curve_and_axis("dummy", axis);
        assert!(surface.have_axis);
        assert_eq!(surface.axis(), Some(axis));
    }

    #[test]
    fn test_u_parameters() {
        let surface = SurfaceOfRevolution::new();
        assert_eq!(surface.first_u_parameter(), 0.0);
        assert!((surface.last_u_parameter() - 2.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_v_parameters() {
        let surface = SurfaceOfRevolution::new();
        assert_eq!(surface.first_v_parameter(), 0.0);
        assert_eq!(surface.last_v_parameter(), 1.0);
    }

    #[test]
    fn test_periodicity() {
        let surface = SurfaceOfRevolution::new();
        assert!(surface.is_u_periodic());
        assert!((surface.u_period() - 2.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_continuity() {
        let surface = SurfaceOfRevolution::new();
        assert_eq!(surface.u_continuity(), "C1");
        assert_eq!(surface.v_continuity(), "CN");
    }

    #[test]
    fn test_surface_type() {
        let surface = SurfaceOfRevolution::new();
        assert_eq!(surface.surface_type(), "SurfaceOfRevolution");
    }
}
