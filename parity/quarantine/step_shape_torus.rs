// FILE: step_shape_torus.rs
// occt: StepShape_Torus

use std::sync::Arc;

/// Placeholder for StepGeom_Axis1Placement
pub struct Axis1Placement {
    x: f64,
    y: f64,
    z: f64,
}

impl Axis1Placement {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Axis1Placement { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

/// Represents a torus in STEP format.
/// Inherits from StepGeom_GeometricRepresentationItem.
pub struct Torus {
    name: Arc<str>,
    position: Option<Arc<Axis1Placement>>,
    major_radius: f64,
    minor_radius: f64,
}

impl Torus {
    /// Create a new Torus
    pub fn new() -> Self {
        Torus {
            name: Arc::from(""),
            position: None,
            major_radius: 0.0,
            minor_radius: 0.0,
        }
    }

    /// Initialize with name, position, and radii
    pub fn init(
        &mut self,
        name: Arc<str>,
        position: Arc<Axis1Placement>,
        major_radius: f64,
        minor_radius: f64,
    ) {
        self.name = name;
        self.position = Some(position);
        self.major_radius = major_radius;
        self.minor_radius = minor_radius;
    }

    /// Set the position (placement)
    pub fn set_position(&mut self, position: Arc<Axis1Placement>) {
        self.position = Some(position);
    }

    /// Get the position
    pub fn position(&self) -> Option<&Arc<Axis1Placement>> {
        self.position.as_ref()
    }

    /// Set the major radius
    pub fn set_major_radius(&mut self, major_radius: f64) {
        self.major_radius = major_radius;
    }

    /// Get the major radius
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Set the minor radius
    pub fn set_minor_radius(&mut self, minor_radius: f64) {
        self.minor_radius = minor_radius;
    }

    /// Get the minor radius
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for Torus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torus_creation() {
        let torus = Torus::new();
        assert_eq!(torus.name(), "");
        assert_eq!(torus.major_radius(), 0.0);
        assert_eq!(torus.minor_radius(), 0.0);
    }

    #[test]
    fn test_init_method() {
        let mut torus = Torus::new();
        let position = Arc::new(Axis1Placement::new(0.0, 0.0, 0.0));
        let name = Arc::from("torus_1");

        torus.init(name.clone(), position.clone(), 50.0, 20.0);

        assert_eq!(torus.name(), "torus_1");
        assert_eq!(torus.major_radius(), 50.0);
        assert_eq!(torus.minor_radius(), 20.0);
        assert!(torus.position().is_some());
    }

    #[test]
    fn test_set_position() {
        let mut torus = Torus::new();
        let position = Arc::new(Axis1Placement::new(5.0, 10.0, 15.0));

        torus.set_position(position);

        assert!(torus.position().is_some());
        let pos = torus.position().unwrap();
        assert_eq!(pos.x(), 5.0);
        assert_eq!(pos.y(), 10.0);
        assert_eq!(pos.z(), 15.0);
    }

    #[test]
    fn test_set_major_radius() {
        let mut torus = Torus::new();
        torus.set_major_radius(75.5);

        assert_eq!(torus.major_radius(), 75.5);
    }

    #[test]
    fn test_set_minor_radius() {
        let mut torus = Torus::new();
        torus.set_minor_radius(25.25);

        assert_eq!(torus.minor_radius(), 25.25);
    }

    #[test]
    fn test_multiple_operations() {
        let mut torus = Torus::new();
        torus.set_name(Arc::from("custom_torus"));
        torus.set_major_radius(100.0);
        torus.set_minor_radius(30.0);

        let position = Arc::new(Axis1Placement::new(1.0, 2.0, 3.0));
        torus.set_position(position);

        assert_eq!(torus.name(), "custom_torus");
        assert_eq!(torus.major_radius(), 100.0);
        assert_eq!(torus.minor_radius(), 30.0);
        assert!(torus.position().is_some());
    }
}
