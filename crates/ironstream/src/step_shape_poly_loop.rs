// FILE: step_shape_poly_loop.rs
// occt: StepShape_PolyLoop

use std::sync::Arc;

/// Placeholder for StepGeom_CartesianPoint
#[derive(Clone, Debug)]
pub struct CartesianPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl CartesianPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        CartesianPoint { x, y, z }
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

/// Placeholder for StepShape_Loop base class
pub struct Loop {
    name: Arc<str>,
}

/// Represents a polygonal loop in STEP format.
/// Inherits from StepShape_Loop.
pub struct PolyLoop {
    name: Arc<str>,
    polygon: Vec<Arc<CartesianPoint>>,
}

impl PolyLoop {
    /// Create a new PolyLoop
    pub fn new() -> Self {
        PolyLoop {
            name: Arc::from(""),
            polygon: Vec::new(),
        }
    }

    /// Initialize with name and polygon points
    pub fn init(&mut self, name: Arc<str>, polygon: Vec<Arc<CartesianPoint>>) {
        self.name = name;
        self.polygon = polygon;
    }

    /// Set the polygon points
    pub fn set_polygon(&mut self, polygon: Vec<Arc<CartesianPoint>>) {
        self.polygon = polygon;
    }

    /// Get the polygon points
    pub fn polygon(&self) -> &[Arc<CartesianPoint>] {
        &self.polygon
    }

    /// Get a polygon point by index (1-based as per OCCT convention)
    pub fn polygon_value(&self, num: usize) -> Option<Arc<CartesianPoint>> {
        if num > 0 && num <= self.polygon.len() {
            Some(self.polygon[num - 1].clone())
        } else {
            None
        }
    }

    /// Get the number of polygon points
    pub fn nb_polygon(&self) -> usize {
        self.polygon.len()
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

impl Default for PolyLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_loop_creation() {
        let pl = PolyLoop::new();
        assert_eq!(pl.name(), "");
        assert_eq!(pl.nb_polygon(), 0);
    }

    #[test]
    fn test_init_method() {
        let mut pl = PolyLoop::new();
        let points = vec![
            Arc::new(CartesianPoint::new(0.0, 0.0, 0.0)),
            Arc::new(CartesianPoint::new(1.0, 0.0, 0.0)),
            Arc::new(CartesianPoint::new(1.0, 1.0, 0.0)),
        ];
        let name: Arc<str> = Arc::from("triangle");

        pl.init(name.clone(), points);

        assert_eq!(pl.name(), "triangle");
        assert_eq!(pl.nb_polygon(), 3);
    }

    #[test]
    fn test_set_polygon() {
        let mut pl = PolyLoop::new();
        let points = vec![
            Arc::new(CartesianPoint::new(0.0, 0.0, 0.0)),
            Arc::new(CartesianPoint::new(1.0, 0.0, 0.0)),
        ];

        pl.set_polygon(points);
        assert_eq!(pl.nb_polygon(), 2);
    }

    #[test]
    fn test_polygon_value() {
        let mut pl = PolyLoop::new();
        let points = vec![
            Arc::new(CartesianPoint::new(1.0, 2.0, 3.0)),
            Arc::new(CartesianPoint::new(4.0, 5.0, 6.0)),
        ];

        pl.set_polygon(points);

        // 1-based indexing
        let pt1 = pl.polygon_value(1);
        assert!(pt1.is_some());
        let p1 = pt1.unwrap();
        assert_eq!(p1.x(), 1.0);
        assert_eq!(p1.y(), 2.0);
        assert_eq!(p1.z(), 3.0);

        // Out of bounds
        let pt_out = pl.polygon_value(3);
        assert!(pt_out.is_none());
    }
}
