// FILE: topo_ds_h_shape.rs
// occt: TopoDS_HShape

//! Handle to a topological shape for reference-counted memory management.

use std::sync::Arc;

use crate::topo_ds::Shape;

/// Reference-counted handle to a shape
pub type TopoDS_HShape = Arc<Shape>;

/// Creates a new handle to a shape
pub fn new_h_shape(shape: Shape) -> TopoDS_HShape {
    Arc::new(shape)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topo_ds::ShapeType;

    #[test]
    fn test_h_shape_new() {
        let shape = Shape::new(ShapeType::Vertex);
        let _handle = new_h_shape(shape);
    }

    #[test]
    fn test_h_shape_clone() {
        let shape = Shape::new(ShapeType::Face);
        let handle1 = new_h_shape(shape);
        let handle2 = Arc::clone(&handle1);
        assert_eq!(handle1.shape_type(), handle2.shape_type());
    }
}
