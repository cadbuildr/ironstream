// FILE: b_rep_graph_layer_deferred.rs
// occt: BRepGraph_LayerDeferred

use std::collections::HashMap;

/// Deferred invalidation buffer for a layer.
pub struct BRepGraphLayerDeferred {
    modified_nodes: Vec<u32>,
    modified_refs: Vec<u32>,
    removed_items: Vec<u32>,
}

impl BRepGraphLayerDeferred {
    /// Create a new deferred invalidation buffer.
    pub fn new() -> Self {
        Self {
            modified_nodes: Vec::new(),
            modified_refs: Vec::new(),
            removed_items: Vec::new(),
        }
    }

    /// Add a modified node.
    pub fn add_modified_node(&mut self, node_id: u32) {
        if !self.modified_nodes.contains(&node_id) {
            self.modified_nodes.push(node_id);
        }
    }

    /// Add a modified reference.
    pub fn add_modified_ref(&mut self, ref_id: u32) {
        if !self.modified_refs.contains(&ref_id) {
            self.modified_refs.push(ref_id);
        }
    }

    /// Add a removed item.
    pub fn add_removed_item(&mut self, item_id: u32) {
        if !self.removed_items.contains(&item_id) {
            self.removed_items.push(item_id);
        }
    }

    /// Get modified nodes.
    pub fn modified_nodes(&self) -> &[u32] {
        &self.modified_nodes
    }

    /// Get modified references.
    pub fn modified_refs(&self) -> &[u32] {
        &self.modified_refs
    }

    /// Get removed items.
    pub fn removed_items(&self) -> &[u32] {
        &self.removed_items
    }

    /// Clear all buffers.
    pub fn clear(&mut self) {
        self.modified_nodes.clear();
        self.modified_refs.clear();
        self.removed_items.clear();
    }

    /// Check if any changes are pending.
    pub fn has_changes(&self) -> bool {
        !self.modified_nodes.is_empty()
            || !self.modified_refs.is_empty()
            || !self.removed_items.is_empty()
    }
}

impl Default for BRepGraphLayerDeferred {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buf = BRepGraphLayerDeferred::new();
        assert!(!buf.has_changes());
    }

    #[test]
    fn test_add_modified_node() {
        let mut buf = BRepGraphLayerDeferred::new();
        buf.add_modified_node(1);
        assert_eq!(buf.modified_nodes(), &[1]);
        assert!(buf.has_changes());
    }

    #[test]
    fn test_add_modified_ref() {
        let mut buf = BRepGraphLayerDeferred::new();
        buf.add_modified_ref(10);
        assert_eq!(buf.modified_refs(), &[10]);
        assert!(buf.has_changes());
    }

    #[test]
    fn test_no_duplicates() {
        let mut buf = BRepGraphLayerDeferred::new();
        buf.add_modified_node(1);
        buf.add_modified_node(1);
        assert_eq!(buf.modified_nodes().len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut buf = BRepGraphLayerDeferred::new();
        buf.add_modified_node(1);
        buf.add_modified_ref(10);
        buf.clear();
        assert!(!buf.has_changes());
    }
}
