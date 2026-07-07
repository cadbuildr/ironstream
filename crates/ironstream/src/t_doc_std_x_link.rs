// FILE: t_doc_std_x_link.rs
// occt: TDocStd_XLink

/// An XLink (external link) attribute for cross-document references.
#[derive(Clone, Debug)]
pub struct TDocStd_XLink {
    reference_doc: String,
    reference_entry: String,
    id: [u8; 16],
}

impl TDocStd_XLink {
    /// Create a new XLink attribute.
    pub fn new() -> Self {
        Self {
            reference_doc: String::new(),
            reference_entry: String::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for XLink attributes.
    pub fn get_id() -> [u8; 16] {
        [
            0x3B, 0x4C, 0x5D, 0x6E, 0x7F, 0x80, 0x91, 0xA2, 0xB3, 0xC4, 0xD5, 0xE6, 0x77, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set the referenced document.
    pub fn set_reference(&mut self, doc: String, entry: String) {
        self.reference_doc = doc;
        self.reference_entry = entry;
    }

    /// Get the referenced document.
    pub fn reference_doc(&self) -> &str {
        &self.reference_doc
    }

    /// Get the referenced entry.
    pub fn reference_entry(&self) -> &str {
        &self.reference_entry
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Update the external link reference.
    pub fn update(&mut self) -> bool {
        !self.reference_doc.is_empty() && !self.reference_entry.is_empty()
    }
}

impl Default for TDocStd_XLink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_xlink() {
        let xlink = TDocStd_XLink::new();
        assert_eq!(xlink.reference_doc(), "");
        assert_eq!(xlink.reference_entry(), "");
    }

    #[test]
    fn test_set_reference() {
        let mut xlink = TDocStd_XLink::new();
        xlink.set_reference("doc.xml".to_string(), "0:1:2".to_string());
        assert_eq!(xlink.reference_doc(), "doc.xml");
        assert_eq!(xlink.reference_entry(), "0:1:2");
    }

    #[test]
    fn test_update() {
        let mut xlink = TDocStd_XLink::new();
        assert!(!xlink.update());
        xlink.set_reference("doc.xml".to_string(), "0:1:2".to_string());
        assert!(xlink.update());
    }

    #[test]
    fn test_default() {
        let xlink = TDocStd_XLink::default();
        assert_eq!(xlink.reference_doc(), "");
    }
}
