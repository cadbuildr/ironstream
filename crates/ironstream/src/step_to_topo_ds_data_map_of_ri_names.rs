// FILE: step_to_topo_ds_data_map_of_ri_names.rs
// occt: StepToTopoDS_DataMapOfRINames

use std::collections::HashMap;

/// Deprecated typedef for NCollection_DataMap mapping Representation Item names.
pub struct StepToTopoDsDataMapOfRINames {
    data: HashMap<String, Option<String>>,
}

impl StepToTopoDsDataMapOfRINames {
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

impl Default for StepToTopoDsDataMapOfRINames {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_and_find() {
        let mut map = StepToTopoDsDataMapOfRINames::new();
        map.bind("name1".to_string(), Some("val1".to_string()));
        assert_eq!(map.find("name1"), Some(&Some("val1".to_string())));
    }

    #[test]
    fn test_size() {
        let mut map = StepToTopoDsDataMapOfRINames::new();
        map.bind("n".to_string(), Some("v".to_string()));
        assert_eq!(map.size(), 1);
    }
}
