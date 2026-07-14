// FILE: resource_data_map_of_ascii_string_extended_string.rs
// occt: Resource_DataMapOfAsciiStringExtendedString
// occt-ref: Resource_DataMapIteratorOfDataMapOfAsciiStringExtendedString

use std::collections::BTreeMap;

/// Deprecated typedef for backward compatibility.
/// Maps ASCII strings to extended strings using a BTreeMap.
/// Corresponds to NCollection_DataMap<TCollection_AsciiString, TCollection_ExtendedString>
pub struct ResourceDataMapOfAsciiStringExtendedString {
    data: BTreeMap<String, String>,
}

impl ResourceDataMapOfAsciiStringExtendedString {
    /// Create a new empty map.
    pub fn new() -> Self {
        ResourceDataMapOfAsciiStringExtendedString {
            data: BTreeMap::new(),
        }
    }

    /// Insert a key-value pair into the map.
    pub fn bind(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Find a value by key.
    pub fn find(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    /// Remove a key-value pair from the map.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    /// Clear all entries from the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the number of entries in the map.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Check if a key exists in the map.
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Create an iterator over the map entries.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }
}

impl Default for ResourceDataMapOfAsciiStringExtendedString {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the deprecated map type.
/// Corresponds to Resource_DataMapIteratorOfDataMapOfAsciiStringExtendedString
pub struct ResourceDataMapIteratorOfDataMapOfAsciiStringExtendedString {
    data: Vec<(String, String)>,
    index: usize,
}

impl ResourceDataMapIteratorOfDataMapOfAsciiStringExtendedString {
    /// Create a new iterator from a map.
    pub fn new(map: &ResourceDataMapOfAsciiStringExtendedString) -> Self {
        ResourceDataMapIteratorOfDataMapOfAsciiStringExtendedString {
            data: map.data.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            index: 0,
        }
    }

    /// Check if there are more entries.
    pub fn more(&self) -> bool {
        self.index < self.data.len()
    }

    /// Move to the next entry.
    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    /// Get the current key.
    pub fn key(&self) -> Option<&String> {
        if self.more() {
            Some(&self.data[self.index].0)
        } else {
            None
        }
    }

    /// Get the current value.
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
    fn test_basic_operations() {
        let mut map = ResourceDataMapOfAsciiStringExtendedString::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        // Test bind and find
        map.bind("key1".to_string(), "value1".to_string());
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
        assert_eq!(map.find("key1"), Some("value1".to_string()));
        assert_eq!(map.find("key2"), None);

        // Test multiple entries
        map.bind("key2".to_string(), "value2".to_string());
        map.bind("key3".to_string(), "value3".to_string());
        assert_eq!(map.len(), 3);

        // Test contains
        assert!(map.contains("key1"));
        assert!(map.contains("key2"));
        assert!(!map.contains("key4"));
    }

    #[test]
    fn test_remove() {
        let mut map = ResourceDataMapOfAsciiStringExtendedString::new();
        map.bind("key1".to_string(), "value1".to_string());
        map.bind("key2".to_string(), "value2".to_string());

        let removed = map.remove("key1");
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(map.len(), 1);
        assert!(!map.contains("key1"));
        assert!(map.contains("key2"));
    }

    #[test]
    fn test_clear() {
        let mut map = ResourceDataMapOfAsciiStringExtendedString::new();
        map.bind("key1".to_string(), "value1".to_string());
        map.bind("key2".to_string(), "value2".to_string());
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_iterator() {
        let mut map = ResourceDataMapOfAsciiStringExtendedString::new();
        map.bind("a".to_string(), "val_a".to_string());
        map.bind("b".to_string(), "val_b".to_string());
        map.bind("c".to_string(), "val_c".to_string());

        let mut iter = ResourceDataMapIteratorOfDataMapOfAsciiStringExtendedString::new(&map);
        let mut count = 0;

        while iter.more() {
            assert!(iter.key().is_some());
            assert!(iter.value().is_some());
            count += 1;
            iter.next();
        }

        assert_eq!(count, 3);
    }
}
