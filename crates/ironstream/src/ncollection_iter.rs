// FILE: ncollection_iter.rs
// occt: NCollection_Iterator, NCollection_StlIterator,
//       NCollection_IndexedMap, NCollection_IndexedDataMap

use std::collections::HashMap;

// occt: NCollection_IndexedMap<KEY> — bijective map: index (1-based) <-> KEY
#[derive(Clone, Debug)]
pub struct IndexedMap<K: Clone + PartialEq + std::hash::Hash + Eq> {
    keys: Vec<K>,
    index: HashMap<K, usize>,  // key -> 1-based index
}

impl<K: Clone + PartialEq + std::hash::Hash + Eq> Default for IndexedMap<K> {
    fn default() -> Self { Self { keys: Vec::new(), index: HashMap::new() } }
}

impl<K: Clone + PartialEq + std::hash::Hash + Eq> IndexedMap<K> {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, key: K) -> usize {
        if let Some(&i) = self.index.get(&key) { return i; }
        self.keys.push(key.clone());
        let i = self.keys.len();
        self.index.insert(key, i);
        i
    }

    pub fn find_index(&self, key: &K) -> usize {
        self.index.get(key).copied().unwrap_or(0)
    }

    pub fn find_key(&self, i: usize) -> Option<&K> {
        if i == 0 || i > self.keys.len() { None } else { Some(&self.keys[i - 1]) }
    }

    pub fn contains(&self, key: &K) -> bool { self.index.contains_key(key) }
    pub fn size(&self) -> usize { self.keys.len() }
    pub fn is_empty(&self) -> bool { self.keys.is_empty() }

    pub fn remove_last(&mut self) {
        if let Some(k) = self.keys.pop() {
            self.index.remove(&k);
        }
    }

    pub fn clear(&mut self) { self.keys.clear(); self.index.clear(); }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &K)> {
        self.keys.iter().enumerate().map(|(i, k)| (i + 1, k))
    }
}

// occt: NCollection_IndexedDataMap<KEY, VAL> — indexed map with associated data
#[derive(Clone, Debug)]
pub struct IndexedDataMap<K, V>
where K: Clone + PartialEq + std::hash::Hash + Eq, V: Clone,
{
    keys: Vec<K>,
    vals: Vec<V>,
    index: HashMap<K, usize>,
}

impl<K, V> Default for IndexedDataMap<K, V>
where K: Clone + PartialEq + std::hash::Hash + Eq, V: Clone,
{
    fn default() -> Self {
        Self { keys: Vec::new(), vals: Vec::new(), index: HashMap::new() }
    }
}

impl<K, V> IndexedDataMap<K, V>
where K: Clone + PartialEq + std::hash::Hash + Eq, V: Clone,
{
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, key: K, val: V) -> usize {
        if let Some(&i) = self.index.get(&key) {
            self.vals[i - 1] = val;
            return i;
        }
        self.keys.push(key.clone());
        self.vals.push(val);
        let i = self.keys.len();
        self.index.insert(key, i);
        i
    }

    pub fn find_index(&self, key: &K) -> usize {
        self.index.get(key).copied().unwrap_or(0)
    }

    pub fn find_key(&self, i: usize) -> Option<&K> {
        if i == 0 || i > self.keys.len() { None } else { Some(&self.keys[i - 1]) }
    }

    pub fn find_from_key(&self, key: &K) -> Option<&V> {
        let i = self.find_index(key);
        if i == 0 { None } else { Some(&self.vals[i - 1]) }
    }

    pub fn find_from_index(&self, i: usize) -> Option<&V> {
        if i == 0 || i > self.vals.len() { None } else { Some(&self.vals[i - 1]) }
    }

    pub fn change_from_index(&mut self, i: usize) -> Option<&mut V> {
        if i == 0 || i > self.vals.len() { None } else { Some(&mut self.vals[i - 1]) }
    }

    pub fn size(&self) -> usize { self.keys.len() }
    pub fn is_empty(&self) -> bool { self.keys.is_empty() }
    pub fn contains(&self, key: &K) -> bool { self.index.contains_key(key) }
    pub fn clear(&mut self) { self.keys.clear(); self.vals.clear(); self.index.clear(); }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &K, &V)> {
        self.keys.iter().zip(self.vals.iter()).enumerate().map(|(i, (k, v))| (i + 1, k, v))
    }
}

// occt: NCollection_List<T> — doubly linked list (Vec-backed)
#[derive(Clone, Debug)]
pub struct NCollectionList<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> Default for NCollectionList<T> {
    fn default() -> Self { Self { data: Vec::new() } }
}

impl<T: Clone> NCollectionList<T> {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, v: T) { self.data.push(v); }
    pub fn prepend(&mut self, v: T) { self.data.insert(0, v); }
    pub fn remove_first(&mut self) { if !self.data.is_empty() { self.data.remove(0); } }
    pub fn first(&self) -> Option<&T> { self.data.first() }
    pub fn last(&self) -> Option<&T> { self.data.last() }
    pub fn size(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }

    pub fn iter(&self) -> std::slice::Iter<T> { self.data.iter() }
    pub fn as_slice(&self) -> &[T] { &self.data }
}

// occt: NCollection_Sequence<T> — random-access sequence (1-based)
#[derive(Clone, Debug)]
pub struct NCollectionSequence<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> Default for NCollectionSequence<T> {
    fn default() -> Self { Self { data: Vec::new() } }
}

impl<T: Clone> NCollectionSequence<T> {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, v: T) { self.data.push(v); }
    pub fn prepend(&mut self, v: T) { self.data.insert(0, v); }
    pub fn insert_before(&mut self, i: usize, v: T) {
        if i >= 1 && i <= self.data.len() + 1 {
            self.data.insert(i - 1, v);
        }
    }
    pub fn remove(&mut self, i: usize) {
        if i >= 1 && i <= self.data.len() { self.data.remove(i - 1); }
    }
    pub fn value(&self, i: usize) -> Option<&T> {
        if i == 0 { None } else { self.data.get(i - 1) }
    }
    pub fn set_value(&mut self, i: usize, v: T) {
        if i >= 1 && i <= self.data.len() { self.data[i - 1] = v; }
    }
    pub fn first(&self) -> Option<&T> { self.data.first() }
    pub fn last(&self) -> Option<&T> { self.data.last() }
    pub fn length(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }
    pub fn iter(&self) -> std::slice::Iter<T> { self.data.iter() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexed_map_add_find() {
        let mut m: IndexedMap<String> = IndexedMap::new();
        let i1 = m.add("alpha".to_string());
        let i2 = m.add("beta".to_string());
        let i3 = m.add("alpha".to_string());  // duplicate
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
        assert_eq!(i3, 1);
        assert_eq!(m.find_index(&"beta".to_string()), 2);
        assert_eq!(m.find_key(1), Some(&"alpha".to_string()));
        assert_eq!(m.size(), 2);
    }

    #[test]
    fn indexed_map_contains_remove() {
        let mut m: IndexedMap<u32> = IndexedMap::new();
        m.add(10);
        m.add(20);
        assert!(m.contains(&10));
        m.remove_last();
        assert_eq!(m.size(), 1);
    }

    #[test]
    fn indexed_data_map_add_find() {
        let mut m: IndexedDataMap<&str, f64> = IndexedDataMap::new();
        m.add("x", 1.5);
        m.add("y", 2.5);
        assert_eq!(m.find_from_key(&"x"), Some(&1.5));
        assert_eq!(m.find_from_index(2), Some(&2.5));
        assert_eq!(m.size(), 2);
    }

    #[test]
    fn indexed_data_map_update() {
        let mut m: IndexedDataMap<u32, String> = IndexedDataMap::new();
        m.add(1, "first".to_string());
        m.add(1, "updated".to_string());
        assert_eq!(m.find_from_index(1), Some(&"updated".to_string()));
        assert_eq!(m.size(), 1);
    }

    #[test]
    fn ncollection_list_basic() {
        let mut l: NCollectionList<i32> = NCollectionList::new();
        l.append(10);
        l.prepend(5);
        assert_eq!(l.first(), Some(&5));
        assert_eq!(l.size(), 2);
        l.remove_first();
        assert_eq!(l.first(), Some(&10));
    }

    #[test]
    fn ncollection_sequence_1based() {
        let mut s: NCollectionSequence<f64> = NCollectionSequence::new();
        s.append(1.0);
        s.append(2.0);
        s.append(3.0);
        assert_eq!(s.value(1), Some(&1.0));
        assert_eq!(s.value(3), Some(&3.0));
        assert_eq!(s.value(0), None);
        s.remove(2);
        assert_eq!(s.length(), 2);
        assert_eq!(s.value(2), Some(&3.0));
    }
}
