// FILE: t_doc_std_x_link_ptr.rs
// occt: TDocStd_XLinkPtr

/// Represents a raw pointer to an XLink.
/// In Rust, we use a type alias for compatibility with C++ semantics.
pub type TDocStd_XLinkPtr = Option<Box<TDocStd_XLink>>;

/// An XLink reference structure.
#[derive(Clone, Debug)]
pub struct TDocStd_XLink {
    doc: String,
    entry: String,
}

impl TDocStd_XLink {
    /// Create a new XLink.
    pub fn new(doc: String, entry: String) -> Self {
        Self { doc, entry }
    }

    /// Get the document reference.
    pub fn doc(&self) -> &str {
        &self.doc
    }

    /// Get the entry reference.
    pub fn entry(&self) -> &str {
        &self.entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_xlink() {
        let xlink = TDocStd_XLink::new("doc.xml".to_string(), "0:1:2".to_string());
        assert_eq!(xlink.doc(), "doc.xml");
        assert_eq!(xlink.entry(), "0:1:2");
    }

    #[test]
    fn test_xlink_ptr_none() {
        let ptr: TDocStd_XLinkPtr = None;
        assert!(ptr.is_none());
    }

    #[test]
    fn test_xlink_ptr_some() {
        let xlink = TDocStd_XLink::new("test.xml".to_string(), "0:2".to_string());
        let ptr: TDocStd_XLinkPtr = Some(Box::new(xlink));
        assert!(ptr.is_some());
    }
}
