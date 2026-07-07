// FILE: step_basic_document_reference.rs
// occt: StepBasic_DocumentReference

/// Representation of STEP entity DocumentReference
#[derive(Clone, Debug)]
pub struct DocumentReference {
    assigned_document: Option<String>,
    source: Option<String>,
}

impl DocumentReference {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            assigned_document: None,
            source: None,
        }
    }

    /// Initialize all fields
    pub fn init0(&mut self, assigned_document: String, source: String) {
        self.assigned_document = Some(assigned_document);
        self.source = Some(source);
    }

    /// Returns field AssignedDocument
    pub fn assigned_document(&self) -> Option<&str> {
        self.assigned_document.as_deref()
    }

    /// Set field AssignedDocument
    pub fn set_assigned_document(&mut self, assigned_document: String) {
        self.assigned_document = Some(assigned_document);
    }

    /// Returns field Source
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Set field Source
    pub fn set_source(&mut self, source: String) {
        self.source = Some(source);
    }
}

impl Default for DocumentReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_constructor() {
        let doc_ref = DocumentReference::new();
        assert!(doc_ref.assigned_document().is_none());
        assert!(doc_ref.source().is_none());
    }

    #[test]
    fn test_init0() {
        let mut doc_ref = DocumentReference::new();
        doc_ref.init0("doc123".to_string(), "source1".to_string());
        assert_eq!(doc_ref.assigned_document(), Some("doc123"));
        assert_eq!(doc_ref.source(), Some("source1"));
    }

    #[test]
    fn test_set_assigned_document() {
        let mut doc_ref = DocumentReference::new();
        doc_ref.set_assigned_document("doc456".to_string());
        assert_eq!(doc_ref.assigned_document(), Some("doc456"));
    }

    #[test]
    fn test_set_source() {
        let mut doc_ref = DocumentReference::new();
        doc_ref.set_source("src2".to_string());
        assert_eq!(doc_ref.source(), Some("src2"));
    }

    #[test]
    fn test_default() {
        let doc_ref = DocumentReference::default();
        assert!(doc_ref.assigned_document().is_none());
        assert!(doc_ref.source().is_none());
    }
}
