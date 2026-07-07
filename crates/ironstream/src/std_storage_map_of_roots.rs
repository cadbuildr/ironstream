// FILE: std_storage_map_of_roots.rs
// occt: StdStorage_MapOfRoots, StdStorage_DataMapIteratorOfMapOfRoots

use std::collections::BTreeMap;

/// Deprecated typedef for backward compatibility.
/// Maps ASCII strings to storage roots.
/// Corresponds to NCollection_IndexedDataMap<TCollection_AsciiString, opencascade::handle<StdStorage_Root>>
pub struct StdStorageMapOfRoots {
    data: BTreeMap<String, String>,
}

impl StdStorageMapOfRoots {
    pub fn new() -> Self {
        StdStorageMapOfRoots {
            data: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn find(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }
}

impl Default for StdStorageMapOfRoots {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the deprecated map type.
pub struct StdStorageDataMapIteratorOfMapOfRoots {
    data: Vec<(String, String)>,
    index: usize,
}

impl StdStorageDataMapIteratorOfMapOfRoots {
    pub fn new(map: &StdStorageMapOfRoots) -> Self {
        StdStorageDataMapIteratorOfMapOfRoots {
            data: map.data.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            index: 0,
        }
    }

    pub fn more(&self) -> bool {
        self.index < self.data.len()
    }

    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    pub fn key(&self) -> Option<&String> {
        if self.more() {
            Some(&self.data[self.index].0)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<&String> {
        if self.more() {
            Some(&self.data[self.index].1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = StdStorageMapOfRoots::new();
        map.bind("root1".to_string(), "data1".to_string());
        assert_eq!(map.find("root1"), Some("data1".to_string()));
    }

    #[test]
    fn test_iterator() {
        let mut map = StdStorageMapOfRoots::new();
        map.bind("a".to_string(), "1".to_string());
        map.bind("b".to_string(), "2".to_string());

        let mut iter = StdStorageDataMapIteratorOfMapOfRoots::new(&map);
        let mut count = 0;
        while iter.more() {
            count += 1;
            iter.next();
        }
        assert_eq!(count, 2);
    }
}
