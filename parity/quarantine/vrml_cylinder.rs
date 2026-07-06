// FILE: vrml_cylinder.rs
// occt: Vrml_Cylinder
//
// Faithful port of OCCT Vrml_Cylinder (DataExchange/TKDEVRML/Vrml/
// Vrml_Cylinder.hxx/.cxx): VRML 1.0 Cylinder geometry primitive.
// Defines a cylinder with radius, height, and parts (sides, top, bottom).

use std::cell::RefCell;
use std::rc::Rc;

/// Cylinder parts bitmask flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlCylinderParts {
    /// Render the cylinder side surface.
    Sides = 1,
    /// Render the cylinder top.
    Top = 2,
    /// Render the cylinder bottom.
    Bottom = 4,
    /// Render all surfaces (sides, top, bottom).
    All = 7,
}

impl VrmlCylinderParts {
    pub fn includes_sides(&self) -> bool {
        (*self as i32) & 1 != 0
    }

    pub fn includes_top(&self) -> bool {
        (*self as i32) & 2 != 0
    }

    pub fn includes_bottom(&self) -> bool {
        (*self as i32) & 4 != 0
    }

    pub fn as_int(&self) -> i32 {
        *self as i32
    }
}

impl Default for VrmlCylinderParts {
    fn default() -> Self {
        VrmlCylinderParts::All
    }
}

/// VRML 1.0 Cylinder primitive: cylinder geometry in 3D space.
/// Height is along Y-axis by default; axis-aligned geometry.
/// Defaults: height 2.0, radius 1.0, parts ALL.
pub struct VrmlCylinder {
    my_height: f64,
    my_radius: f64,
    my_parts: VrmlCylinderParts,
    my_name: String,
}

impl VrmlCylinder {
    /// Constructor: creates a cylinder with default dimensions.
    pub fn new(name: Option<&str>) -> Self {
        VrmlCylinder {
            my_height: 2.0,
            my_radius: 1.0,
            my_parts: VrmlCylinderParts::All,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with explicit parameters.
    pub fn with_fields(
        height: f64,
        radius: f64,
        parts: VrmlCylinderParts,
        name: Option<&str>,
    ) -> Self {
        VrmlCylinder {
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

    /// Get the cylinder height.
    pub fn height(&self) -> f64 {
        self.my_height
    }

    /// Set the cylinder height (enforces non-negative).
    pub fn set_height(&mut self, height: f64) {
        self.my_height = height.max(0.0);
    }

    /// Get the cylinder radius.
    pub fn radius(&self) -> f64 {
        self.my_radius
    }

    /// Set the cylinder radius (enforces non-negative).
    pub fn set_radius(&mut self, radius: f64) {
        self.my_radius = radius.max(0.0);
    }

    /// Get the cylinder parts to render.
    pub fn parts(&self) -> VrmlCylinderParts {
        self.my_parts
    }

    /// Set the cylinder parts to render.
    pub fn set_parts(&mut self, parts: VrmlCylinderParts) {
        self.my_parts = parts;
    }

    /// Check if this cylinder is in default state.
    pub fn is_default(&self) -> bool {
        (self.my_height - 2.0).abs() < 1e-10
            && (self.my_radius - 1.0).abs() < 1e-10
            && self.my_parts == VrmlCylinderParts::All
    }

    /// Compute the surface area (sides + top + bottom if included).
    pub fn surface_area(&self) -> f64 {
        let mut area = 0.0;

        if self.my_parts.includes_sides() {
            // Lateral surface: 2 * pi * r * h
            area += 2.0 * std::f64::consts::PI * self.my_radius * self.my_height;
        }

        if self.my_parts.includes_top() || self.my_parts.includes_bottom() {
            // Each circular face: pi * r^2
            let base_area = std::f64::consts::PI * self.my_radius * self.my_radius;
            if self.my_parts.includes_top() {
                area += base_area;
            }
            if self.my_parts.includes_bottom() {
                area += base_area;
            }
        }

        area
    }

    /// Compute the volume (pi * r^2 * h).
    pub fn volume(&self) -> f64 {
        std::f64::consts::PI * self.my_radius * self.my_radius * self.my_height
    }
}

impl Default for VrmlCylinder {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlCylinder {
    fn clone(&self) -> Self {
        VrmlCylinder {
            my_height: self.my_height,
            my_radius: self.my_radius,
            my_parts: self.my_parts,
            my_name: self.my_name.clone(),
        }
    }
}

impl PartialEq for VrmlCylinder {
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
    fn default_cylinder() {
        let cyl = VrmlCylinder::new(None);
        assert_eq!(cyl.height(), 2.0);
        assert_eq!(cyl.radius(), 1.0);
        assert_eq!(cyl.parts(), VrmlCylinderParts::All);
        assert!(cyl.is_default());
    }

    #[test]
    fn named_cylinder() {
        let cyl = VrmlCylinder::new(Some("MyCylinder"));
        assert_eq!(cyl.name(), "MyCylinder");
    }

    #[test]
    fn with_fields() {
        let cyl = VrmlCylinder::with_fields(5.0, 2.5, VrmlCylinderParts::Sides, Some("Cyl1"));
        assert_eq!(cyl.height(), 5.0);
        assert_eq!(cyl.radius(), 2.5);
        assert_eq!(cyl.parts(), VrmlCylinderParts::Sides);
    }

    #[test]
    fn set_height() {
        let mut cyl = VrmlCylinder::new(None);
        cyl.set_height(4.0);
        assert_eq!(cyl.height(), 4.0);
    }

    #[test]
    fn height_non_negative() {
        let cyl = VrmlCylinder::with_fields(-3.0, 1.0, VrmlCylinderParts::All, None);
        assert_eq!(cyl.height(), 0.0);
    }

    #[test]
    fn set_radius() {
        let mut cyl = VrmlCylinder::new(None);
        cyl.set_radius(2.5);
        assert_eq!(cyl.radius(), 2.5);
    }

    #[test]
    fn radius_non_negative() {
        let cyl = VrmlCylinder::with_fields(2.0, -1.0, VrmlCylinderParts::All, None);
        assert_eq!(cyl.radius(), 0.0);
    }

    #[test]
    fn cylinder_parts_sides() {
        assert!(VrmlCylinderParts::Sides.includes_sides());
        assert!(!VrmlCylinderParts::Sides.includes_top());
        assert!(!VrmlCylinderParts::Sides.includes_bottom());
    }

    #[test]
    fn cylinder_parts_all() {
        assert!(VrmlCylinderParts::All.includes_sides());
        assert!(VrmlCylinderParts::All.includes_top());
        assert!(VrmlCylinderParts::All.includes_bottom());
    }

    #[test]
    fn set_parts() {
        let mut cyl = VrmlCylinder::new(None);
        cyl.set_parts(VrmlCylinderParts::Top);
        assert_eq!(cyl.parts(), VrmlCylinderParts::Top);
    }

    #[test]
    fn surface_area_sides_only() {
        let cyl = VrmlCylinder::with_fields(3.0, 1.0, VrmlCylinderParts::Sides, None);
        let area = cyl.surface_area();
        // Lateral: 2*pi*1*3 = 6*pi
        let expected = 6.0 * std::f64::consts::PI;
        assert!((area - expected).abs() < 1e-10);
    }

    #[test]
    fn surface_area_with_tops() {
        let cyl = VrmlCylinder::with_fields(2.0, 2.0, VrmlCylinderParts::All, None);
        let area = cyl.surface_area();
        // Lateral: 2*pi*2*2 = 8*pi
        // Tops: 2 * pi*4 = 8*pi
        // Total: 16*pi
        let expected = 16.0 * std::f64::consts::PI;
        assert!((area - expected).abs() < 1e-10);
    }

    #[test]
    fn volume() {
        let cyl = VrmlCylinder::with_fields(4.0, 2.0, VrmlCylinderParts::All, None);
        let vol = cyl.volume();
        // V = pi * 4 * 4 = 16*pi
        let expected = 16.0 * std::f64::consts::PI;
        assert!((vol - expected).abs() < 1e-10);
    }

    #[test]
    fn clone_preserves_data() {
        let cyl = VrmlCylinder::with_fields(3.5, 1.5, VrmlCylinderParts::Sides, Some("Original"));
        let cloned = cyl.clone();
        assert_eq!(cloned.height(), 3.5);
        assert_eq!(cloned.radius(), 1.5);
        assert_eq!(cloned.parts(), VrmlCylinderParts::Sides);
        assert_eq!(cloned.name(), "Original");
    }

    #[test]
    fn equality() {
        let c1 = VrmlCylinder::with_fields(2.0, 1.0, VrmlCylinderParts::All, Some("C"));
        let c2 = VrmlCylinder::with_fields(2.0, 1.0, VrmlCylinderParts::All, Some("C"));
        assert_eq!(c1, c2);
    }

    #[test]
    fn inequality_different_parts() {
        let c1 = VrmlCylinder::with_fields(2.0, 1.0, VrmlCylinderParts::All, None);
        let c2 = VrmlCylinder::with_fields(2.0, 1.0, VrmlCylinderParts::Sides, None);
        assert_ne!(c1, c2);
    }

    #[test]
    fn set_name() {
        let mut cyl = VrmlCylinder::new(Some("Old"));
        cyl.set_name("New");
        assert_eq!(cyl.name(), "New");
    }
}
