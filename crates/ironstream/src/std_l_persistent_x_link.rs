// FILE: std_l_persistent_x_link.rs
// occt: StdLPersistent_XLink

/// Persistent representation of XLink attribute for document storage.
/// Maps to TDocStd_XLink in the transient domain.
pub struct StdLPersistentXLink {
    /// Handle to persistent document entry string
    doc_entry: Option<Box<str>>,
    /// Handle to persistent label entry string
    lab_entry: Option<Box<str>>,
}

impl StdLPersistentXLink {
    /// Create a new empty XLink
    pub fn new() -> Self {
        StdLPersistentXLink {
            doc_entry: None,
            lab_entry: None,
        }
    }

    /// Read persistent data from a file.
    pub fn read(&mut self, doc_entry: Option<Box<str>>, lab_entry: Option<Box<str>>) {
        self.doc_entry = doc_entry;
        self.lab_entry = lab_entry;
    }

    /// Write persistent data to a file.
    pub fn write(&self) -> (Option<&str>, Option<&str>) {
        (
            self.doc_entry.as_ref().map(|s| s.as_ref()),
            self.lab_entry.as_ref().map(|s| s.as_ref()),
        )
    }

    /// Gets persistent child objects
    pub fn p_children(&self) -> Vec<&str> {
        let mut children = Vec::new();
        if let Some(doc) = &self.doc_entry {
            children.push(doc.as_ref());
        }
        if let Some(lab) = &self.lab_entry {
            children.push(lab.as_ref());
        }
        children
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "PDocStd_XLink"
    }

    /// Get document entry
    pub fn document_entry(&self) -> Option<&str> {
        self.doc_entry.as_ref().map(|s| s.as_ref())
    }

    /// Set document entry
    pub fn set_document_entry(&mut self, entry: Option<Box<str>>) {
        self.doc_entry = entry;
    }

    /// Get label entry
    pub fn label_entry(&self) -> Option<&str> {
        self.lab_entry.as_ref().map(|s| s.as_ref())
    }

    /// Set label entry
    pub fn set_label_entry(&mut self, entry: Option<Box<str>>) {
        self.lab_entry = entry;
    }
}

impl Default for StdLPersistentXLink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let xlink = StdLPersistentXLink::new();
        assert_eq!(xlink.document_entry(), None);
        assert_eq!(xlink.label_entry(), None);
    }

    #[test]
    fn test_read_write() {
        let mut xlink = StdLPersistentXLink::new();
        let doc_entry = Some(Box::from("0:1:1:1"));
        let lab_entry = Some(Box::from("0:1:1:2"));

        xlink.read(doc_entry.clone(), lab_entry.clone());

        let (written_doc, written_lab) = xlink.write();
        assert_eq!(written_doc, Some("0:1:1:1"));
        assert_eq!(written_lab, Some("0:1:1:2"));
    }

    #[test]
    fn test_p_children() {
        let mut xlink = StdLPersistentXLink::new();
        xlink.set_document_entry(Some(Box::from("0:1:1:1")));
        xlink.set_label_entry(Some(Box::from("0:1:1:2")));

        let children = xlink.p_children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0], "0:1:1:1");
        assert_eq!(children[1], "0:1:1:2");
    }

    #[test]
    fn test_p_name() {
        let xlink = StdLPersistentXLink::new();
        assert_eq!(xlink.p_name(), "PDocStd_XLink");
    }

    #[test]
    fn test_partial_children() {
        let mut xlink = StdLPersistentXLink::new();
        xlink.set_document_entry(Some(Box::from("0:1:1:1")));

        let children = xlink.p_children();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], "0:1:1:1");
    }
}
