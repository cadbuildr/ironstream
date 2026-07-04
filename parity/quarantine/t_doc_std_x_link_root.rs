// FILE: t_doc_std_x_link_root.rs
// occt: TDocStd_XLinkRoot

use std::collections::HashMap;

/// The root container for XLinks in a document.
#[derive(Clone, Debug)]
pub struct TDocStd_XLinkRoot {
    xlinks: HashMap<String, String>,
    id: [u8; 16],
}

impl TDocStd_XLinkRoot {
    /// Create a new XLink root.
    pub fn new() -> Self {
        Self {
            xlinks: HashMap::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for XLinkRoot attributes.
    pub fn get_id() -> [u8; 16] {
        [
            0x4D, 0x5E, 0x6F, 0x70, 0x81, 0x92, 0xA3, 0xB4, 0xC5, 0xD6, 0xE7, 0xF8, 0x66, 0x22,
            0x22, 0x22,
        ]
    }

    /// Add an XLink.
    pub fn add_xlink(&mut self, key: String, reference: String) {
        self.xlinks.insert(key, reference);
    }

    /// Get an XLink.
    pub fn get_xlink(&self, key: &str) -> Option<&str> {
        self.xlinks.get(key).map(|s| s.as_str())
    }

    /// Remove an XLink.
    pub fn remove_xlink(&mut self, key: &str) -> Option<String> {
        self.xlinks.remove(key)
    }

    /// Get the number of XLinks.
    pub fn xlink_count(&self) -> usize {
        self.xlinks.len()
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Clear all XLinks.
    pub fn clear(&mut self) {
        self.xlinks.clear();
    }
}

impl Default for TDocStd_XLinkRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_root() {
        let root = TDocStd_XLinkRoot::new();
        assert_eq!(root.xlink_count(), 0);
    }

    #[test]
    fn test_add_xlink() {
        let mut root = TDocStd_XLinkRoot::new();
        root.add_xlink("link1".to_string(), "doc.xml#0:1:2".to_string());
        assert_eq!(root.xlink_count(), 1);
    }

    #[test]
    fn test_get_xlink() {
        let mut root = TDocStd_XLinkRoot::new();
        root.add_xlink("mylink".to_string(), "file.xml#0:1".to_string());
        assert_eq!(root.get_xlink("mylink"), Some("file.xml#0:1"));
    }

    #[test]
    fn test_remove_xlink() {
        let mut root = TDocStd_XLinkRoot::new();
        root.add_xlink("link".to_string(), "ref".to_string());
        assert_eq!(root.remove_xlink("link"), Some("ref".to_string()));
        assert_eq!(root.xlink_count(), 0);
    }

    #[test]
    fn test_clear() {
        let mut root = TDocStd_XLinkRoot::new();
        root.add_xlink("a".to_string(), "1".to_string());
        root.add_xlink("b".to_string(), "2".to_string());
        root.clear();
        assert_eq!(root.xlink_count(), 0);
    }

    #[test]
    fn test_default() {
        let root = TDocStd_XLinkRoot::default();
        assert_eq!(root.xlink_count(), 0);
    }
}
