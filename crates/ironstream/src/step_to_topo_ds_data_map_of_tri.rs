// FILE: step_to_topo_ds_data_map_of_tri.rs
// occt: StepToTopoDS_DataMapOfTRI

use std::collections::HashMap;

/// Deprecated typedef for NCollection_DataMap mapping Transient Representation Items.
pub struct StepToTopoDsDataMapOfTRI {
    data: HashMap<String, Option<String>>,
}

impl StepToTopoDsDataMapOfTRI {
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

impl Default for StepToTopoDsDataMapOfTRI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_and_find() {
        let mut map = StepToTopoDsDataMapOfTRI::new();
        map.bind("tri_key".to_string(), Some("tri_val".to_string()));
        assert_eq!(map.find("tri_key"), Some(&Some("tri_val".to_string())));
    }

    #[test]
    fn test_missing_key() {
        let map = StepToTopoDsDataMapOfTRI::new();
        assert_eq!(map.find("missing"), None);
    }
}
