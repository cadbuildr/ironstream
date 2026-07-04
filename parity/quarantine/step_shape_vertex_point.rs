// FILE: step_shape_vertex_point.rs
// occt: StepShape_VertexPoint

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

/// Placeholder for StepShape_Vertex base class
pub struct Vertex {
    name: Arc<str>,
}

impl Vertex {
    pub fn new(name: Arc<str>) -> Self {
        Vertex { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Represents a vertex point in STEP format.
/// Inherits from StepShape_Vertex.
pub struct VertexPoint {
    name: Arc<str>,
    vertex_geometry: Option<Arc<Point>>,
}

impl VertexPoint {
    /// Create a new VertexPoint
    pub fn new() -> Self {
        VertexPoint {
            name: Arc::from(""),
            vertex_geometry: None,
        }
    }

    /// Initialize with name and vertex geometry (point)
    pub fn init(&mut self, name: Arc<str>, vertex_geometry: Arc<Point>) {
        self.name = name;
        self.vertex_geometry = Some(vertex_geometry);
    }

    /// Set the vertex geometry (point)
    pub fn set_vertex_geometry(&mut self, vertex_geometry: Arc<Point>) {
        self.vertex_geometry = Some(vertex_geometry);
    }

    /// Get the vertex geometry (point)
    pub fn vertex_geometry(&self) -> Option<&Arc<Point>> {
        self.vertex_geometry.as_ref()
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

impl Default for VertexPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_point_creation() {
        let vp = VertexPoint::new();
        assert_eq!(vp.name(), "");
        assert!(vp.vertex_geometry().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut vp = VertexPoint::new();
        let geometry = Arc::new(Point::new(1.0, 2.0, 3.0));
        let name = Arc::from("vertex_point_1");

        vp.init(name.clone(), geometry.clone());

        assert_eq!(vp.name(), "vertex_point_1");
        assert!(vp.vertex_geometry().is_some());
        let pt = vp.vertex_geometry().unwrap();
        assert_eq!(pt.x(), 1.0);
        assert_eq!(pt.y(), 2.0);
        assert_eq!(pt.z(), 3.0);
    }

    #[test]
    fn test_set_vertex_geometry() {
        let mut vp = VertexPoint::new();
        let geometry = Arc::new(Point::new(10.0, 20.0, 30.0));

        vp.set_vertex_geometry(geometry);

        assert!(vp.vertex_geometry().is_some());
        let pt = vp.vertex_geometry().unwrap();
        assert_eq!(pt.x(), 10.0);
        assert_eq!(pt.y(), 20.0);
        assert_eq!(pt.z(), 30.0);
    }

    #[test]
    fn test_set_name() {
        let mut vp = VertexPoint::new();
        vp.set_name(Arc::from("custom_vertex"));

        assert_eq!(vp.name(), "custom_vertex");
    }

    #[test]
    fn test_full_initialization() {
        let mut vp = VertexPoint::new();
        let geometry = Arc::new(Point::new(5.0, 5.0, 5.0));
        let name = Arc::from("full_vertex_point");

        vp.init(name.clone(), geometry);

        assert_eq!(vp.name(), "full_vertex_point");
        assert!(vp.vertex_geometry().is_some());
        assert_eq!(vp.vertex_geometry().unwrap().x(), 5.0);
    }
}
