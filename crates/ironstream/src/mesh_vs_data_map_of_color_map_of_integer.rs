// FILE: mesh_vs_data_map_of_color_map_of_integer.rs
// occt: MeshVS_DataMapOfColorMapOfInteger

use std::collections::BTreeMap;

pub struct MeshVSDataMapOfColorMapOfInteger {
    items: BTreeMap<u32, BTreeMap<i32, i32>>,
}

impl MeshVSDataMapOfColorMapOfInteger {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, color: u32, value: BTreeMap<i32, i32>) {
        self.items.insert(color, value);
    }

    pub fn find(&self, color: u32) -> Option<BTreeMap<i32, i32>> {
        self.items.get(&color).cloned()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for MeshVSDataMapOfColorMapOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = MeshVSDataMapOfColorMapOfInteger::new();
        let mut inner = BTreeMap::new();
        inner.insert(1, 10);
        map.bind(0xFF0000, inner);
        assert_eq!(map.len(), 1);
    }
}
