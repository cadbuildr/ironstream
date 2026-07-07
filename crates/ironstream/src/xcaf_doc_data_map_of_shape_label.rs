// FILE: xcaf_doc_data_map_of_shape_label.rs
// occt: XCAFDoc_DataMapOfShapeLabel

//! Deprecated NCollection alias for mapping shapes to labels.
//! Original: Deprecated/NCollectionAliases/XCAFDoc_DataMapOfShapeLabel.hxx
//!
//! This is a thin alias over a generic hash map. In a full port, this would be:
//! type XCAFDoc_DataMapOfShapeLabel = NCollection_DataMap<TopoDS_Shape, TDF_Label>

use std::collections::HashMap;

/// A deprecated data map type that associates TopoDS shapes with TDF labels.
/// This is provided for backward compatibility in XCAF document handling.
#[derive(Clone, Debug)]
pub struct XCAFDocDataMapOfShapeLabel {
    inner: HashMap<String, String>, // Placeholder: shape string key -> label string value
}

impl XCAFDocDataMapOfShapeLabel {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Binds a shape key to a label value. Returns true if inserted, false if replaced.
    pub fn bind(&mut self, shape_key: String, label_value: String) -> bool {
        self.inner.insert(shape_key, label_value).is_none()
    }

    /// Returns true if the map contains the given shape key.
    pub fn contains(&self, shape_key: &str) -> bool {
        self.inner.contains_key(shape_key)
    }

    /// Returns a reference to the label associated with the shape key, if it exists.
    pub fn find(&self, shape_key: &str) -> Option<&str> {
        self.inner.get(shape_key).map(|s| s.as_str())
    }

    /// Removes the entry for the given shape key. Returns true if it was present.
    pub fn remove(&mut self, shape_key: &str) -> bool {
        self.inner.remove(shape_key).is_some()
    }

    /// Returns the number of entries in the map.
    pub fn size(&self) -> usize {
        self.inner.len()
    }

    /// Clears all entries from the map.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl Default for XCAFDocDataMapOfShapeLabel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_and_find() {
        let mut map = XCAFDocDataMapOfShapeLabel::new();
        let inserted = map.bind("shape1".to_string(), "label1".to_string());
        assert!(inserted);
        assert_eq!(map.find("shape1"), Some("label1"));
    }

    #[test]
    fn test_bind_replace() {
        let mut map = XCAFDocDataMapOfShapeLabel::new();
        map.bind("shape1".to_string(), "label1".to_string());
        let replaced = map.bind("shape1".to_string(), "label2".to_string());
        assert!(!replaced); // Returns false when replacing
        assert_eq!(map.find("shape1"), Some("label2"));
    }

    #[test]
    fn test_contains() {
        let mut map = XCAFDocDataMapOfShapeLabel::new();
        assert!(!map.contains("shape1"));
        map.bind("shape1".to_string(), "label1".to_string());
        assert!(map.contains("shape1"));
    }

    #[test]
    fn test_remove() {
        let mut map = XCAFDocDataMapOfShapeLabel::new();
        map.bind("shape1".to_string(), "label1".to_string());
        assert!(map.remove("shape1"));
        assert!(!map.contains("shape1"));
        assert!(!map.remove("shape1")); // Already removed
    }

    #[test]
    fn test_size_and_clear() {
        let mut map = XCAFDocDataMapOfShapeLabel::new();
        assert_eq!(map.size(), 0);
        map.bind("s1".to_string(), "l1".to_string());
        map.bind("s2".to_string(), "l2".to_string());
        assert_eq!(map.size(), 2);
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
