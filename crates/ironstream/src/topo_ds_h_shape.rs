// FILE: topo_ds_h_shape.rs
// occt: TopoDS_HShape

// Handle (reference-counted transient wrapper) around a topological shape,
// mirroring OCCT's TopoDS_HShape.

use std::cell::RefCell;
use std::rc::Rc;

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

/// Transient wrapper holding a shape, mirroring OCCT TopoDS_HShape.
#[derive(Debug)]
pub struct TopoDsHShape {
    shape: RefCell<Shape>,
}

impl TopoDsHShape {
    /// Constructs an empty (null) shape object.
    pub fn new() -> Self {
        TopoDsHShape {
            shape: RefCell::new(Shape::null()),
        }
    }

    /// Constructs a shape object defined by the given shape.
    pub fn with_shape(shape: Shape) -> Self {
        TopoDsHShape {
            shape: RefCell::new(shape),
        }
    }

    /// Loads this shape object with the given shape (`Shape(const TopoDS_Shape&)`).
    pub fn set_shape(&self, shape: Shape) {
        *self.shape.borrow_mut() = shape;
    }

    /// Returns a copy of the contained shape (`Shape() const`).
    pub fn shape(&self) -> Shape {
        self.shape.borrow().clone()
    }

    /// Modifies the contained shape in place (`ChangeShape()`).
    pub fn change_shape<F: FnOnce(&mut Shape)>(&self, f: F) {
        f(&mut self.shape.borrow_mut());
    }

    /// Returns the shape type of the contained shape.
    pub fn shape_type(&self) -> Option<ShapeType> {
        self.shape.borrow().shape_type()
    }
}

impl Default for TopoDsHShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Reference-counted handle to an HShape (occ::handle<TopoDS_HShape>).
pub type HShapeHandle = Rc<TopoDsHShape>;

/// Creates a new handle to a shape.
pub fn new_h_shape(shape: Shape) -> HShapeHandle {
    Rc::new(TopoDsHShape::with_shape(shape))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_shape_empty() {
        let h = TopoDsHShape::new();
        assert!(h.shape().is_null());
    }

    #[test]
    fn test_h_shape_new() {
        let handle = new_h_shape(Shape::new(ShapeType::Vertex));
        assert_eq!(handle.shape_type(), Some(ShapeType::Vertex));
        assert!(!handle.shape().is_null());
    }

    #[test]
    fn test_h_shape_clone_shares_shape() {
        let handle1 = new_h_shape(Shape::new(ShapeType::Face));
        let handle2 = Rc::clone(&handle1);
        assert_eq!(handle1.shape_type(), handle2.shape_type());

        // Loading a new shape through one handle is visible through the other,
        // since both reference the same underlying object.
        handle2.set_shape(Shape::new(ShapeType::Edge));
        assert_eq!(handle1.shape_type(), Some(ShapeType::Edge));
    }

    #[test]
    fn test_h_shape_change_shape() {
        let h = TopoDsHShape::with_shape(Shape::new(ShapeType::Wire));
        h.change_shape(|s| *s = Shape::new(ShapeType::Shell));
        assert_eq!(h.shape_type(), Some(ShapeType::Shell));
    }
}
