// FILE: b_rep_graph_data.rs
// occt: BRepGraph_Data

use std::collections::HashMap;

/// Internal storage for BRepGraph (PIMPL).
///
/// All topology definition data and UIDs live in incidence table storage.
/// Views provide access to this data.
pub struct BRepGraphData {
    /// Incidence-table storage - sole source of truth for all topology data,
    /// original shapes, TShape->NodeId mapping, UIDs, and UID reverse indexes.
    pub inc_storage: HashMap<String, Vec<u32>>,

    /// Registered graph layers.
    pub layer_registry: HashMap<u32, String>,

    /// Registered transient cache services.
    pub cache_registry: HashMap<u32, Vec<u8>>,
}

impl BRepGraphData {
    /// Create a new empty BRepGraph_Data.
    pub fn new() -> Self {
        Self {
            inc_storage: HashMap::new(),
            layer_registry: HashMap::new(),
            cache_registry: HashMap::new(),
        }
    }

    /// Clear all internal data.
    pub fn clear(&mut self) {
        self.inc_storage.clear();
        self.layer_registry.clear();
        self.cache_registry.clear();
    }

    /// Check if the storage is empty.
    pub fn is_empty(&self) -> bool {
        self.inc_storage.is_empty() && self.layer_registry.is_empty()
    }
}

impl Default for BRepGraphData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_data() {
        let data = BRepGraphData::new();
        assert!(data.is_empty());
    }

    #[test]
    fn test_clear_data() {
        let mut data = BRepGraphData::new();
        data.inc_storage.insert("test".to_string(), vec![1, 2, 3]);
        data.layer_registry.insert(1, "layer1".to_string());

        assert!(!data.is_empty());
        data.clear();
        assert!(data.is_empty());
    }

    #[test]
    fn test_default_data() {
        let data = BRepGraphData::default();
        assert!(data.is_empty());
    }
}
