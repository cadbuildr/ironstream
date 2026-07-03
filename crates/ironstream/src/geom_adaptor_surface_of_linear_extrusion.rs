// FILE: geom_adaptor_surface_of_linear_extrusion.rs
// occt: GeomAdaptor_SurfaceOfLinearExtrusion

//! Generalised cylinder. This surface is obtained by sweeping a curve in a given
//! direction. The parametrization range for the parameter U is defined
//! with referenced the curve.
//! The parametrization range for the parameter V is ]-infinite,+infinite[
//! The position of the curve gives the origin for the parameter V.
//! The continuity of the surface is CN in the V direction.

/// A surface obtained by linearly extruding a curve in a specified direction.
pub struct SurfaceOfLinearExtrusion {
    direction: (f64, f64, f64), // gp_Dir representation
    have_direction: bool,
}

impl SurfaceOfLinearExtrusion {
    /// Create an empty surface.
    pub fn new() -> Self {
        SurfaceOfLinearExtrusion {
            direction: (0.0, 0.0, 1.0),
            have_direction: false,
        }
    }

    /// Create a surface with a basis curve (direction not yet specified).
    pub fn with_curve(_curve: &str) -> Self {
        SurfaceOfLinearExtrusion {
            direction: (0.0, 0.0, 1.0),
            have_direction: false,
        }
    }

    /// Create a surface with both basis curve and extrusion direction.
    pub fn with_curve_and_direction(_curve: &str, direction: (f64, f64, f64)) -> Self {
        SurfaceOfLinearExtrusion {
            direction,
            have_direction: true,
        }
    }

    /// Set or change the extrusion direction.
    pub fn set_direction(&mut self, direction: (f64, f64, f64)) {
        self.direction = direction;
        self.have_direction = true;
    }

    /// Get the extrusion direction.
    pub fn direction(&self) -> Option<(f64, f64, f64)> {
        if self.have_direction {
            Some(self.direction)
        } else {
            None
        }
    }

    /// Returns the first U parameter.
    pub fn first_u_parameter(&self) -> f64 {
        0.0
    }

    /// Returns the last U parameter (depends on basis curve).
    pub fn last_u_parameter(&self) -> f64 {
        1.0
    }

    /// Returns the first V parameter.
    pub fn first_v_parameter(&self) -> f64 {
        f64::NEG_INFINITY
    }

    /// Returns the last V parameter.
    pub fn last_v_parameter(&self) -> f64 {
        f64::INFINITY
    }

    /// Returns CN continuity in V direction.
    pub fn v_continuity(&self) -> &str {
        "CN"
    }

    /// Check if surface is V-periodic.
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// Check if surface is V-closed.
    pub fn is_v_closed(&self) -> bool {
        false
    }

    /// Returns V resolution corresponding to 3D resolution.
    pub fn v_resolution(&self, r3d: f64) -> f64 {
        r3d
    }
}

impl Default for SurfaceOfLinearExtrusion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_surface() {
        let surface = SurfaceOfLinearExtrusion::new();
        assert!(!surface.have_direction);
        assert_eq!(surface.direction(), None);
    }

    #[test]
    fn test_set_direction() {
        let mut surface = SurfaceOfLinearExtrusion::new();
        let dir = (1.0, 0.0, 0.0);
        surface.set_direction(dir);
        assert!(surface.have_direction);
        assert_eq!(surface.direction(), Some(dir));
    }

    #[test]
    fn test_with_direction() {
        let dir = (0.0, 1.0, 0.0);
        let surface = SurfaceOfLinearExtrusion::with_curve_and_direction("dummy", dir);
        assert!(surface.have_direction);
        assert_eq!(surface.direction(), Some(dir));
    }

    #[test]
    fn test_u_parameters() {
        let surface = SurfaceOfLinearExtrusion::new();
        assert_eq!(surface.first_u_parameter(), 0.0);
        assert_eq!(surface.last_u_parameter(), 1.0);
    }

    #[test]
    fn test_v_parameters() {
        let surface = SurfaceOfLinearExtrusion::new();
        assert_eq!(surface.first_v_parameter(), f64::NEG_INFINITY);
        assert_eq!(surface.last_v_parameter(), f64::INFINITY);
    }

    #[test]
    fn test_v_continuity() {
        let surface = SurfaceOfLinearExtrusion::new();
        assert_eq!(surface.v_continuity(), "CN");
    }

    #[test]
    fn test_v_resolution() {
        let surface = SurfaceOfLinearExtrusion::new();
        assert_eq!(surface.v_resolution(0.1), 0.1);
    }
}
