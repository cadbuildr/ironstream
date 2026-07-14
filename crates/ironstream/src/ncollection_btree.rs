// FILE: ncollection_btree.rs
// occt: NCollection_IndexedMap
// occt-ref: NCollection_IndexedDataMap
//       NCollection_DataMap, NCollection_Map

use std::collections::HashMap;

/// A map where each unique item also gets a stable 1-based index.
/// occt-note: NCollection_IndexedMap<TheKey>
#[derive(Clone, Debug)]
pub struct NCollIndexedMap {
    items: Vec<u32>,
    index: HashMap<u32, usize>,  // value → 1-based index
}

impl Default for NCollIndexedMap {
    fn default() -> Self { Self { items: Vec::new(), index: HashMap::new() } }
}

impl NCollIndexedMap {
    pub fn new() -> Self { Self::default() }

    /// Add a key; return its 1-based index (existing key returns its prior index).
    pub fn add(&mut self, key: u32) -> usize {
        if let Some(&idx) = self.index.get(&key) { return idx; }
        self.items.push(key);
        let idx = self.items.len();
        self.index.insert(key, idx);
        idx
    }

    /// Find the 1-based index of a key, or 0 if absent.
    pub fn find_index(&self, key: u32) -> usize {
        self.index.get(&key).copied().unwrap_or(0)
    }

    /// Find the key at a 1-based index, or None.
    pub fn find_key(&self, idx: usize) -> Option<u32> {
        if idx == 0 { return None; }
        self.items.get(idx - 1).copied()
    }

    pub fn contains(&self, key: u32) -> bool { self.index.contains_key(&key) }
    pub fn size(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }

    /// Remove the last-added key (stack-like removal).
    pub fn remove_last(&mut self) {
        if let Some(key) = self.items.pop() {
            self.index.remove(&key);
        }
    }

    pub fn clear(&mut self) { self.items.clear(); self.index.clear(); }
}

/// A map where each unique key maps to a value AND has a stable 1-based index.
/// occt-note: NCollection_IndexedDataMap<TheKey, TheItem>
#[derive(Clone, Debug)]
pub struct NCollIndexedDataMap {
    keys: Vec<u32>,
    values: Vec<u32>,
    index: HashMap<u32, usize>,  // key → 1-based index
}

impl Default for NCollIndexedDataMap {
    fn default() -> Self { Self { keys: Vec::new(), values: Vec::new(), index: HashMap::new() } }
}

impl NCollIndexedDataMap {
    pub fn new() -> Self { Self::default() }

    /// Add or replace a key/value pair; return 1-based index.
    pub fn add(&mut self, key: u32, value: u32) -> usize {
        if let Some(&idx) = self.index.get(&key) {
            self.values[idx - 1] = value;
            return idx;
        }
        self.keys.push(key);
        self.values.push(value);
        let idx = self.keys.len();
        self.index.insert(key, idx);
        idx
    }

    pub fn find(&self, key: u32) -> Option<u32> {
        self.index.get(&key).map(|&i| self.values[i - 1])
    }

    pub fn find_from_index(&self, idx: usize) -> Option<u32> {
        if idx == 0 { return None; }
        self.values.get(idx - 1).copied()
    }

    pub fn key_from_index(&self, idx: usize) -> Option<u32> {
        if idx == 0 { return None; }
        self.keys.get(idx - 1).copied()
    }

    pub fn contains(&self, key: u32) -> bool { self.index.contains_key(&key) }
    pub fn size(&self) -> usize { self.keys.len() }
    pub fn clear(&mut self) { self.keys.clear(); self.values.clear(); self.index.clear(); }
}

/// General key→value map.
/// occt-note: NCollection_DataMap<TheKey, TheItem>
#[derive(Clone, Debug, Default)]
pub struct NCollDataMap {
    map: HashMap<u32, u32>,
}

impl NCollDataMap {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, key: u32, value: u32) { self.map.insert(key, value); }
    pub fn unbind(&mut self, key: u32) { self.map.remove(&key); }
    pub fn find(&self, key: u32) -> Option<u32> { self.map.get(&key).copied() }
    pub fn is_bound(&self, key: u32) -> bool { self.map.contains_key(&key) }
    pub fn size(&self) -> usize { self.map.len() }
    pub fn is_empty(&self) -> bool { self.map.is_empty() }
    pub fn clear(&mut self) { self.map.clear(); }

    /// Change the value at key if bound; return a mutable reference or insert new.
    pub fn change_find_or_insert(&mut self, key: u32) -> &mut u32 {
        self.map.entry(key).or_insert(0)
    }
}

/// A set of unique keys (no values).
/// occt-note: NCollection_Map<TheKey>
#[derive(Clone, Debug, Default)]
pub struct NCollMap {
    set: std::collections::HashSet<u32>,
}

impl NCollMap {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, key: u32) -> bool { self.set.insert(key) }
    pub fn remove(&mut self, key: u32) -> bool { self.set.remove(&key) }
    pub fn contains(&self, key: u32) -> bool { self.set.contains(&key) }
    pub fn size(&self) -> usize { self.set.len() }
    pub fn is_empty(&self) -> bool { self.set.is_empty() }
    pub fn clear(&mut self) { self.set.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexed_map_add_find() {
        let mut m = NCollIndexedMap::new();
        let i1 = m.add(100);
        let i2 = m.add(200);
        let i3 = m.add(100); // duplicate
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
        assert_eq!(i3, 1);  // same index for duplicate
        assert_eq!(m.size(), 2);
        assert_eq!(m.find_key(1), Some(100));
        assert_eq!(m.find_index(200), 2);
        assert_eq!(m.find_index(999), 0);
    }

    #[test]
    fn indexed_map_remove_last() {
        let mut m = NCollIndexedMap::new();
        m.add(1); m.add(2); m.add(3);
        m.remove_last();
        assert_eq!(m.size(), 2);
        assert!(!m.contains(3));
    }

    #[test]
    fn indexed_data_map() {
        let mut m = NCollIndexedDataMap::new();
        m.add(10, 100);
        m.add(20, 200);
        m.add(10, 999); // update
        assert_eq!(m.find(10), Some(999));
        assert_eq!(m.find(20), Some(200));
        assert_eq!(m.size(), 2);
        assert_eq!(m.key_from_index(1), Some(10));
        assert_eq!(m.find_from_index(2), Some(200));
    }

    #[test]
    fn data_map_bind_find() {
        let mut m = NCollDataMap::new();
        m.bind(1, 100);
        m.bind(2, 200);
        assert_eq!(m.find(1), Some(100));
        assert!(m.is_bound(2));
        m.unbind(1);
        assert!(!m.is_bound(1));
    }

    #[test]
    fn map_set() {
        let mut m = NCollMap::new();
        assert!(m.add(5));
        assert!(!m.add(5)); // already present
        assert!(m.contains(5));
        m.remove(5);
        assert!(!m.contains(5));
    }
}
