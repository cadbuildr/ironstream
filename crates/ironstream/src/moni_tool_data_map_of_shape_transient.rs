// FILE: moni_tool_data_map_of_shape_transient.rs
// occt: MoniTool_DataMapOfShapeTransient
// occt-ref: MoniTool_DataMapIteratorOfDataMapOfShapeTransient

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

/// TopoDS_Shape represents a topological shape in OCC.
#[derive(Clone, Debug)]
pub struct TopoDsShape {
    id: i32,
    shape_type: ShapeType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
}

impl TopoDsShape {
    pub fn new(id: i32, shape_type: ShapeType) -> Self {
        TopoDsShape { id, shape_type }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }
}

impl PartialEq for TopoDsShape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.shape_type == other.shape_type
    }
}

impl Eq for TopoDsShape {}

impl Hash for TopoDsShape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        (self.shape_type as i32).hash(state);
    }
}

/// Standard_Transient is the base class for reference-counted objects.
pub type StandardTransient = Rc<RefCell<String>>;

/// TopTools_ShapeMapHasher is a custom hasher for shapes.
/// In Rust we use the built-in Hash trait, so this is implicitly handled.

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<TopoDS_Shape, opencascade::handle<Standard_Transient>, TopTools_ShapeMapHasher>`
pub type MoniToolDataMapOfShapeTransient = HashMap<TopoDsShape, StandardTransient>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<TopoDS_Shape, opencascade::handle<Standard_Transient>, TopTools_ShapeMapHasher>::Iterator`
pub type MoniToolDataMapIteratorOfDataMapOfShapeTransient =
    std::collections::hash_map::IntoIter<TopoDsShape, StandardTransient>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation() {
        let shape = TopoDsShape::new(1, ShapeType::Face);
        assert_eq!(shape.id(), 1);
        assert_eq!(shape.shape_type(), ShapeType::Face);
    }

    #[test]
    fn test_shape_equality() {
        let shape1 = TopoDsShape::new(1, ShapeType::Edge);
        let shape2 = TopoDsShape::new(1, ShapeType::Edge);
        let shape3 = TopoDsShape::new(2, ShapeType::Edge);

        assert_eq!(shape1, shape2);
        assert_ne!(shape1, shape3);
    }

    #[test]
    fn test_shape_hash() {
        let shape1 = TopoDsShape::new(1, ShapeType::Vertex);
        let shape2 = TopoDsShape::new(1, ShapeType::Vertex);

        let mut map = HashMap::new();
        map.insert(shape1, Rc::new(RefCell::new("test".to_string())));
        map.insert(shape2, Rc::new(RefCell::new("updated".to_string())));

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_transient_creation() {
        let transient = Rc::new(RefCell::new("transient_data".to_string()));
        assert_eq!(*transient.borrow(), "transient_data");
    }

    #[test]
    fn test_data_map_creation() {
        let map: MoniToolDataMapOfShapeTransient = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MoniToolDataMapOfShapeTransient = HashMap::new();

        let shape1 = TopoDsShape::new(1, ShapeType::Face);
        let shape2 = TopoDsShape::new(2, ShapeType::Edge);

        let transient1 = Rc::new(RefCell::new("data1".to_string()));
        let transient2 = Rc::new(RefCell::new("data2".to_string()));

        map.insert(shape1.clone(), transient1.clone());
        map.insert(shape2.clone(), transient2.clone());

        assert_eq!(map.get(&shape1), Some(&transient1));
        assert_eq!(map.get(&shape2), Some(&transient2));
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MoniToolDataMapOfShapeTransient = HashMap::new();
        assert_eq!(map.len(), 0);

        let shape1 = TopoDsShape::new(1, ShapeType::Wire);
        let shape2 = TopoDsShape::new(2, ShapeType::Solid);
        let transient = Rc::new(RefCell::new("test".to_string()));

        map.insert(shape1.clone(), transient.clone());
        map.insert(shape2, transient.clone());
        assert_eq!(map.len(), 2);

        map.remove(&shape1);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MoniToolDataMapOfShapeTransient = HashMap::new();

        for i in 0..3 {
            let shape = TopoDsShape::new(i as i32, ShapeType::Face);
            let transient = Rc::new(RefCell::new(format!("data{}", i)));
            map.insert(shape, transient);
        }

        let collected: Vec<(TopoDsShape, StandardTransient)> = map.into_iter().collect();
        assert_eq!(collected.len(), 3);
    }

    #[test]
    fn test_shape_types() {
        let vertex = TopoDsShape::new(1, ShapeType::Vertex);
        let edge = TopoDsShape::new(2, ShapeType::Edge);
        let face = TopoDsShape::new(3, ShapeType::Face);
        let solid = TopoDsShape::new(4, ShapeType::Solid);

        assert_eq!(vertex.shape_type(), ShapeType::Vertex);
        assert_eq!(edge.shape_type(), ShapeType::Edge);
        assert_eq!(face.shape_type(), ShapeType::Face);
        assert_eq!(solid.shape_type(), ShapeType::Solid);
    }
}
