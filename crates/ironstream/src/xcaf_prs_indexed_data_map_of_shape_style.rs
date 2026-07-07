// FILE: xcaf_prs_indexed_data_map_of_shape_style.rs
// occt: XCAFPrs_IndexedDataMapOfShapeStyle

//! Deprecated NCollection alias for indexed mapping of shapes to styles.
//! Original: Deprecated/NCollectionAliases/XCAFPrs_IndexedDataMapOfShapeStyle.hxx
//!
//! This is a thin alias over an indexed hash map where shapes are keys.
//! type XCAFPrs_IndexedDataMapOfShapeStyle = NCollection_IndexedDataMap<TopoDS_Shape, XCAFPrs_Style>

use std::collections::HashMap;

/// A deprecated indexed data map for associating shapes with presentation styles.
/// Indexed maps maintain insertion order and allow 1-based access by index.
#[derive(Clone, Debug)]
pub struct XCAFPrsIndexedDataMapOfShapeStyle {
    entries: Vec<(String, String)>, // Maintains order: shape key -> style value
    map: HashMap<String, usize>,    // Quick lookup: shape key -> index (0-based internally)
}

impl XCAFPrsIndexedDataMapOfShapeStyle {
    /// Creates a new empty indexed data map.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Adds or updates a shape-to-style binding. Returns the 1-based index.
    pub fn add(&mut self, shape_key: String, style_value: String) -> usize {
        match self.map.get(&shape_key) {
            Some(&idx) => {
                self.entries[idx].1 = style_value;
                idx + 1 // Return 1-based index
            }
            None => {
                let idx = self.entries.len();
                self.entries.push((shape_key.clone(), style_value));
                self.map.insert(shape_key, idx);
                idx + 1 // Return 1-based index
            }
        }
    }

    /// Returns the style associated with the given shape, if it exists.
    pub fn find(&self, shape_key: &str) -> Option<&str> {
        self.map
            .get(shape_key)
            .map(|&idx| self.entries[idx].1.as_str())
    }

    /// Returns the style at the given 1-based index, if valid.
    pub fn find_from_index(&self, index: usize) -> Option<(&str, &str)> {
        if index == 0 || index > self.entries.len() {
            None
        } else {
            let (k, v) = &self.entries[index - 1];
            Some((k.as_str(), v.as_str()))
        }
    }

    /// Removes the entry for the given shape key. Returns true if it was present.
    pub fn remove(&mut self, shape_key: &str) -> bool {
        if let Some(&idx) = self.map.get(shape_key) {
            self.entries.remove(idx);
            self.map.remove(shape_key);
            // Rebuild map indices after removal
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.map.insert(k.clone(), i);
            }
            true
        } else {
            false
        }
    }

    /// Returns the number of entries in the map.
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Clears all entries from the map.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.map.clear();
    }

    /// Returns true if the map contains the given shape key.
    pub fn contains(&self, shape_key: &str) -> bool {
        self.map.contains_key(shape_key)
    }
}

impl Default for XCAFPrsIndexedDataMapOfShapeStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_find() {
        let mut map = XCAFPrsIndexedDataMapOfShapeStyle::new();
        let idx = map.add("box".to_string(), "red_style".to_string());
        assert_eq!(idx, 1);
        assert_eq!(map.find("box"), Some("red_style"));
    }

    #[test]
    fn test_indexed_access() {
        let mut map = XCAFPrsIndexedDataMapOfShapeStyle::new();
        map.add("shape1".to_string(), "style1".to_string());
        map.add("shape2".to_string(), "style2".to_string());
        map.add("shape3".to_string(), "style3".to_string());

        assert_eq!(map.find_from_index(1), Some(("shape1", "style1")));
        assert_eq!(map.find_from_index(2), Some(("shape2", "style2")));
        assert_eq!(map.find_from_index(3), Some(("shape3", "style3")));
        assert_eq!(map.find_from_index(0), None);
        assert_eq!(map.find_from_index(4), None);
    }

    #[test]
    fn test_update_existing() {
        let mut map = XCAFPrsIndexedDataMapOfShapeStyle::new();
        let idx1 = map.add("shape".to_string(), "style1".to_string());
        let idx2 = map.add("shape".to_string(), "style2".to_string());
        assert_eq!(idx1, idx2); // Same index
        assert_eq!(map.find("shape"), Some("style2")); // Updated
    }

    #[test]
    fn test_contains() {
        let mut map = XCAFPrsIndexedDataMapOfShapeStyle::new();
        assert!(!map.contains("shape"));
        map.add("shape".to_string(), "style".to_string());
        assert!(map.contains("shape"));
    }

    #[test]
    fn test_remove() {
        let mut map = XCAFPrsIndexedDataMapOfShapeStyle::new();
        map.add("s1".to_string(), "st1".to_string());
        map.add("s2".to_string(), "st2".to_string());
        map.add("s3".to_string(), "st3".to_string());

        assert!(map.remove("s2"));
        assert_eq!(map.size(), 2);
        assert!(!map.contains("s2"));
        assert_eq!(map.find_from_index(2), Some(("s3", "st3"))); // Reindexed
    }

    #[test]
    fn test_size_and_clear() {
        let mut map = XCAFPrsIndexedDataMapOfShapeStyle::new();
        assert_eq!(map.size(), 0);
        map.add("s1".to_string(), "st1".to_string());
        map.add("s2".to_string(), "st2".to_string());
        assert_eq!(map.size(), 2);
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
