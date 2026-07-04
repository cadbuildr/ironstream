// FILE: step_shape_right_circular_cylinder.rs
// occt: StepShape_RightCircularCylinder

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

/// Represents a right circular cylinder in STEP format.
/// Inherits from StepGeom_GeometricRepresentationItem.
pub struct RightCircularCylinder {
    name: Arc<str>,
    position: Option<Arc<Axis1Placement>>,
    height: f64,
    radius: f64,
}

impl RightCircularCylinder {
    /// Create a new RightCircularCylinder
    pub fn new() -> Self {
        RightCircularCylinder {
            name: Arc::from(""),
            position: None,
            height: 0.0,
            radius: 0.0,
        }
    }

    /// Initialize with name, position, height, and radius
    pub fn init(
        &mut self,
        name: Arc<str>,
        position: Arc<Axis1Placement>,
        height: f64,
        radius: f64,
    ) {
        self.name = name;
        self.position = Some(position);
        self.height = height;
        self.radius = radius;
    }

    /// Set the position (placement)
    pub fn set_position(&mut self, position: Arc<Axis1Placement>) {
        self.position = Some(position);
    }

    /// Get the position
    pub fn position(&self) -> Option<&Arc<Axis1Placement>> {
        self.position.as_ref()
    }

    /// Set the height
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    /// Get the height
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Set the radius
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Get the radius
    pub fn radius(&self) -> f64 {
        self.radius
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

impl Default for RightCircularCylinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_circular_cylinder_creation() {
        let rcc = RightCircularCylinder::new();
        assert_eq!(rcc.name(), "");
        assert_eq!(rcc.height(), 0.0);
        assert_eq!(rcc.radius(), 0.0);
    }

    #[test]
    fn test_init_method() {
        let mut rcc = RightCircularCylinder::new();
        let position = Arc::new(Axis1Placement::new(0.0, 0.0, 0.0));
        let name = Arc::from("cylinder_1");

        rcc.init(name.clone(), position.clone(), 200.0, 50.0);

        assert_eq!(rcc.name(), "cylinder_1");
        assert_eq!(rcc.height(), 200.0);
        assert_eq!(rcc.radius(), 50.0);
        assert!(rcc.position().is_some());
    }

    #[test]
    fn test_set_position() {
        let mut rcc = RightCircularCylinder::new();
        let position = Arc::new(Axis1Placement::new(5.0, 10.0, 15.0));

        rcc.set_position(position);

        assert!(rcc.position().is_some());
        let pos = rcc.position().unwrap();
        assert_eq!(pos.x(), 5.0);
        assert_eq!(pos.y(), 10.0);
        assert_eq!(pos.z(), 15.0);
    }

    #[test]
    fn test_set_height() {
        let mut rcc = RightCircularCylinder::new();
        rcc.set_height(150.0);

        assert_eq!(rcc.height(), 150.0);
    }

    #[test]
    fn test_set_radius() {
        let mut rcc = RightCircularCylinder::new();
        rcc.set_radius(75.5);

        assert_eq!(rcc.radius(), 75.5);
    }

    #[test]
    fn test_multiple_updates() {
        let mut rcc = RightCircularCylinder::new();
        let position = Arc::new(Axis1Placement::new(1.0, 2.0, 3.0));

        rcc.set_position(position);
        rcc.set_height(100.0);
        rcc.set_radius(25.0);
        rcc.set_name(Arc::from("updated_cylinder"));

        assert_eq!(rcc.name(), "updated_cylinder");
        assert_eq!(rcc.height(), 100.0);
        assert_eq!(rcc.radius(), 25.0);
        assert!(rcc.position().is_some());
    }
}
