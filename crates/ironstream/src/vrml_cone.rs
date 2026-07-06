// FILE: vrml_cone.rs
// occt: Vrml_Cone
//
// Faithful port of OCCT Vrml_Cone (DataExchange/TKDEVRML/Vrml/
// Vrml_Cone.hxx/.cxx): VRML 1.0 Cone geometry primitive.
// Defines a cone with radius, height, and parts (sides, bottom).

use std::cell::RefCell;
use std::rc::Rc;

/// Cone parts bitmask flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlConeParts {
    /// Render the cone side surface.
    Sides = 1,
    /// Render the cone bottom.
    Bottom = 2,
    /// Render both sides and bottom (default).
    All = 3,
}

impl VrmlConeParts {
    pub fn includes_sides(&self) -> bool {
        (*self as i32) & 1 != 0
    }

    pub fn includes_bottom(&self) -> bool {
        (*self as i32) & 2 != 0
    }

    pub fn as_int(&self) -> i32 {
        *self as i32
    }
}

impl Default for VrmlConeParts {
    fn default() -> Self {
        VrmlConeParts::All
    }
}

/// VRML 1.0 Cone primitive: cone geometry in 3D space.
/// Height is along Y-axis by default; apex at top, base at bottom.
/// Defaults: height 2.0, radius 1.0, parts ALL.
#[derive(Debug)]
pub struct VrmlCone {
    my_height: f64,
    my_radius: f64,
    my_parts: VrmlConeParts,
    my_name: String,
}

impl VrmlCone {
    /// Constructor: creates a cone with default dimensions.
    pub fn new(name: Option<&str>) -> Self {
        VrmlCone {
            my_height: 2.0,
            my_radius: 1.0,
            my_parts: VrmlConeParts::All,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with explicit parameters.
    pub fn with_fields(
        height: f64,
        radius: f64,
        parts: VrmlConeParts,
        name: Option<&str>,
    ) -> Self {
        VrmlCone {
            my_height: height.max(0.0),
            my_radius: radius.max(0.0),
            my_parts: parts,
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

    /// Get the cone height.
    pub fn height(&self) -> f64 {
        self.my_height
    }

    /// Set the cone height (enforces non-negative).
    pub fn set_height(&mut self, height: f64) {
        self.my_height = height.max(0.0);
    }

    /// Get the cone radius.
    pub fn radius(&self) -> f64 {
        self.my_radius
    }

    /// Set the cone radius (enforces non-negative).
    pub fn set_radius(&mut self, radius: f64) {
        self.my_radius = radius.max(0.0);
    }

    /// Get the cone parts to render.
    pub fn parts(&self) -> VrmlConeParts {
        self.my_parts
    }

    /// Set the cone parts to render.
    pub fn set_parts(&mut self, parts: VrmlConeParts) {
        self.my_parts = parts;
    }

    /// Check if this cone is in default state.
    pub fn is_default(&self) -> bool {
        (self.my_height - 2.0).abs() < 1e-10
            && (self.my_radius - 1.0).abs() < 1e-10
            && self.my_parts == VrmlConeParts::All
    }

    /// Compute the surface area (sides + bottom if included).
    pub fn surface_area(&self) -> f64 {
        let mut area = 0.0;

        if self.my_parts.includes_sides() {
            // Lateral surface: pi * r * s (where s is slant height)
            let slant = (self.my_height * self.my_height + self.my_radius * self.my_radius).sqrt();
            area += std::f64::consts::PI * self.my_radius * slant;
        }

        if self.my_parts.includes_bottom() {
            // Base area: pi * r^2
            area += std::f64::consts::PI * self.my_radius * self.my_radius;
        }

        area
    }

    /// Compute the volume (1/3 * pi * r^2 * h).
    pub fn volume(&self) -> f64 {
        (1.0 / 3.0) * std::f64::consts::PI * self.my_radius * self.my_radius * self.my_height
    }

    /// Get the slant height (distance from apex to base edge).
    pub fn slant_height(&self) -> f64 {
        (self.my_height * self.my_height + self.my_radius * self.my_radius).sqrt()
    }
}

impl Default for VrmlCone {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlCone {
    fn clone(&self) -> Self {
        VrmlCone {
            my_height: self.my_height,
            my_radius: self.my_radius,
            my_parts: self.my_parts,
            my_name: self.my_name.clone(),
        }
    }
}

impl PartialEq for VrmlCone {
    fn eq(&self, other: &Self) -> bool {
        (self.my_height - other.my_height).abs() < 1e-10
            && (self.my_radius - other.my_radius).abs() < 1e-10
            && self.my_parts == other.my_parts
            && self.my_name == other.my_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_cone() {
        let cone = VrmlCone::new(None);
        assert_eq!(cone.height(), 2.0);
        assert_eq!(cone.radius(), 1.0);
        assert_eq!(cone.parts(), VrmlConeParts::All);
        assert!(cone.is_default());
    }

    #[test]
    fn named_cone() {
        let cone = VrmlCone::new(Some("MyCone"));
        assert_eq!(cone.name(), "MyCone");
    }

    #[test]
    fn with_fields() {
        let cone = VrmlCone::with_fields(3.0, 2.5, VrmlConeParts::Sides, Some("Cone1"));
        assert_eq!(cone.height(), 3.0);
        assert_eq!(cone.radius(), 2.5);
        assert_eq!(cone.parts(), VrmlConeParts::Sides);
    }

    #[test]
    fn set_height() {
        let mut cone = VrmlCone::new(None);
        cone.set_height(5.0);
        assert_eq!(cone.height(), 5.0);
    }

    #[test]
    fn height_non_negative() {
        let cone = VrmlCone::with_fields(-2.0, 1.0, VrmlConeParts::All, None);
        assert_eq!(cone.height(), 0.0);
    }

    #[test]
    fn set_radius() {
        let mut cone = VrmlCone::new(None);
        cone.set_radius(3.5);
        assert_eq!(cone.radius(), 3.5);
    }

    #[test]
    fn radius_non_negative() {
        let cone = VrmlCone::with_fields(2.0, -1.5, VrmlConeParts::All, None);
        assert_eq!(cone.radius(), 0.0);
    }

    #[test]
    fn cone_parts_sides() {
        assert!(VrmlConeParts::Sides.includes_sides());
        assert!(!VrmlConeParts::Sides.includes_bottom());
        assert!(VrmlConeParts::All.includes_sides());
        assert!(VrmlConeParts::All.includes_bottom());
    }

    #[test]
    fn set_parts() {
        let mut cone = VrmlCone::new(None);
        cone.set_parts(VrmlConeParts::Bottom);
        assert_eq!(cone.parts(), VrmlConeParts::Bottom);
        assert!(!cone.is_default());
    }

    #[test]
    fn surface_area_sides_only() {
        let cone = VrmlCone::with_fields(3.0, 1.0, VrmlConeParts::Sides, None);
        let area = cone.surface_area();
        // Slant height = sqrt(9 + 1) = sqrt(10)
        // Lateral area = pi * 1 * sqrt(10)
        let slant = (9.0 + 1.0_f64).sqrt();
        let expected = std::f64::consts::PI * slant;
        assert!((area - expected).abs() < 1e-10);
    }

    #[test]
    fn surface_area_bottom_only() {
        let cone = VrmlCone::with_fields(2.0, 2.0, VrmlConeParts::Bottom, None);
        let area = cone.surface_area();
        // Base area = pi * 2^2 = 4*pi
        let expected = 4.0 * std::f64::consts::PI;
        assert!((area - expected).abs() < 1e-10);
    }

    #[test]
    fn volume() {
        let cone = VrmlCone::with_fields(3.0, 2.0, VrmlConeParts::All, None);
        let vol = cone.volume();
        // V = (1/3) * pi * 4 * 3 = 4*pi
        let expected = 4.0 * std::f64::consts::PI;
        assert!((vol - expected).abs() < 1e-10);
    }

    #[test]
    fn slant_height() {
        let cone = VrmlCone::with_fields(3.0, 4.0, VrmlConeParts::All, None);
        let slant = cone.slant_height();
        // slant = sqrt(9 + 16) = 5
        assert!((slant - 5.0).abs() < 1e-10);
    }

    #[test]
    fn clone_preserves_data() {
        let cone = VrmlCone::with_fields(2.5, 1.5, VrmlConeParts::Sides, Some("Original"));
        let cloned = cone.clone();
        assert_eq!(cloned.height(), 2.5);
        assert_eq!(cloned.radius(), 1.5);
        assert_eq!(cloned.parts(), VrmlConeParts::Sides);
        assert_eq!(cloned.name(), "Original");
    }

    #[test]
    fn equality() {
        let c1 = VrmlCone::with_fields(2.0, 1.0, VrmlConeParts::All, Some("C"));
        let c2 = VrmlCone::with_fields(2.0, 1.0, VrmlConeParts::All, Some("C"));
        assert_eq!(c1, c2);
    }

    #[test]
    fn inequality_different_height() {
        let c1 = VrmlCone::with_fields(2.0, 1.0, VrmlConeParts::All, None);
        let c2 = VrmlCone::with_fields(3.0, 1.0, VrmlConeParts::All, None);
        assert_ne!(c1, c2);
    }
}
