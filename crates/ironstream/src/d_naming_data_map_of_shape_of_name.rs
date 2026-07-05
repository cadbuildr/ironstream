// FILE: d_naming_data_map_of_shape_of_name.rs
// occt: DNaming_DataMapOfShapeOfName

use std::collections::HashMap;

/// Data map for shape to name associations
pub struct DataMapOfShapeOfName {
    map: HashMap<String, String>,
}

impl DataMapOfShapeOfName {
    pub fn new() -> Self {
        DataMapOfShapeOfName { map: HashMap::new() }
    }

    pub fn insert(&mut self, shape_id: String, name: String) {
        self.map.insert(shape_id, name);
    }

    pub fn get(&self, shape_id: &str) -> Option<&String> {
        self.map.get(shape_id)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for DataMapOfShapeOfName {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut map = DataMapOfShapeOfName::new();
        map.insert("shape1".to_string(), "name1".to_string());
        assert_eq!(map.get("shape1"), Some(&"name1".to_string()));
    }

    #[test]
    fn test_len() {
        let mut map = DataMapOfShapeOfName::new();
        assert_eq!(map.len(), 0);
        map.insert("s1".to_string(), "n1".to_string());
        assert_eq!(map.len(), 1);
    }
}
