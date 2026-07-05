// FILE: mesh_vs_data_map_of_h_array1_of_sequence_of_integer.rs
// occt: MeshVS_DataMapOfHArray1OfSequenceOfInteger

use std::collections::BTreeMap;

pub struct MeshVSDataMapOfHArray1OfSequenceOfInteger {
    items: BTreeMap<u32, Vec<Vec<i32>>>,
}

impl MeshVSDataMapOfHArray1OfSequenceOfInteger {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: u32, value: Vec<Vec<i32>>) {
        self.items.insert(key, value);
    }

    pub fn find(&self, key: u32) -> Option<Vec<Vec<i32>>> {
        self.items.get(&key).cloned()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for MeshVSDataMapOfHArray1OfSequenceOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = MeshVSDataMapOfHArray1OfSequenceOfInteger::new();
        map.bind(1, vec![vec![1, 2, 3]]);
        assert_eq!(map.len(), 1);
    }
}
