// FILE: t_doc_std_x_link_tool.rs
// occt: TDocStd_XLinkTool

/// A utility tool for managing XLink operations.
pub struct TDocStd_XLinkTool;

impl TDocStd_XLinkTool {
    /// Encode a reference as a string.
    pub fn encode_reference(doc: &str, entry: &str) -> String {
        format!("{}#{}", doc, entry)
    }

    /// Decode a reference string into document and entry.
    pub fn decode_reference(reference: &str) -> (String, String) {
        if let Some(hash_idx) = reference.find('#') {
            let doc = reference[..hash_idx].to_string();
            let entry = reference[hash_idx + 1..].to_string();
            (doc, entry)
        } else {
            (reference.to_string(), String::new())
        }
    }

    /// Check if a reference is valid.
    pub fn is_valid_reference(reference: &str) -> bool {
        !reference.is_empty() && reference.contains('#')
    }

    /// Update an XLink reference.
    pub fn update_reference(old_ref: &str, new_doc: &str) -> String {
        let (_, entry) = Self::decode_reference(old_ref);
        Self::encode_reference(new_doc, &entry)
    }

    /// Create a new reference.
    pub fn create_reference(doc: &str, entry: &str) -> String {
        Self::encode_reference(doc, entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_reference() {
        let ref_str = TDocStd_XLinkTool::encode_reference("doc.xml", "0:1:2");
        assert_eq!(ref_str, "doc.xml#0:1:2");
    }

    #[test]
    fn test_decode_reference() {
        let (doc, entry) = TDocStd_XLinkTool::decode_reference("doc.xml#0:1:2");
        assert_eq!(doc, "doc.xml");
        assert_eq!(entry, "0:1:2");
    }

    #[test]
    fn test_decode_no_entry() {
        let (doc, entry) = TDocStd_XLinkTool::decode_reference("doc.xml");
        assert_eq!(doc, "doc.xml");
        assert_eq!(entry, "");
    }

    #[test]
    fn test_is_valid_reference() {
        assert!(TDocStd_XLinkTool::is_valid_reference("doc.xml#0:1"));
        assert!(!TDocStd_XLinkTool::is_valid_reference("doc.xml"));
        assert!(!TDocStd_XLinkTool::is_valid_reference(""));
    }

    #[test]
    fn test_update_reference() {
        let updated = TDocStd_XLinkTool::update_reference("old.xml#0:1:2", "new.xml");
        assert_eq!(updated, "new.xml#0:1:2");
    }

    #[test]
    fn test_create_reference() {
        let ref_str = TDocStd_XLinkTool::create_reference("test.xml", "0:2");
        assert_eq!(ref_str, "test.xml#0:2");
    }
}
