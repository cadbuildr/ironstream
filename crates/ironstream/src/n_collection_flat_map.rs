// FILE: n_collection_flat_map.rs
// occt: NCollection_FlatMap

/// Flat map with compact memory layout.
pub struct FlatMap<K: Clone + Ord, V: Clone> {
    entries: Vec<(K, V)>,
}

impl<K: Clone + Ord, V: Clone> FlatMap<K, V> {
    pub fn new() -> Self {
        FlatMap {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        for entry in &mut self.entries {
            if entry.0 == key {
                entry.1 = val;
                return;
            }
        }
        self.entries.push((key, val));
        self.entries.sort_by(|a, b| a.0.cmp(&b.0));
    }

    pub fn find(&self, key: &K) -> Option<V> {
        self.entries
            .binary_search_by(|entry| entry.0.cmp(key))
            .ok()
            .map(|idx| self.entries[idx].1.clone())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K: Clone + Ord, V: Clone> Default for FlatMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_map() {
        let mut map = FlatMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        assert_eq!(map.find(&1), Some("one"));
        assert_eq!(map.len(), 2);
    }
}
