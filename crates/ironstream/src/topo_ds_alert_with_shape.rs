// FILE: topo_ds_alert_with_shape.rs
// occt: TopoDS_AlertWithShape

// Alert object storing a topological shape, mirroring OCCT's
// TopoDS_AlertWithShape (a Message_Alert that carries a TopoDS_Shape).

/// Local helper: shape type enumeration (subset of TopAbs_ShapeEnum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Compound,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
}

/// Local helper: simplified topological shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    shape_type: Option<ShapeType>,
}

impl Shape {
    /// Creates a shape of the given type.
    pub fn new(shape_type: ShapeType) -> Self {
        Shape {
            shape_type: Some(shape_type),
        }
    }

    /// Creates a null shape.
    pub fn null() -> Self {
        Shape { shape_type: None }
    }

    /// Returns true if the shape is null.
    pub fn is_null(&self) -> bool {
        self.shape_type.is_none()
    }

    /// Returns the shape type, if any.
    pub fn shape_type(&self) -> Option<ShapeType> {
        self.shape_type
    }
}

/// Alert with an associated shape.
/// Mirrors OCCT TopoDS_AlertWithShape.
#[derive(Clone, Debug)]
pub struct TopoDsAlertWithShape {
    shape: Shape,
}

impl TopoDsAlertWithShape {
    /// Constructor with shape argument.
    pub fn new(shape: Shape) -> Self {
        TopoDsAlertWithShape { shape }
    }

    /// Returns the contained shape.
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    /// Sets the shape.
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

    /// Returns false (alerts with shapes cannot be merged), as in OCCT.
    pub fn supports_merge(&self) -> bool {
        false
    }

    /// Returns false: merge is not supported, as in OCCT.
    pub fn merge(&mut self, _target: &TopoDsAlertWithShape) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_with_shape_new() {
        let shape = Shape::new(ShapeType::Face);
        let alert = TopoDsAlertWithShape::new(shape.clone());
        assert_eq!(alert.get_shape(), &shape);
        assert!(!alert.get_shape().is_null());
        assert_eq!(alert.get_shape().shape_type(), Some(ShapeType::Face));
    }

    #[test]
    fn test_alert_with_null_shape() {
        let alert = TopoDsAlertWithShape::new(Shape::null());
        assert!(alert.get_shape().is_null());
    }

    #[test]
    fn test_alert_set_shape() {
        let mut alert = TopoDsAlertWithShape::new(Shape::new(ShapeType::Edge));
        alert.set_shape(Shape::new(ShapeType::Vertex));
        assert_eq!(alert.get_shape().shape_type(), Some(ShapeType::Vertex));
    }

    #[test]
    fn test_alert_merge_not_supported() {
        let mut alert = TopoDsAlertWithShape::new(Shape::new(ShapeType::Solid));
        let other = TopoDsAlertWithShape::new(Shape::new(ShapeType::Solid));
        assert!(!alert.supports_merge());
        assert!(!alert.merge(&other));
        // Merge must not change the stored shape.
        assert_eq!(alert.get_shape().shape_type(), Some(ShapeType::Solid));
    }
}
