// FILE: xcaf_prs_data_map_of_style_transient.rs
// occt: XCAFPrs_DataMapOfStyleTransient

//! Deprecated NCollection alias for mapping styles to transient objects.
//! Original: Deprecated/NCollectionAliases/XCAFPrs_DataMapOfStyleTransient.hxx
//!
//! Maps XCAFPrs_Style to generic transient objects (visualization primitives).

use std::collections::HashMap;

/// A deprecated data map for associating presentation styles with transient objects.
/// Used in XCAF presentation layer for style-to-transient (typically drawable) mapping.
#[derive(Clone, Debug)]
pub struct XCAFPrsDataMapOfStyleTransient {
    inner: HashMap<String, String>, // Placeholder: style string -> transient handle string
}

impl XCAFPrsDataMapOfStyleTransient {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Binds a style key to a transient handle. Returns true if inserted, false if replaced.
    pub fn bind(&mut self, style_key: String, transient_handle: String) -> bool {
        self.inner.insert(style_key, transient_handle).is_none()
    }

    /// Returns true if the map contains the given style key.
    pub fn contains(&self, style_key: &str) -> bool {
        self.inner.contains_key(style_key)
    }

    /// Returns a reference to the transient handle associated with the style.
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

impl Default for XCAFPrsDataMapOfStyleTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_transient() {
        let mut map = XCAFPrsDataMapOfStyleTransient::new();
        let inserted = map.bind("style_blue".to_string(), "transient_0x1234".to_string());
        assert!(inserted);
        assert_eq!(map.find("style_blue"), Some("transient_0x1234"));
    }

    #[test]
    fn test_contains() {
        let mut map = XCAFPrsDataMapOfStyleTransient::new();
        assert!(!map.contains("missing"));
        map.bind("present".to_string(), "handle".to_string());
        assert!(map.contains("present"));
    }

    #[test]
    fn test_remove() {
        let mut map = XCAFPrsDataMapOfStyleTransient::new();
        map.bind("style".to_string(), "transient".to_string());
        assert!(map.remove("style"));
        assert!(!map.contains("style"));
        assert!(!map.remove("nonexistent"));
    }

    #[test]
    fn test_multiple_styles() {
        let mut map = XCAFPrsDataMapOfStyleTransient::new();
        for i in 0..5 {
            map.bind(
                format!("style_{}", i),
                format!("transient_{:x}", i),
            );
        }
        assert_eq!(map.size(), 5);
        assert_eq!(map.find("style_2"), Some("transient_2"));
    }

    #[test]
    fn test_clear() {
        let mut map = XCAFPrsDataMapOfStyleTransient::new();
        map.bind("s1".to_string(), "t1".to_string());
        map.bind("s2".to_string(), "t2".to_string());
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
