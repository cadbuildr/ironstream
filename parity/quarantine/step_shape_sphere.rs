// FILE: step_shape_sphere.rs
// occt: StepShape_Sphere

use std::sync::Arc;

/// Placeholder for StepGeom_Point
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
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

/// Represents a sphere in STEP format.
/// Inherits from StepGeom_GeometricRepresentationItem.
pub struct Sphere {
    name: Arc<str>,
    radius: f64,
    centre: Option<Arc<Point>>,
}

impl Sphere {
    /// Create a new Sphere
    pub fn new() -> Self {
        Sphere {
            name: Arc::from(""),
            radius: 0.0,
            centre: None,
        }
    }

    /// Initialize with name, radius, and centre point
    pub fn init(&mut self, name: Arc<str>, radius: f64, centre: Arc<Point>) {
        self.name = name;
        self.radius = radius;
        self.centre = Some(centre);
    }

    /// Set the radius
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Get the radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Set the centre point
    pub fn set_centre(&mut self, centre: Arc<Point>) {
        self.centre = Some(centre);
    }

    /// Get the centre point
    pub fn centre(&self) -> Option<&Arc<Point>> {
        self.centre.as_ref()
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

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_creation() {
        let sphere = Sphere::new();
        assert_eq!(sphere.name(), "");
        assert_eq!(sphere.radius(), 0.0);
        assert!(sphere.centre().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut sphere = Sphere::new();
        let centre = Arc::new(Point::new(0.0, 0.0, 0.0));
        let name = Arc::from("sphere_1");

        sphere.init(name.clone(), 25.0, centre.clone());

        assert_eq!(sphere.name(), "sphere_1");
        assert_eq!(sphere.radius(), 25.0);
        assert!(sphere.centre().is_some());
    }

    #[test]
    fn test_set_radius() {
        let mut sphere = Sphere::new();
        sphere.set_radius(50.5);

        assert_eq!(sphere.radius(), 50.5);
    }

    #[test]
    fn test_set_centre() {
        let mut sphere = Sphere::new();
        let centre = Arc::new(Point::new(10.0, 20.0, 30.0));

        sphere.set_centre(centre);

        assert!(sphere.centre().is_some());
        let c = sphere.centre().unwrap();
        assert_eq!(c.x(), 10.0);
        assert_eq!(c.y(), 20.0);
        assert_eq!(c.z(), 30.0);
    }

    #[test]
    fn test_multiple_operations() {
        let mut sphere = Sphere::new();
        sphere.set_name(Arc::from("big_sphere"));
        sphere.set_radius(100.0);

        let centre = Arc::new(Point::new(5.0, 5.0, 5.0));
        sphere.set_centre(centre);

        assert_eq!(sphere.name(), "big_sphere");
        assert_eq!(sphere.radius(), 100.0);
        assert!(sphere.centre().is_some());
    }
}
