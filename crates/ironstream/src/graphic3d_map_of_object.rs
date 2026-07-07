// FILE: graphic3d_map_of_object.rs
// occt: Graphic3d_MapOfObject

//! Deprecated: Use HashMap<usize, Object> directly.
//! Map of graphic 3D objects.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Object {
    pub id: usize,
}

impl Object {
    pub fn new(id: usize) -> Self {
        Object { id }
    }
}

pub type Graphic3dMapOfObject = HashMap<usize, Object>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: Graphic3dMapOfObject = HashMap::new();
        map.insert(1, Object::new(1));

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1).unwrap().id, 1);
    }

    #[test]
    fn test_map_operations() {
        let mut map: Graphic3dMapOfObject = HashMap::new();
        map.insert(10, Object::new(100));
        map.insert(20, Object::new(200));

        assert_eq!(map.len(), 2);
        assert_eq!(map[&10].id, 100);
    }

    #[test]
    fn test_map_iteration() {
        let mut map: Graphic3dMapOfObject = HashMap::new();
        map.insert(1, Object::new(10));
        map.insert(2, Object::new(20));

        let ids: Vec<usize> = map.values().map(|obj| obj.id).collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&10));
        assert!(ids.contains(&20));
    }
}
