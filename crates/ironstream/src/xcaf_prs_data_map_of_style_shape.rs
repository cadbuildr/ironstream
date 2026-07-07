// FILE: xcaf_prs_data_map_of_style_shape.rs
// occt: XCAFPrs_DataMapOfStyleShape

//! Deprecated NCollection alias for mapping styles to shapes.
//! Original: Deprecated/NCollectionAliases/XCAFPrs_DataMapOfStyleShape.hxx
//!
//! This provides a map from XCAFPrs_Style to TopoDS_Shape for presentation purposes.

use std::collections::HashMap;

/// A deprecated data map for associating presentation styles with shapes.
/// Used in XCAF presentation layer for style-based shape lookup.
#[derive(Clone, Debug)]
pub struct XCAFPrsDataMapOfStyleShape {
    inner: HashMap<String, String>, // Placeholder: style string key -> shape string value
}

impl XCAFPrsDataMapOfStyleShape {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Binds a style key to a shape value. Returns true if inserted, false if replaced.
    pub fn bind(&mut self, style_key: String, shape_value: String) -> bool {
        self.inner.insert(style_key, shape_value).is_none()
    }

    /// Returns true if the map contains the given style key.
    pub fn contains(&self, style_key: &str) -> bool {
        self.inner.contains_key(style_key)
    }

    /// Returns a reference to the shape associated with the style, if it exists.
    pub fn find(&self, style_key: &str) -> Option<&str> {
        self.inner.get(style_key).map(|s| s.as_str())
    }

    /// Removes the entry for the given style key. Returns true if it was present.
    pub fn remove(&mut self, style_key: &str) -> bool {
        self.inner.remove(style_key).is_some()
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

impl Default for XCAFPrsDataMapOfStyleShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_shape_binding() {
        let mut map = XCAFPrsDataMapOfStyleShape::new();
        let inserted = map.bind("red_style".to_string(), "cube_shape".to_string());
        assert!(inserted);
        assert_eq!(map.find("red_style"), Some("cube_shape"));
    }

    #[test]
    fn test_multiple_entries() {
        let mut map = XCAFPrsDataMapOfStyleShape::new();
        map.bind("style1".to_string(), "shape1".to_string());
        map.bind("style2".to_string(), "shape2".to_string());
        map.bind("style3".to_string(), "shape3".to_string());
        assert_eq!(map.size(), 3);
        assert_eq!(map.find("style2"), Some("shape2"));
    }

    #[test]
    fn test_remove_entry() {
        let mut map = XCAFPrsDataMapOfStyleShape::new();
        map.bind("style".to_string(), "shape".to_string());
        assert!(map.remove("style"));
        assert!(!map.contains("style"));
    }

    #[test]
    fn test_clear() {
        let mut map = XCAFPrsDataMapOfStyleShape::new();
        map.bind("s1".to_string(), "sh1".to_string());
        map.bind("s2".to_string(), "sh2".to_string());
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
