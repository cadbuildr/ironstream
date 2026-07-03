// FILE: b_rep_graph_layer_registry.rs
// occt: BRepGraph_LayerRegistry

use std::collections::HashMap;

/// Dense GUID-keyed runtime registry of graph layers.
///
/// Stores registered layers in a compact vector for O(1) slot access and a
/// GUID-to-slot map for O(1) lookup by stable public identity.
pub struct BRepGraphLayerRegistry {
    layers: Vec<(String, Vec<u8>)>,
    guid_to_index: HashMap<String, usize>,
}

impl BRepGraphLayerRegistry {
    /// Create a new empty layer registry.
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            guid_to_index: HashMap::new(),
        }
    }

    /// Register a layer. Returns slot index.
    pub fn register_layer(&mut self, guid: String, data: Vec<u8>) -> u32 {
        if let Some(&idx) = self.guid_to_index.get(&guid) {
            self.layers[idx].1 = data;
            idx as u32
        } else {
            let idx = self.layers.len();
            self.layers.push((guid.clone(), data));
            self.guid_to_index.insert(guid, idx);
            idx as u32
        }
    }

    /// Remove a layer by GUID.
    pub fn unregister_layer(&mut self, guid: &str) {
        if let Some(idx) = self.guid_to_index.remove(guid) {
            if idx < self.layers.len() {
                self.layers.remove(idx);
                // Update indices in the map
                for entry in self.guid_to_index.iter_mut() {
                    if *entry.1 > idx {
                        *entry.1 -= 1;
                    }
                }
            }
        }
    }

    /// Find a layer by GUID.
    pub fn find_layer(&self, guid: &str) -> Option<&[u8]> {
        self.guid_to_index
            .get(guid)
            .and_then(|&idx| self.layers.get(idx).map(|(_, data)| data.as_slice()))
    }

    /// Get the number of registered layers.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Clear all layers.
    pub fn clear(&mut self) {
        self.layers.clear();
        self.guid_to_index.clear();
    }

    /// Iterate over layer GUIDs.
    pub fn iter_guids(&self) -> impl Iterator<Item = &str> {
        self.layers.iter().map(|(guid, _)| guid.as_str())
    }
}

impl Default for BRepGraphLayerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_registry() {
        let reg = BRepGraphLayerRegistry::new();
        assert_eq!(reg.layer_count(), 0);
    }

    #[test]
    fn test_register_layer() {
        let mut reg = BRepGraphLayerRegistry::new();
        let idx = reg.register_layer("guid1".to_string(), vec![1, 2, 3]);
        assert_eq!(idx, 0);
        assert_eq!(reg.layer_count(), 1);
    }

    #[test]
    fn test_find_layer() {
        let mut reg = BRepGraphLayerRegistry::new();
        reg.register_layer("guid1".to_string(), vec![1, 2, 3]);

        assert_eq!(reg.find_layer("guid1"), Some(&[1, 2, 3][..]));
        assert_eq!(reg.find_layer("guid2"), None);
    }

    #[test]
    fn test_unregister_layer() {
        let mut reg = BRepGraphLayerRegistry::new();
        reg.register_layer("guid1".to_string(), vec![1, 2, 3]);
        reg.unregister_layer("guid1");

        assert_eq!(reg.layer_count(), 0);
        assert_eq!(reg.find_layer("guid1"), None);
    }

    #[test]
    fn test_replace_layer() {
        let mut reg = BRepGraphLayerRegistry::new();
        reg.register_layer("guid1".to_string(), vec![1, 2, 3]);
        reg.register_layer("guid1".to_string(), vec![4, 5, 6]);

        assert_eq!(reg.layer_count(), 1);
        assert_eq!(reg.find_layer("guid1"), Some(&[4, 5, 6][..]));
    }
}
