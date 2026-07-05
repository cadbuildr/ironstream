// FILE: mesh_vs_data_map_of_integer_ascii_string.rs
// occt: MeshVS_DataMapOfIntegerAsciiString

use std::collections::BTreeMap;

pub struct MeshVSDataMapOfIntegerAsciiString {
    items: BTreeMap<i32, String>,
}

impl MeshVSDataMapOfIntegerAsciiString {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, value: String) {
        self.items.insert(key, value);
    }

    pub fn find(&self, key: i32) -> Option<String> {
        self.items.get(&key).cloned()
    }

    pub fn unbind(&mut self, key: i32) -> bool {
        self.items.remove(&key).is_some()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for MeshVSDataMapOfIntegerAsciiString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = MeshVSDataMapOfIntegerAsciiString::new();
        map.bind(1, "hello".to_string());
        assert_eq!(map.find(1), Some("hello".to_string()));
        assert!(map.unbind(1));
        assert_eq!(map.find(1), None);
    }
}
