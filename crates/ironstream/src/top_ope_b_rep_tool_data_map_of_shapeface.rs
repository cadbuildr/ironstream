// FILE: top_ope_b_rep_tool_data_map_of_shapeface.rs
// occt: TopOpeBRepTool_DataMapOfShapeface

use std::collections::HashMap;

/// ShapeKey: Shape identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
}

impl ShapeKey {
    pub fn new(id: usize) -> Self {
        ShapeKey { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Face: Face representation.
#[derive(Clone, Debug)]
pub struct Face {
    face_id: usize,
}

impl Face {
    pub fn new(face_id: usize) -> Self {
        Face { face_id }
    }

    pub fn face_id(&self) -> usize {
        self.face_id
    }
}

/// DataMapOfShapeface: Maps Shape to Face.
#[derive(Clone, Debug)]
pub struct DataMapOfShapeface {
    data: HashMap<ShapeKey, Face>,
}

impl DataMapOfShapeface {
    pub fn new() -> Self {
        DataMapOfShapeface {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, face: Face) -> bool {
        self.data.insert(shape, face).is_none()
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&Face> {
        self.data.get(shape)
    }

    pub fn find_mut(&mut self, shape: &ShapeKey) -> Option<&mut Face> {
        self.data.get_mut(shape)
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        self.data.remove(shape).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &Face)> {
        self.data.iter()
    }
}

impl Default for DataMapOfShapeface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_key() {
        let key = ShapeKey::new(42);
        assert_eq!(key.id(), 42);
    }

    #[test]
    fn test_face() {
        let face = Face::new(99);
        assert_eq!(face.face_id(), 99);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfShapeface::new();
        let shape = ShapeKey::new(5);
        let face = Face::new(50);
        assert!(map.bind(shape.clone(), face));
        assert!(!map.bind(shape, Face::new(51)));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfShapeface::new();
        let shape = ShapeKey::new(3);
        let face = Face::new(30);
        map.bind(shape.clone(), face);

        let found = map.find(&shape).unwrap();
        assert_eq!(found.face_id(), 30);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfShapeface::new();
        let shape = ShapeKey::new(7);
        map.bind(shape.clone(), Face::new(70));

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }
}
