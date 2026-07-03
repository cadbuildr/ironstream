// FILE: n_collection_flat_data_map.rs
// occt: NCollection_FlatDataMap

/// Flat map optimized for cache efficiency.
pub struct FlatDataMap<K: Clone + Eq + std::hash::Hash, V: Clone> {
    map: std::collections::HashMap<K, V>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> FlatDataMap<K, V> {
    pub fn new() -> Self {
        FlatDataMap {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        self.map.insert(key, val);
    }

    pub fn find(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> Default for FlatDataMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_data_map() {
        let mut map = FlatDataMap::new();
        map.insert("key", vec![1, 2, 3]);
        assert_eq!(map.find(&"key"), Some(vec![1, 2, 3]));
    }
}
