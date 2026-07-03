// FILE: n_collection_double_map.rs
// occt: NCollection_DoubleMap

/// Bidirectional map (key ↔ value both unique).
pub struct DoubleMap<K: Clone + Eq + std::hash::Hash, V: Clone + Eq + std::hash::Hash> {
    forward: std::collections::HashMap<K, V>,
    backward: std::collections::HashMap<V, K>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone + Eq + std::hash::Hash> DoubleMap<K, V> {
    pub fn new() -> Self {
        DoubleMap {
            forward: std::collections::HashMap::new(),
            backward: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        self.forward.insert(key.clone(), val.clone());
        self.backward.insert(val, key);
    }

    pub fn find1(&self, key: &K) -> Option<V> {
        self.forward.get(key).cloned()
    }

    pub fn find2(&self, val: &V) -> Option<K> {
        self.backward.get(val).cloned()
    }

    pub fn len(&self) -> usize {
        self.forward.len()
    }

    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone + Eq + std::hash::Hash> Default
    for DoubleMap<K, V>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_map() {
        let mut map = DoubleMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        assert_eq!(map.find1(&"a"), Some(1));
        assert_eq!(map.find2(&1), Some("a"));
        assert_eq!(map.len(), 2);
    }
}
