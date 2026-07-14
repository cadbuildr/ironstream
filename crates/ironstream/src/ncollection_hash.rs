// FILE: ncollection_hash.rs
// occt-ref: NCollection_Hash, NCollection_Hashing, NCollection_HashSet
//       NCollection_HashMap

/// Hash function trait: abstract.
// occt-ref: NCollection_Hash
pub trait NCollectionHashable {
    fn hash(&self) -> u32;
}

impl NCollectionHashable for u32 {
    fn hash(&self) -> u32 { self ^ (self >> 16) }
}

impl NCollectionHashable for String {
    fn hash(&self) -> u32 {
        let mut h = 5381u32;
        for byte in self.as_bytes() {
            h = h.wrapping_mul(33).wrapping_add(*byte as u32);
        }
        h
    }
}

impl NCollectionHashable for [f64; 3] {
    fn hash(&self) -> u32 {
        let mut h = 5381u32;
        for &v in self {
            let bits = v.to_bits() as u32;
            h = h.wrapping_mul(33).wrapping_add(bits);
        }
        h
    }
}

/// Hash set: collision-chained hash table (stub).
// occt-ref: NCollection_HashSet
#[derive(Clone, Debug, Default)]
pub struct NCollectionHashSet<T: Clone + NCollectionHashable + PartialEq> {
    pub entries: Vec<T>,
    pub capacity: usize,
}

impl<T: Clone + NCollectionHashable + PartialEq> NCollectionHashSet<T> {
    pub fn new(capacity: usize) -> Self {
        Self { entries: Vec::new(), capacity: capacity.max(16) }
    }

    pub fn add(&mut self, item: T) {
        if !self.entries.contains(&item) { self.entries.push(item); }
    }

    pub fn contains(&self, item: &T) -> bool { self.entries.contains(item) }
    pub fn remove(&mut self, item: &T) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e != item);
        self.entries.len() < before
    }

    pub fn nb_entries(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn clear(&mut self) { self.entries.clear(); }
}

/// Hash map: key-value pairs (stub).
// occt-ref: NCollection_HashMap
#[derive(Clone, Debug)]
pub struct NCollectionHashMap<K: Clone + NCollectionHashable + PartialEq, V: Clone> {
    pub pairs: Vec<(K, V)>,
}

impl<K: Clone + NCollectionHashable + PartialEq, V: Clone> Default for NCollectionHashMap<K, V> {
    fn default() -> Self { Self { pairs: Vec::new() } }
}

impl<K: Clone + NCollectionHashable + PartialEq, V: Clone> NCollectionHashMap<K, V> {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(entry) = self.pairs.iter_mut().find(|(k, _)| k == &key) {
            entry.1 = value;
        } else {
            self.pairs.push((key, value));
        }
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        self.pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: &K) -> bool {
        let before = self.pairs.len();
        self.pairs.retain(|(k, _)| k != key);
        self.pairs.len() < before
    }

    pub fn nb_entries(&self) -> usize { self.pairs.len() }
    pub fn keys(&self) -> Vec<K> { self.pairs.iter().map(|(k, _)| k.clone()).collect() }
    pub fn values(&self) -> Vec<V> { self.pairs.iter().map(|(_, v)| v.clone()).collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_u32() {
        let h1 = 42u32.hash();
        let h2 = 42u32.hash();
        assert_eq!(h1, h2);
        assert_ne!(42u32.hash(), 43u32.hash());
    }

    #[test]
    fn hash_string() {
        assert_ne!("hello".to_string().hash(), "world".to_string().hash());
        assert_eq!("test".to_string().hash(), "test".to_string().hash());
    }

    #[test]
    fn hash_set_operations() {
        let mut hs: NCollectionHashSet<u32> = NCollectionHashSet::new(16);
        hs.add(1);
        hs.add(2);
        hs.add(1); // duplicate
        assert_eq!(hs.nb_entries(), 2);
        assert!(hs.contains(&1));
        hs.remove(&2);
        assert_eq!(hs.nb_entries(), 1);
    }

    #[test]
    fn hash_map_operations() {
        let mut hm: NCollectionHashMap<String, i32> = NCollectionHashMap::new();
        hm.insert("a".to_string(), 1);
        hm.insert("b".to_string(), 2);
        hm.insert("a".to_string(), 10); // override
        assert_eq!(hm.nb_entries(), 2);
        assert_eq!(hm.find(&"a".to_string()), Some(&10));
        hm.remove(&"a".to_string());
        assert_eq!(hm.nb_entries(), 1);
    }

    #[test]
    fn hash_map_keys_values() {
        let mut hm: NCollectionHashMap<u32, String> = NCollectionHashMap::new();
        hm.insert(1, "one".to_string());
        hm.insert(2, "two".to_string());
        assert_eq!(hm.keys(), vec![1, 2]);
        assert_eq!(hm.values(), vec!["one".to_string(), "two".to_string()]);
    }
}
