// FILE: b_rep_graph_ui_ds_view.rs
// occt: BRepGraph_UIDsView

use std::collections::HashMap;

/// View for accessing UIDs in the graph.
pub struct BRepGraphUIDsView {
    graph_id: usize,
    uid_map: HashMap<u32, u64>,     // item_id -> uid
    reverse_map: HashMap<u64, u32>, // uid -> item_id
}

impl BRepGraphUIDsView {
    /// Create a new UIDs view.
    pub fn new(graph_id: usize) -> Self {
        Self {
            graph_id,
            uid_map: HashMap::new(),
            reverse_map: HashMap::new(),
        }
    }

    /// Get the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Add a UID mapping.
    pub fn add_uid(&mut self, item_id: u32, uid: u64) {
        if let Some(old_uid) = self.uid_map.get(&item_id) {
            self.reverse_map.remove(old_uid);
        }
        self.uid_map.insert(item_id, uid);
        self.reverse_map.insert(uid, item_id);
    }

    /// Get UID for an item.
    pub fn get_uid(&self, item_id: u32) -> Option<u64> {
        self.uid_map.get(&item_id).copied()
    }

    /// Get item id for a UID.
    pub fn get_item_id(&self, uid: u64) -> Option<u32> {
        self.reverse_map.get(&uid).copied()
    }

    /// Remove a UID mapping.
    pub fn remove_uid(&mut self, item_id: u32) {
        if let Some(uid) = self.uid_map.remove(&item_id) {
            self.reverse_map.remove(&uid);
        }
    }

    /// Get the number of UIDs.
    pub fn uid_count(&self) -> usize {
        self.uid_map.len()
    }

    /// Clear all UIDs.
    pub fn clear(&mut self) {
        self.uid_map.clear();
        self.reverse_map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let view = BRepGraphUIDsView::new(1);
        assert_eq!(view.graph_id(), 1);
        assert_eq!(view.uid_count(), 0);
    }

    #[test]
    fn test_add_uid() {
        let mut view = BRepGraphUIDsView::new(1);
        view.add_uid(1, 100);

        assert_eq!(view.get_uid(1), Some(100));
        assert_eq!(view.get_item_id(100), Some(1));
        assert_eq!(view.uid_count(), 1);
    }

    #[test]
    fn test_remove_uid() {
        let mut view = BRepGraphUIDsView::new(1);
        view.add_uid(1, 100);
        view.remove_uid(1);

        assert_eq!(view.get_uid(1), None);
        assert_eq!(view.get_item_id(100), None);
        assert_eq!(view.uid_count(), 0);
    }

    #[test]
    fn test_replace_uid() {
        let mut view = BRepGraphUIDsView::new(1);
        view.add_uid(1, 100);
        view.add_uid(1, 200);

        assert_eq!(view.get_uid(1), Some(200));
        assert_eq!(view.get_item_id(100), None);
        assert_eq!(view.get_item_id(200), Some(1));
    }

    #[test]
    fn test_clear() {
        let mut view = BRepGraphUIDsView::new(1);
        view.add_uid(1, 100);
        view.add_uid(2, 200);

        view.clear();
        assert_eq!(view.uid_count(), 0);
    }
}
