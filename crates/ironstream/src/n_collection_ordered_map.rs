// FILE: n_collection_ordered_map.rs
// occt: NCollection_OrderedMap

/// Ordered set maintaining insertion order.
pub struct OrderedMap<K: Clone + Eq + std::hash::Hash> {
    map: std::collections::HashMap<K, ()>,
    order: Vec<K>,
}

impl<K: Clone + Eq + std::hash::Hash> OrderedMap<K> {
    pub fn new() -> Self {
        OrderedMap {
            map: std::collections::HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K) {
        if !self.map.contains_key(&key) {
            self.order.push(key.clone());
            self.map.insert(key, ());
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn keys_in_order(&self) -> &[K] {
        &self.order
    }
}

impl<K: Clone + Eq + std::hash::Hash> Default for OrderedMap<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_map() {
        let mut map = OrderedMap::new();
        map.insert("a");
        map.insert("b");
        assert_eq!(map.len(), 2);
        assert!(map.contains(&"a"));
    }
}
