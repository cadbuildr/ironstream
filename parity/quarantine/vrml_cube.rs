// FILE: vrml_cube.rs
// occt: Vrml_Cube
//
// Faithful port of OCCT Vrml_Cube (DataExchange/TKDEVRML/Vrml/
// Vrml_Cube.hxx/.cxx): VRML 1.0 Cube geometry primitive.
// Axis-aligned box with uniform side length (default 2.0).

use std::cell::RefCell;
use std::rc::Rc;

/// VRML 1.0 Cube primitive: axis-aligned box in 3D space.
/// All sides have equal length (uniform cube). Centered at origin.
/// Defaults: width 2.0 (extends from -1 to +1 on each axis).
pub struct VrmlCube {
    my_width: f64,
    my_name: String,
}

impl VrmlCube {
    /// Constructor: creates a cube with default dimensions.
    pub fn new(name: Option<&str>) -> Self {
        VrmlCube {
            my_width: 2.0,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with explicit width.
    pub fn with_width(width: f64, name: Option<&str>) -> Self {
        VrmlCube {
            my_width: width.max(0.0),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Get the cube side length.
    pub fn width(&self) -> f64 {
        self.my_width
    }

    /// Set the cube side length (enforces non-negative).
    pub fn set_width(&mut self, width: f64) {
        self.my_width = width.max(0.0);
    }

    /// Check if this cube is in default state (width == 2.0).
    pub fn is_default(&self) -> bool {
        (self.my_width - 2.0).abs() < 1e-10
    }

    /// Compute the surface area (6 * side^2).
    pub fn surface_area(&self) -> f64 {
        6.0 * self.my_width * self.my_width
    }

    /// Compute the volume (side^3).
    pub fn volume(&self) -> f64 {
        self.my_width * self.my_width * self.my_width
    }

    /// Get the half-width (extent from center to face).
    pub fn half_width(&self) -> f64 {
        self.my_width / 2.0
    }

    /// Get the cube bounds (min and max corners).
    pub fn bounds(&self) -> ((f64, f64, f64), (f64, f64, f64)) {
        let hw = self.half_width();
        ((-hw, -hw, -hw), (hw, hw, hw))
    }

    /// Get the diagonal length (space diagonal of the cube).
    pub fn diagonal(&self) -> f64 {
        self.my_width * (3.0_f64).sqrt()
    }
}

impl Default for VrmlCube {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlCube {
    fn clone(&self) -> Self {
        VrmlCube {
            my_width: self.my_width,
            my_name: self.my_name.clone(),
        }
    }
}

impl PartialEq for VrmlCube {
    fn eq(&self, other: &Self) -> bool {
        (self.my_width - other.my_width).abs() < 1e-10 && self.my_name == other.my_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_cube() {
        let cube = VrmlCube::new(None);
        assert_eq!(cube.width(), 2.0);
        assert!(cube.is_default());
    }

    #[test]
    fn named_cube() {
        let cube = VrmlCube::new(Some("MyCube"));
        assert_eq!(cube.name(), "MyCube");
    }

    #[test]
    fn with_width() {
        let cube = VrmlCube::with_width(5.0, Some("Big"));
        assert_eq!(cube.width(), 5.0);
        assert_eq!(cube.name(), "Big");
    }

    #[test]
    fn set_width() {
        let mut cube = VrmlCube::new(None);
        cube.set_width(3.5);
        assert_eq!(cube.width(), 3.5);
    }

    #[test]
    fn width_non_negative() {
        let cube = VrmlCube::with_width(-2.0, None);
        assert_eq!(cube.width(), 0.0);
    }

    #[test]
    fn surface_area() {
        let cube = VrmlCube::new(None); // width 2.0
        let area = cube.surface_area();
        // 6 * 2^2 = 24
        assert!((area - 24.0).abs() < 1e-10);
    }

    #[test]
    fn surface_area_unit_cube() {
        let cube = VrmlCube::with_width(1.0, None);
        let area = cube.surface_area();
        // 6 * 1^2 = 6
        assert!((area - 6.0).abs() < 1e-10);
    }

    #[test]
    fn volume() {
        let cube = VrmlCube::new(None); // width 2.0
        let vol = cube.volume();
        // 2^3 = 8
        assert!((vol - 8.0).abs() < 1e-10);
    }

    #[test]
    fn volume_unit_cube() {
        let cube = VrmlCube::with_width(1.0, None);
        let vol = cube.volume();
        // 1^3 = 1
        assert!((vol - 1.0).abs() < 1e-10);
    }

    #[test]
    fn half_width() {
        let cube = VrmlCube::with_width(4.0, None);
        assert!((cube.half_width() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn bounds() {
        let cube = VrmlCube::with_width(2.0, None);
        let (min, max) = cube.bounds();
        assert!((min.0 - (-1.0)).abs() < 1e-10);
        assert!((max.0 - 1.0).abs() < 1e-10);
        assert!((min.1 - (-1.0)).abs() < 1e-10);
        assert!((max.1 - 1.0).abs() < 1e-10);
        assert!((min.2 - (-1.0)).abs() < 1e-10);
        assert!((max.2 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn diagonal() {
        let cube = VrmlCube::with_width(1.0, None);
        let diag = cube.diagonal();
        // diag = 1 * sqrt(3) ~= 1.732
        let expected = (3.0_f64).sqrt();
        assert!((diag - expected).abs() < 1e-10);
    }

    #[test]
    fn clone_preserves_data() {
        let cube = VrmlCube::with_width(3.0, Some("Original"));
        let cloned = cube.clone();
        assert_eq!(cloned.width(), 3.0);
        assert_eq!(cloned.name(), "Original");
    }

    #[test]
    fn equality() {
        let c1 = VrmlCube::with_width(2.5, Some("C"));
        let c2 = VrmlCube::with_width(2.5, Some("C"));
        assert_eq!(c1, c2);
    }

    #[test]
    fn inequality_different_width() {
        let c1 = VrmlCube::with_width(2.0, None);
        let c2 = VrmlCube::with_width(3.0, None);
        assert_ne!(c1, c2);
    }

    #[test]
    fn set_name() {
        let mut cube = VrmlCube::new(Some("Old"));
        cube.set_name("New");
        assert_eq!(cube.name(), "New");
    }
}
