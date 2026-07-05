// FILE: graphic3d_n_map_of_transient.rs
// occt: Graphic3d_NMapOfTransient

//! Deprecated: Use HashMap<usize, TransientObject> directly.
//! NCollection-based map for transient objects.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TransientObject {
    pub id: usize,
}

impl TransientObject {
    pub fn new(id: usize) -> Self {
        TransientObject { id }
    }
}

pub type Graphic3dNMapOfTransient = HashMap<usize, TransientObject>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: Graphic3dNMapOfTransient = HashMap::new();
        map.insert(1, TransientObject::new(100));

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_insert_retrieve() {
        let mut map: Graphic3dNMapOfTransient = HashMap::new();
        let obj = TransientObject::new(42);
        map.insert(1, obj);

        assert_eq!(map.get(&1).unwrap().id, 42);
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map: Graphic3dNMapOfTransient = HashMap::new();
        for i in 0..5 {
            map.insert(i, TransientObject::new(i * 10));
        }

        assert_eq!(map.len(), 5);
        assert_eq!(map[&4].id, 40);
    }

    #[test]
    fn test_map_iteration() {
        let mut map: Graphic3dNMapOfTransient = HashMap::new();
        map.insert(1, TransientObject::new(10));
        map.insert(2, TransientObject::new(20));

        let sum: usize = map.values().map(|obj| obj.id).sum();
        assert_eq!(sum, 30);
    }
}
