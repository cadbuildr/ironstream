// FILE: stepcaf_control_data_map_of_label_shape.rs
// occt: STEPCAFControl_DataMapOfLabelShape
// occt-ref: STEPCAFControl_DataMapIteratorOfDataMapOfLabelShape

use std::collections::BTreeMap;

/// Deprecated typedef for backward compatibility.
/// A map from labels to shapes using a BTreeMap with a custom key type wrapper.
/// Corresponds to NCollection_DataMap<TDF_Label, TopoDS_Shape>
pub struct StepcafControlDataMapOfLabelShape {
    // Use string representation for labels and shapes since actual types don't exist yet
    data: BTreeMap<String, String>,
}

impl StepcafControlDataMapOfLabelShape {
    /// Create a new empty map.
    pub fn new() -> Self {
        StepcafControlDataMapOfLabelShape {
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

impl Default for StepcafControlDataMapOfLabelShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the deprecated map type.
/// Corresponds to STEPCAFControl_DataMapIteratorOfDataMapOfLabelShape
pub struct StepcafControlDataMapIteratorOfDataMapOfLabelShape {
    data: Vec<(String, String)>,
    index: usize,
}

impl StepcafControlDataMapIteratorOfDataMapOfLabelShape {
    /// Create a new iterator from a map.
    pub fn new(map: &StepcafControlDataMapOfLabelShape) -> Self {
        StepcafControlDataMapIteratorOfDataMapOfLabelShape {
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
        let mut map = StepcafControlDataMapOfLabelShape::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        map.bind("label1".to_string(), "shape1".to_string());
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
        assert_eq!(map.find("label1"), Some("shape1".to_string()));
        assert_eq!(map.find("label2"), None);

        map.bind("label2".to_string(), "shape2".to_string());
        assert_eq!(map.len(), 2);
        assert!(map.contains("label1"));
        assert!(map.contains("label2"));
        assert!(!map.contains("label3"));
    }

    #[test]
    fn test_remove() {
        let mut map = StepcafControlDataMapOfLabelShape::new();
        map.bind("label1".to_string(), "shape1".to_string());
        map.bind("label2".to_string(), "shape2".to_string());

        let removed = map.remove("label1");
        assert_eq!(removed, Some("shape1".to_string()));
        assert_eq!(map.len(), 1);
        assert!(!map.contains("label1"));
    }

    #[test]
    fn test_iterator() {
        let mut map = StepcafControlDataMapOfLabelShape::new();
        map.bind("a".to_string(), "val_a".to_string());
        map.bind("b".to_string(), "val_b".to_string());

        let mut iter = StepcafControlDataMapIteratorOfDataMapOfLabelShape::new(&map);
        let mut count = 0;

        while iter.more() {
            assert!(iter.key().is_some());
            assert!(iter.value().is_some());
            count += 1;
            iter.next();
        }

        assert_eq!(count, 2);
    }
}
