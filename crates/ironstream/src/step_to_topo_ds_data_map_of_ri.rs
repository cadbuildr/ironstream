// FILE: step_to_topo_ds_data_map_of_ri.rs
// occt: StepToTopoDS_DataMapOfRI

use std::collections::HashMap;

/// Deprecated typedef for NCollection_DataMap<Standard_Integer, TopoDS_Shape>.
/// Provides a hash-map-like container for mapping keys to Representation Items.
pub struct StepToTopoDsDataMapOfRI {
    data: HashMap<String, Option<String>>,
}

impl StepToTopoDsDataMapOfRI {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn bind(&mut self, key: String, value: Option<String>) {
        self.data.insert(key, value);
    }

    pub fn find(&self, key: &str) -> Option<&Option<String>> {
        self.data.get(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for StepToTopoDsDataMapOfRI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_and_find() {
        let mut map = StepToTopoDsDataMapOfRI::new();
        map.bind("key1".to_string(), Some("value1".to_string()));
        assert_eq!(map.find("key1"), Some(&Some("value1".to_string())));
    }

    #[test]
    fn test_size() {
        let mut map = StepToTopoDsDataMapOfRI::new();
        assert_eq!(map.size(), 0);
        map.bind("k1".to_string(), Some("v1".to_string()));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_clear() {
        let mut map = StepToTopoDsDataMapOfRI::new();
        map.bind("k".to_string(), Some("v".to_string()));
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
