// FILE: vrml_www_anchor_map.rs
// occt: Vrml_WWWAnchorMap
//
// Faithful port of OCCT Vrml_WWWAnchorMap (DataExchange/TKDEVRML/Vrml/
// Vrml_WWWAnchorMap.hxx/.cxx): a collection mapping IDs to WWW anchors.

use std::collections::HashMap;

/// Port of Vrml_WWWAnchorMap.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlWwwAnchorMap {
    anchors: HashMap<String, String>,
}

impl VrmlWwwAnchorMap {
    /// Vrml_WWWAnchorMap with empty map.
    pub fn new() -> Self {
        VrmlWwwAnchorMap {
            anchors: HashMap::new(),
        }
    }

    /// Add or update an anchor mapping.
    pub fn bind(&mut self, key: &str, url: &str) {
        self.anchors.insert(key.to_string(), url.to_string());
    }

    /// Retrieve anchor URL by key.
    pub fn find(&self, key: &str) -> Option<&str> {
        self.anchors.get(key).map(|s| s.as_str())
    }

    /// Check if key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.anchors.contains_key(key)
    }

    /// Remove anchor by key.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.anchors.remove(key)
    }

    /// Get number of anchors.
    pub fn size(&self) -> usize {
        self.anchors.len()
    }

    /// Clear all anchors.
    pub fn clear(&mut self) {
        self.anchors.clear();
    }

    /// Get all keys.
    pub fn keys(&self) -> Vec<&str> {
        self.anchors.keys().map(|k| k.as_str()).collect()
    }

    /// Get all values.
    pub fn values(&self) -> Vec<&str> {
        self.anchors.values().map(|v| v.as_str()).collect()
    }
}

impl Default for VrmlWwwAnchorMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty() {
        let map = VrmlWwwAnchorMap::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn bind_and_find() {
        let mut map = VrmlWwwAnchorMap::new();
        map.bind("link1", "http://example.com");
        assert_eq!(map.find("link1"), Some("http://example.com"));
    }

    #[test]
    fn find_nonexistent() {
        let map = VrmlWwwAnchorMap::new();
        assert_eq!(map.find("missing"), None);
    }

    #[test]
    fn contains() {
        let mut map = VrmlWwwAnchorMap::new();
        map.bind("key1", "url1");
        assert!(map.contains("key1"));
        assert!(!map.contains("key2"));
    }

    #[test]
    fn remove() {
        let mut map = VrmlWwwAnchorMap::new();
        map.bind("key1", "url1");
        assert_eq!(map.size(), 1);
        let removed = map.remove("key1");
        assert_eq!(removed, Some("url1".to_string()));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn remove_nonexistent() {
        let mut map = VrmlWwwAnchorMap::new();
        assert_eq!(map.remove("missing"), None);
    }

    #[test]
    fn clear() {
        let mut map = VrmlWwwAnchorMap::new();
        map.bind("k1", "v1");
        map.bind("k2", "v2");
        assert_eq!(map.size(), 2);
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn keys() {
        let mut map = VrmlWwwAnchorMap::new();
        map.bind("key1", "url1");
        map.bind("key2", "url2");
        let keys = map.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"key1"));
        assert!(keys.contains(&"key2"));
    }

    #[test]
    fn values() {
        let mut map = VrmlWwwAnchorMap::new();
        map.bind("k1", "url1");
        map.bind("k2", "url2");
        let vals = map.values();
        assert_eq!(vals.len(), 2);
        assert!(vals.contains(&"url1"));
        assert!(vals.contains(&"url2"));
    }

    #[test]
    fn update_existing_key() {
        let mut map = VrmlWwwAnchorMap::new();
        map.bind("key", "url1");
        assert_eq!(map.find("key"), Some("url1"));
        map.bind("key", "url2");
        assert_eq!(map.find("key"), Some("url2"));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn equality() {
        let mut m1 = VrmlWwwAnchorMap::new();
        let mut m2 = VrmlWwwAnchorMap::new();
        m1.bind("k1", "v1");
        m2.bind("k1", "v1");
        assert_eq!(m1, m2);
    }
}
