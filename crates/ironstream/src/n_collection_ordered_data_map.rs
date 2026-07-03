// FILE: n_collection_ordered_data_map.rs
// occt: NCollection_OrderedDataMap

/// Ordered map maintaining insertion order.
pub struct OrderedDataMap<K: Clone + Eq + std::hash::Hash, V: Clone> {
    map: std::collections::HashMap<K, V>,
    order: Vec<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> OrderedDataMap<K, V> {
    pub fn new() -> Self {
        OrderedDataMap {
            map: std::collections::HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        if !self.map.contains_key(&key) {
            self.order.push(key.clone());
        }
        self.map.insert(key, val);
    }

    pub fn find(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn keys_in_order(&self) -> &[K] {
        &self.order
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> Default for OrderedDataMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_data_map() {
        let mut map = OrderedDataMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        assert_eq!(map.len(), 3);
        assert_eq!(map.find(&"b"), Some(2));
    }
}
