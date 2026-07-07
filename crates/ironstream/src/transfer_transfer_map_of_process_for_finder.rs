// FILE: transfer_transfer_map_of_process_for_finder.rs
// occt: Transfer_TransferMapOfProcessForFinder

use std::collections::HashMap;

/// Placeholder for Transfer_Finder handle
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TransferFinderMapHandle {
    id: usize,
}

impl TransferFinderMapHandle {
    fn new(id: usize) -> Self {
        TransferFinderMapHandle { id }
    }
}

/// Placeholder for Transfer_Binder handle
#[derive(Clone, Debug)]
pub struct TransferBinderMapHandle {
    // In real implementation, would contain reference-counted pointer to Transfer_Binder
}

/// Deprecated typedef: NCollection_IndexedDataMap<opencascade::handle<Transfer_Finder>,
///     opencascade::handle<Transfer_Binder>, Transfer_FindHasher>
///
/// An indexed data map mapping Transfer_Finder keys to Transfer_Binder values.
/// IndexedDataMap maintains insertion order and provides both key-based and index-based access.
/// Implements 1-based indexing as per OCCT semantics.
#[derive(Clone, Debug)]
pub struct TransferTransferMapOfProcessForFinder {
    // Maintain insertion order with a vec of keys and values
    pairs: Vec<(TransferFinderMapHandle, TransferBinderMapHandle)>,
    // For fast lookup
    key_map: HashMap<usize, usize>, // Maps finder id to index in pairs vec
}

impl TransferTransferMapOfProcessForFinder {
    /// Create an empty indexed data map.
    pub fn new() -> Self {
        TransferTransferMapOfProcessForFinder {
            pairs: Vec::new(),
            key_map: HashMap::new(),
        }
    }

    /// Create a map with initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        TransferTransferMapOfProcessForFinder {
            pairs: Vec::with_capacity(capacity),
            key_map: HashMap::with_capacity(capacity),
        }
    }

    /// Bind a key to a value. If key exists, update its value.
    pub fn bind(&mut self, key: TransferFinderMapHandle, value: TransferBinderMapHandle) {
        if let Some(&idx) = self.key_map.get(&key.id) {
            // Update existing
            self.pairs[idx].1 = value;
        } else {
            // Insert new
            self.key_map.insert(key.id, self.pairs.len());
            self.pairs.push((key, value));
        }
    }

    /// Find a value by key, returns Option.
    pub fn find(&self, key: &TransferFinderMapHandle) -> Option<&TransferBinderMapHandle> {
        if let Some(&idx) = self.key_map.get(&key.id) {
            Some(&self.pairs[idx].1)
        } else {
            None
        }
    }

    /// Find a mutable reference to value by key.
    pub fn change_find(
        &mut self,
        key: &TransferFinderMapHandle,
    ) -> Option<&mut TransferBinderMapHandle> {
        if let Some(&idx) = self.key_map.get(&key.id) {
            Some(&mut self.pairs[idx].1)
        } else {
            None
        }
    }

    /// Remove a key-value pair by key.
    pub fn remove(&mut self, key: &TransferFinderMapHandle) -> bool {
        if let Some(&idx) = self.key_map.get(&key.id) {
            // Remove from vec
            self.pairs.remove(idx);
            // Rebuild key_map to account for shifted indices
            self.key_map.clear();
            for (i, (k, _)) in self.pairs.iter().enumerate() {
                self.key_map.insert(k.id, i);
            }
            true
        } else {
            false
        }
    }

    /// Get value at position (1-based OCCT indexing).
    pub fn value(&self, index: i32) -> Option<&TransferBinderMapHandle> {
        let idx = (index - 1) as usize;
        self.pairs.get(idx).map(|(_, v)| v)
    }

    /// Get key at position (1-based OCCT indexing).
    pub fn key(&self, index: i32) -> Option<&TransferFinderMapHandle> {
        let idx = (index - 1) as usize;
        self.pairs.get(idx).map(|(k, _)| k)
    }

    /// Find index of key (1-based OCCT indexing, returns Option).
    pub fn find_index(&self, key: &TransferFinderMapHandle) -> Option<i32> {
        self.key_map
            .get(&key.id)
            .map(|&idx| (idx + 1) as i32)
    }

    /// Get the number of items.
    pub fn size(&self) -> usize {
        self.pairs.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Clear the map.
    pub fn clear(&mut self) {
        self.pairs.clear();
        self.key_map.clear();
    }

    /// Iterate over key-value pairs.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&TransferFinderMapHandle, &TransferBinderMapHandle)> {
        self.pairs.iter().map(|(k, v)| (k, v))
    }

    /// Mutable iteration.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&TransferFinderMapHandle, &mut TransferBinderMapHandle)> {
        self.pairs.iter_mut().map(|(k, v)| (&*k, v))
    }
}

impl Default for TransferTransferMapOfProcessForFinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_map() {
        let map = TransferTransferMapOfProcessForFinder::new();
        assert!(map.is_empty());
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        let key = TransferFinderMapHandle::new(1);
        let value = TransferBinderMapHandle {};

        map.bind(key.clone(), value);
        assert_eq!(map.size(), 1);
        assert!(map.find(&key).is_some());
    }

    #[test]
    fn test_bind_update_existing() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        let key = TransferFinderMapHandle::new(1);
        let value1 = TransferBinderMapHandle {};
        let value2 = TransferBinderMapHandle {};

        map.bind(key.clone(), value1);
        map.bind(key.clone(), value2);
        assert_eq!(map.size(), 1); // Should still be 1
    }

    #[test]
    fn test_find_nonexistent() {
        let map = TransferTransferMapOfProcessForFinder::new();
        let key = TransferFinderMapHandle::new(1);
        assert!(map.find(&key).is_none());
    }

    #[test]
    fn test_value_by_index() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        map.bind(TransferFinderMapHandle::new(1), TransferBinderMapHandle {});
        map.bind(TransferFinderMapHandle::new(2), TransferBinderMapHandle {});

        assert!(map.value(1).is_some());
        assert!(map.value(2).is_some());
        assert!(map.value(3).is_none());
    }

    #[test]
    fn test_key_by_index() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        let key1 = TransferFinderMapHandle::new(1);
        let key2 = TransferFinderMapHandle::new(2);
        map.bind(key1.clone(), TransferBinderMapHandle {});
        map.bind(key2.clone(), TransferBinderMapHandle {});

        assert_eq!(map.key(1).map(|k| k.id), Some(1));
        assert_eq!(map.key(2).map(|k| k.id), Some(2));
    }

    #[test]
    fn test_find_index() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        let key1 = TransferFinderMapHandle::new(1);
        let key2 = TransferFinderMapHandle::new(2);
        map.bind(key1.clone(), TransferBinderMapHandle {});
        map.bind(key2.clone(), TransferBinderMapHandle {});

        assert_eq!(map.find_index(&key1), Some(1));
        assert_eq!(map.find_index(&key2), Some(2));

        let key3 = TransferFinderMapHandle::new(3);
        assert_eq!(map.find_index(&key3), None);
    }

    #[test]
    fn test_remove() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        let key = TransferFinderMapHandle::new(1);
        map.bind(key.clone(), TransferBinderMapHandle {});
        assert_eq!(map.size(), 1);

        assert!(map.remove(&key));
        assert_eq!(map.size(), 0);
        assert!(map.find(&key).is_none());
    }

    #[test]
    fn test_clear() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        map.bind(TransferFinderMapHandle::new(1), TransferBinderMapHandle {});
        map.bind(TransferFinderMapHandle::new(2), TransferBinderMapHandle {});

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_iteration() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        map.bind(TransferFinderMapHandle::new(1), TransferBinderMapHandle {});
        map.bind(TransferFinderMapHandle::new(2), TransferBinderMapHandle {});

        let count = map.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_change_find() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        let key = TransferFinderMapHandle::new(1);
        map.bind(key.clone(), TransferBinderMapHandle {});

        assert!(map.change_find(&key).is_some());
    }

    #[test]
    fn test_1based_indexing() {
        let mut map = TransferTransferMapOfProcessForFinder::new();
        map.bind(TransferFinderMapHandle::new(10), TransferBinderMapHandle {});

        // Index 1 should give first element
        assert_eq!(map.key(1).map(|k| k.id), Some(10));
        // Index 0 and 2 should be out of bounds
        assert!(map.key(0).is_none());
        assert!(map.key(2).is_none());
    }
}
