// FILE: b_rep_graph_layer.rs
// occt: BRepGraph_Layer

use std::collections::HashMap;

/// Abstract base class for named attribute layers.
///
/// A layer groups per-node and per-reference metadata under a unique name with
/// lifecycle callbacks. Layers are registered on BRepGraph and automatically
/// notified when nodes or references are removed, remapped (compact), or modified.
pub struct BRepGraphLayer {
    id: String,
    name: String,
    revision: u64,
    attached: bool,
    node_data: HashMap<u32, Vec<u8>>,
    ref_data: HashMap<u32, Vec<u8>>,
}

impl BRepGraphLayer {
    /// Create a new layer.
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            revision: 0,
            attached: false,
            node_data: HashMap::new(),
            ref_data: HashMap::new(),
        }
    }

    /// Layer identity (unique within a graph).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Layer type identity (unique within a graph).
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Called when a node is soft-removed without a replacement.
    pub fn on_node_removed(&mut self, node_id: u32) {
        self.node_data.remove(&node_id);
        self.touch();
    }

    /// Called when a node is soft-removed and replaced by another node.
    pub fn on_node_replaced(&mut self, old_node: u32, new_node: u32) {
        if let Some(data) = self.node_data.remove(&old_node) {
            self.node_data.insert(new_node, data);
        }
        self.touch();
    }

    /// Called when a reference is soft-deleted.
    pub fn on_ref_removed(&mut self, ref_id: u32) {
        self.ref_data.remove(&ref_id);
        self.touch();
    }

    /// Monotonic revision counter incremented on every state change.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Bump the revision counter.
    pub fn touch(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }

    /// Mark all cached values dirty (bulk invalidation).
    pub fn invalidate_all(&mut self) {
        self.touch();
    }

    /// Clear all stored data.
    pub fn clear(&mut self) {
        self.node_data.clear();
        self.ref_data.clear();
        self.touch();
    }

    /// True while this layer is registered in a live graph registry.
    pub fn is_attached(&self) -> bool {
        self.attached
    }

    /// Attach the layer to a graph.
    pub(crate) fn attach(&mut self) {
        self.attached = true;
        self.touch();
    }

    /// Detach the layer from a graph.
    pub(crate) fn detach(&mut self) {
        self.attached = false;
        self.touch();
    }

    /// Get node-specific data.
    pub fn get_node_data(&self, node_id: u32) -> Option<&[u8]> {
        self.node_data.get(&node_id).map(|v| v.as_slice())
    }

    /// Set node-specific data.
    pub fn set_node_data(&mut self, node_id: u32, data: Vec<u8>) {
        self.node_data.insert(node_id, data);
        self.touch();
    }

    /// Get reference-specific data.
    pub fn get_ref_data(&self, ref_id: u32) -> Option<&[u8]> {
        self.ref_data.get(&ref_id).map(|v| v.as_slice())
    }

    /// Set reference-specific data.
    pub fn set_ref_data(&mut self, ref_id: u32, data: Vec<u8>) {
        self.ref_data.insert(ref_id, data);
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_new() {
        let layer = BRepGraphLayer::new("layer_id".to_string(), "Layer Name".to_string());
        assert_eq!(layer.id(), "layer_id");
        assert_eq!(layer.name(), "Layer Name");
        assert_eq!(layer.revision(), 0);
        assert!(!layer.is_attached());
    }

    #[test]
    fn test_layer_attach() {
        let mut layer = BRepGraphLayer::new("id".to_string(), "name".to_string());
        layer.attach();
        assert!(layer.is_attached());
        assert_eq!(layer.revision(), 1);
    }

    #[test]
    fn test_node_data() {
        let mut layer = BRepGraphLayer::new("id".to_string(), "name".to_string());
        layer.set_node_data(1, vec![1, 2, 3]);

        assert_eq!(layer.get_node_data(1), Some(&[1, 2, 3][..]));
        assert_eq!(layer.get_node_data(2), None);
    }

    #[test]
    fn test_on_node_replaced() {
        let mut layer = BRepGraphLayer::new("id".to_string(), "name".to_string());
        layer.set_node_data(1, vec![1, 2, 3]);
        layer.on_node_replaced(1, 2);

        assert_eq!(layer.get_node_data(1), None);
        assert_eq!(layer.get_node_data(2), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn test_clear() {
        let mut layer = BRepGraphLayer::new("id".to_string(), "name".to_string());
        layer.set_node_data(1, vec![1, 2, 3]);
        layer.set_ref_data(1, vec![4, 5, 6]);

        layer.clear();
        assert_eq!(layer.get_node_data(1), None);
        assert_eq!(layer.get_ref_data(1), None);
    }
}
