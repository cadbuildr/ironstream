// FILE: b_rep_graph_layer_lock.rs
// occt: BRepGraph_LayerLock

use std::collections::HashMap;

/// Scoped permission for an owner layer to edit one item it owns.
pub struct ScopedOwnerEdit {
    item_id: u32,
    is_active: bool,
}

impl ScopedOwnerEdit {
    /// Begin scoped owner edit.
    pub fn new(item_id: u32) -> Self {
        Self {
            item_id,
            is_active: true,
        }
    }

    /// Get the item id.
    pub fn item_id(&self) -> u32 {
        self.item_id
    }

    /// Check if the scope is active.
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Drop for ScopedOwnerEdit {
    fn drop(&mut self) {
        self.is_active = false;
    }
}

/// Owner metadata layer for owned BRepGraph items.
///
/// Uses a root-based ownership model: only the highest owned item per group
/// is stored in the map. All descendants receive the fast IsOwned bit-flag
/// via automatic downward propagation.
pub struct BRepGraphLayerLock {
    layer_id: String,
    owner_map: HashMap<u32, String>, // item_id -> owner_guid
    owned_flags: HashMap<u32, bool>, // item_id -> is_owned
}

impl BRepGraphLayerLock {
    /// Create lock-owner storage.
    pub fn new() -> Self {
        Self {
            layer_id: "BRepGraph_LayerLock".to_string(),
            owner_map: HashMap::new(),
            owned_flags: HashMap::new(),
        }
    }

    /// Return layer type GUID string.
    pub fn get_id() -> &'static str {
        "BRepGraph_LayerLock"
    }

    /// Return owner ID for an item.
    pub fn find_owner_id(&self, item_id: u32) -> Option<&str> {
        self.owner_map.get(&item_id).map(|s| s.as_str())
    }

    /// Set owner for an item.
    pub fn set_owner(&mut self, item_id: u32, owner_guid: String) {
        self.owner_map.insert(item_id, owner_guid);
        self.owned_flags.insert(item_id, true);
    }

    /// Check if an item is owned.
    pub fn has_owner(&self, item_id: u32) -> bool {
        *self.owned_flags.get(&item_id).unwrap_or(&false)
    }

    /// Remove ownership for an item.
    pub fn remove_owner(&mut self, item_id: u32) {
        self.owner_map.remove(&item_id);
        self.owned_flags.remove(&item_id);
    }

    /// Clear all ownership data.
    pub fn clear(&mut self) {
        self.owner_map.clear();
        self.owned_flags.clear();
    }
}

impl Default for BRepGraphLayerLock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoped_owner_edit() {
        let scope = ScopedOwnerEdit::new(1);
        assert_eq!(scope.item_id(), 1);
        assert!(scope.is_active());
    }

    #[test]
    fn test_layer_lock_new() {
        let layer = BRepGraphLayerLock::new();
        assert_eq!(layer.layer_id, "BRepGraph_LayerLock");
    }

    #[test]
    fn test_set_owner() {
        let mut layer = BRepGraphLayerLock::new();
        layer.set_owner(1, "owner_guid_1".to_string());

        assert!(layer.has_owner(1));
        assert_eq!(layer.find_owner_id(1), Some("owner_guid_1"));
    }

    #[test]
    fn test_remove_owner() {
        let mut layer = BRepGraphLayerLock::new();
        layer.set_owner(1, "owner_guid".to_string());
        layer.remove_owner(1);

        assert!(!layer.has_owner(1));
        assert_eq!(layer.find_owner_id(1), None);
    }

    #[test]
    fn test_clear() {
        let mut layer = BRepGraphLayerLock::new();
        layer.set_owner(1, "owner1".to_string());
        layer.set_owner(2, "owner2".to_string());

        layer.clear();
        assert!(!layer.has_owner(1));
        assert!(!layer.has_owner(2));
    }
}
