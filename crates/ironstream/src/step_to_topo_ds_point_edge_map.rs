// FILE: step_to_topo_ds_point_edge_map.rs
// occt: StepToTopoDS_PointEdgeMap

use std::collections::HashMap;

/// Deprecated typedef for NCollection_DataMap mapping points to edges.
pub struct StepToTopoDsPointEdgeMap {
    data: HashMap<String, Option<String>>,
}

impl StepToTopoDsPointEdgeMap {
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

impl Default for StepToTopoDsPointEdgeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_edge_mapping() {
        let mut map = StepToTopoDsPointEdgeMap::new();
        map.bind("pt_0".to_string(), Some("edge_0".to_string()));
        assert_eq!(map.find("pt_0"), Some(&Some("edge_0".to_string())));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_clear() {
        let mut map = StepToTopoDsPointEdgeMap::new();
        map.bind("p".to_string(), Some("e".to_string()));
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
