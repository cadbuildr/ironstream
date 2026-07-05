// FILE: xml_obj_mgt_document.rs
// occt: XmlObjMgt_Document

/// XmlObjMgt_Document is a typedef alias for XML DOM document.
/// In OCCT, it wraps LDOM_Document for XML document operations.
#[derive(Clone, Debug)]
pub struct XmlObjMgt_Document {
    root: Option<String>,
}

impl XmlObjMgt_Document {
    /// Create a new empty document.
    pub fn new() -> Self {
        XmlObjMgt_Document { root: None }
    }

    /// Set the root element.
    pub fn set_root(&mut self, root: String) {
        self.root = Some(root);
    }

    /// Get the root element.
    pub fn root(&self) -> Option<&str> {
        self.root.as_deref()
    }
}

impl Default for XmlObjMgt_Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = XmlObjMgt_Document::new();
        assert_eq!(doc.root(), None);
    }

    #[test]
    fn test_set_root() {
        let mut doc = XmlObjMgt_Document::new();
        doc.set_root("test_root".to_string());
        assert_eq!(doc.root(), Some("test_root"));
    }
}
