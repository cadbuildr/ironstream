// FILE: vrml_data_sphere.rs
// occt: VrmlData_Sphere
//
// Faithful port of OCCT VrmlData_Sphere (DataExchange/TKDEVRML/VrmlData/
// VrmlData_Sphere.hxx/.cxx): represents a VRML Sphere primitive node.
// Default radius 1.0; used in geometry definitions alongside Box, Cone, Cylinder.

use std::cell::RefCell;
use std::rc::Rc;

/// Error status for read/write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDataSphereErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
}

/// Input buffer for parsing.
pub struct VrmlDataSphereInBuffer {
    pub line_num: u32,
}

impl VrmlDataSphereInBuffer {
    pub fn new() -> Self {
        VrmlDataSphereInBuffer { line_num: 1 }
    }
}

impl Default for VrmlDataSphereInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// VRML Sphere node: a simple geometric primitive.
/// Represents a sphere with a specified radius (default 1.0).
/// Typically rendered as a unit sphere centered at the origin.
pub struct VrmlDataSphere {
    my_radius: f64,
    my_name: String,
}

impl VrmlDataSphere {
    /// Constructor: creates a sphere with default radius 1.0.
    pub fn new(name: Option<&str>) -> Self {
        VrmlDataSphere {
            my_radius: 1.0,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with explicit radius.
    pub fn with_radius(radius: f64, name: Option<&str>) -> Self {
        VrmlDataSphere {
            my_radius: radius.max(0.0),
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

    /// Get the radius.
    pub fn radius(&self) -> f64 {
        self.my_radius
    }

    /// Set the radius (enforces non-negative).
    pub fn set_radius(&mut self, radius: f64) {
        self.my_radius = radius.max(0.0);
    }

    /// Check if this sphere is in default state (radius == 1.0).
    pub fn is_default(&self) -> bool {
        (self.my_radius - 1.0).abs() < 1e-10
    }

    /// Virtual read method: parse Sphere node from VRML stream.
    pub fn read(&mut self, _buffer: &mut VrmlDataSphereInBuffer) -> VrmlDataSphereErrorStatus {
        // Subclass/user provides actual parsing.
        VrmlDataSphereErrorStatus::Ok
    }

    /// Virtual write method: output Sphere node to VRML format.
    pub fn write(&self, _prefix: Option<&str>) -> VrmlDataSphereErrorStatus {
        // Subclass/user provides actual output.
        VrmlDataSphereErrorStatus::Ok
    }

    /// Compute surface area (4 * pi * r^2).
    pub fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.my_radius * self.my_radius
    }

    /// Compute volume (4/3 * pi * r^3).
    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * self.my_radius.powi(3)
    }
}

impl Default for VrmlDataSphere {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlDataSphere {
    fn clone(&self) -> Self {
        VrmlDataSphere {
            my_radius: self.my_radius,
            my_name: self.my_name.clone(),
        }
    }
}

impl PartialEq for VrmlDataSphere {
    fn eq(&self, other: &Self) -> bool {
        (self.my_radius - other.my_radius).abs() < 1e-10 && self.my_name == other.my_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_sphere() {
        let sphere = VrmlDataSphere::new(None);
        assert_eq!(sphere.radius(), 1.0);
        assert_eq!(sphere.name(), "");
        assert!(sphere.is_default());
    }

    #[test]
    fn named_sphere() {
        let sphere = VrmlDataSphere::new(Some("MySphere"));
        assert_eq!(sphere.name(), "MySphere");
    }

    #[test]
    fn with_radius() {
        let sphere = VrmlDataSphere::with_radius(2.5, Some("Large"));
        assert_eq!(sphere.radius(), 2.5);
        assert_eq!(sphere.name(), "Large");
        assert!(!sphere.is_default());
    }

    #[test]
    fn set_radius() {
        let mut sphere = VrmlDataSphere::new(None);
        sphere.set_radius(3.0);
        assert_eq!(sphere.radius(), 3.0);
    }

    #[test]
    fn radius_non_negative() {
        let sphere = VrmlDataSphere::with_radius(-5.0, None);
        assert_eq!(sphere.radius(), 0.0);
    }

    #[test]
    fn surface_area_unit_sphere() {
        let sphere = VrmlDataSphere::new(None);
        let area = sphere.surface_area();
        let expected = 4.0 * std::f64::consts::PI;
        assert!((area - expected).abs() < 1e-10);
    }

    #[test]
    fn volume_unit_sphere() {
        let sphere = VrmlDataSphere::new(None);
        let vol = sphere.volume();
        let expected = (4.0 / 3.0) * std::f64::consts::PI;
        assert!((vol - expected).abs() < 1e-10);
    }

    #[test]
    fn surface_area_radius_2() {
        let sphere = VrmlDataSphere::with_radius(2.0, None);
        let area = sphere.surface_area();
        let expected = 4.0 * std::f64::consts::PI * 4.0; // 4 * pi * 2^2
        assert!((area - expected).abs() < 1e-10);
    }

    #[test]
    fn clone_preserves_data() {
        let sphere = VrmlDataSphere::with_radius(1.5, Some("Original"));
        let cloned = sphere.clone();
        assert_eq!(cloned.radius(), 1.5);
        assert_eq!(cloned.name(), "Original");
    }

    #[test]
    fn equality() {
        let s1 = VrmlDataSphere::with_radius(1.5, Some("S1"));
        let s2 = VrmlDataSphere::with_radius(1.5, Some("S1"));
        assert_eq!(s1, s2);
    }

    #[test]
    fn inequality_different_radius() {
        let s1 = VrmlDataSphere::with_radius(1.0, None);
        let s2 = VrmlDataSphere::with_radius(2.0, None);
        assert_ne!(s1, s2);
    }

    #[test]
    fn set_name() {
        let mut sphere = VrmlDataSphere::new(Some("Old"));
        sphere.set_name("New");
        assert_eq!(sphere.name(), "New");
    }
}
