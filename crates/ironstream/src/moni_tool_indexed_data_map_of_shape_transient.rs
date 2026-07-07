// FILE: moni_tool_indexed_data_map_of_shape_transient.rs
// occt: MoniTool_IndexedDataMapOfShapeTransient

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

/// NCollection_IndexedDataMap is a map with index-based access.
/// This implementation uses a vector of (key, value) pairs to maintain order.
pub struct NcollectionIndexedDataMapOfShapeTransient {
    entries: Vec<(TopoDsShape, StandardTransient)>,
}

impl NcollectionIndexedDataMapOfShapeTransient {
    pub fn new() -> Self {
        NcollectionIndexedDataMapOfShapeTransient {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, key: TopoDsShape, value: StandardTransient) -> i32 {
        for (idx, (existing_key, _)) in self.entries.iter().enumerate() {
            if *existing_key == key {
                self.entries[idx].1 = value;
                return (idx + 1) as i32;
            }
        }
        self.entries.push((key, value));
        self.entries.len() as i32
    }

    pub fn find_index(&self, key: &TopoDsShape) -> i32 {
        for (idx, (existing_key, _)) in self.entries.iter().enumerate() {
            if *existing_key == *key {
                return (idx + 1) as i32;
            }
        }
        0
    }

    pub fn find_key(&self, index: i32) -> Option<TopoDsShape> {
        if index > 0 && (index as usize) <= self.entries.len() {
            Some(self.entries[(index - 1) as usize].0.clone())
        } else {
            None
        }
    }

    pub fn find_value(&self, index: i32) -> Option<StandardTransient> {
        if index > 0 && (index as usize) <= self.entries.len() {
            Some(self.entries[(index - 1) as usize].1.clone())
        } else {
            None
        }
    }

    pub fn size(&self) -> i32 {
        self.entries.len() as i32
    }

    pub fn contains(&self, key: &TopoDsShape) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    pub fn remove(&mut self, key: &TopoDsShape) -> bool {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Deprecated typedef alias for backward compatibility.
pub type MoniToolIndexedDataMapOfShapeTransient = NcollectionIndexedDataMapOfShapeTransient;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map = NcollectionIndexedDataMapOfShapeTransient::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_map_add_and_find() {
        let mut map = NcollectionIndexedDataMapOfShapeTransient::new();

        let shape = TopoDsShape::new(1, ShapeType::Face);
        let value = Rc::new(RefCell::new("test_value".to_string()));

        let index = map.add(shape.clone(), value.clone());
        assert_eq!(index, 1);

        let found_index = map.find_index(&shape);
        assert_eq!(found_index, 1);

        let found_key = map.find_key(1).unwrap();
        assert_eq!(found_key, shape);

        let found_value = map.find_value(1).unwrap();
        assert_eq!(*found_value.borrow(), "test_value");
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map = NcollectionIndexedDataMapOfShapeTransient::new();

        let shape1 = TopoDsShape::new(1, ShapeType::Face);
        let shape2 = TopoDsShape::new(2, ShapeType::Edge);
        let shape3 = TopoDsShape::new(3, ShapeType::Vertex);

        let val1 = Rc::new(RefCell::new("val1".to_string()));
        let val2 = Rc::new(RefCell::new("val2".to_string()));
        let val3 = Rc::new(RefCell::new("val3".to_string()));

        map.add(shape1.clone(), val1.clone());
        map.add(shape2.clone(), val2.clone());
        map.add(shape3.clone(), val3.clone());

        assert_eq!(map.size(), 3);
        assert_eq!(map.find_index(&shape2), 2);
        assert_eq!(map.find_index(&shape3), 3);
    }

    #[test]
    fn test_map_contains() {
        let mut map = NcollectionIndexedDataMapOfShapeTransient::new();

        let shape = TopoDsShape::new(1, ShapeType::Solid);
        let value = Rc::new(RefCell::new("data".to_string()));

        assert!(!map.contains(&shape));
        map.add(shape.clone(), value);
        assert!(map.contains(&shape));
    }

    #[test]
    fn test_map_remove() {
        let mut map = NcollectionIndexedDataMapOfShapeTransient::new();

        let shape = TopoDsShape::new(1, ShapeType::Wire);
        let value = Rc::new(RefCell::new("data".to_string()));

        map.add(shape.clone(), value);
        assert_eq!(map.size(), 1);

        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
        assert!(!map.contains(&shape));
    }

    #[test]
    fn test_map_update_existing() {
        let mut map = NcollectionIndexedDataMapOfShapeTransient::new();

        let shape = TopoDsShape::new(1, ShapeType::Face);
        let val1 = Rc::new(RefCell::new("original".to_string()));
        let val2 = Rc::new(RefCell::new("updated".to_string()));

        let idx1 = map.add(shape.clone(), val1);
        let idx2 = map.add(shape.clone(), val2);

        assert_eq!(idx1, idx2);
        assert_eq!(map.size(), 1);

        let found = map.find_value(1).unwrap();
        assert_eq!(*found.borrow(), "updated");
    }

    #[test]
    fn test_map_out_of_bounds() {
        let map = NcollectionIndexedDataMapOfShapeTransient::new();

        assert_eq!(map.find_key(0), None);
        assert_eq!(map.find_key(1), None);
        assert_eq!(map.find_key(100), None);
        assert_eq!(map.find_value(0), None);
        assert_eq!(map.find_value(1), None);
    }
}
