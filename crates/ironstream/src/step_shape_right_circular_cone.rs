// FILE: step_shape_right_circular_cone.rs
// occt: StepShape_RightCircularCone

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

/// Represents a right circular cone in STEP format.
/// Inherits from StepGeom_GeometricRepresentationItem.
pub struct RightCircularCone {
    name: Arc<str>,
    position: Option<Arc<Axis1Placement>>,
    height: f64,
    radius: f64,
    semi_angle: f64,
}

impl RightCircularCone {
    /// Create a new RightCircularCone
    pub fn new() -> Self {
        RightCircularCone {
            name: Arc::from(""),
            position: None,
            height: 0.0,
            radius: 0.0,
            semi_angle: 0.0,
        }
    }

    /// Initialize with name, position, height, radius, and semi-angle
    pub fn init(
        &mut self,
        name: Arc<str>,
        position: Arc<Axis1Placement>,
        height: f64,
        radius: f64,
        semi_angle: f64,
    ) {
        self.name = name;
        self.position = Some(position);
        self.height = height;
        self.radius = radius;
        self.semi_angle = semi_angle;
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

    /// Set the semi-angle (half of the apex angle)
    pub fn set_semi_angle(&mut self, semi_angle: f64) {
        self.semi_angle = semi_angle;
    }

    /// Get the semi-angle
    pub fn semi_angle(&self) -> f64 {
        self.semi_angle
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

impl Default for RightCircularCone {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_right_circular_cone_creation() {
        let rcc = RightCircularCone::new();
        assert_eq!(rcc.name(), "");
        assert_eq!(rcc.height(), 0.0);
        assert_eq!(rcc.radius(), 0.0);
        assert_eq!(rcc.semi_angle(), 0.0);
    }

    #[test]
    fn test_init_method() {
        let mut rcc = RightCircularCone::new();
        let position = Arc::new(Axis1Placement::new(0.0, 0.0, 0.0));
        let name: Arc<str> = Arc::from("cone_1");

        rcc.init(name.clone(), position.clone(), 100.0, 50.0, PI / 6.0);

        assert_eq!(rcc.name(), "cone_1");
        assert_eq!(rcc.height(), 100.0);
        assert_eq!(rcc.radius(), 50.0);
        assert!((rcc.semi_angle() - PI / 6.0).abs() < 1e-10);
        assert!(rcc.position().is_some());
    }

    #[test]
    fn test_set_position() {
        let mut rcc = RightCircularCone::new();
        let position = Arc::new(Axis1Placement::new(1.0, 2.0, 3.0));

        rcc.set_position(position);

        assert!(rcc.position().is_some());
        let pos = rcc.position().unwrap();
        assert_eq!(pos.x(), 1.0);
        assert_eq!(pos.y(), 2.0);
        assert_eq!(pos.z(), 3.0);
    }

    #[test]
    fn test_set_height() {
        let mut rcc = RightCircularCone::new();
        rcc.set_height(75.5);

        assert_eq!(rcc.height(), 75.5);
    }

    #[test]
    fn test_set_radius() {
        let mut rcc = RightCircularCone::new();
        rcc.set_radius(25.25);

        assert_eq!(rcc.radius(), 25.25);
    }

    #[test]
    fn test_set_semi_angle() {
        let mut rcc = RightCircularCone::new();
        let angle = PI / 4.0;
        rcc.set_semi_angle(angle);

        assert_eq!(rcc.semi_angle(), angle);
    }
}
