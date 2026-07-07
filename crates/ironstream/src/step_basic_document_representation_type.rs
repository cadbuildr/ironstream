// FILE: step_basic_document_representation_type.rs
// occt: StepBasic_DocumentRepresentationType

/// Representation of STEP entity DocumentRepresentationType
#[derive(Clone, Debug)]
pub struct DocumentRepresentationType {
    name: Option<String>,
    represented_document: Option<String>,
}

impl DocumentRepresentationType {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            name: None,
            represented_document: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, represented_document: String) {
        self.name = Some(name);
        self.represented_document = Some(represented_document);
    }

    /// Get name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get represented document
    pub fn represented_document(&self) -> Option<&str> {
        self.represented_document.as_deref()
    }

    /// Set represented document
    pub fn set_represented_document(&mut self, represented_document: String) {
        self.represented_document = Some(represented_document);
    }
}

impl Default for DocumentRepresentationType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let doc_rep_type = DocumentRepresentationType::new();
        assert!(doc_rep_type.name().is_none());
        assert!(doc_rep_type.represented_document().is_none());
    }

    #[test]
    fn test_init() {
        let mut doc_rep_type = DocumentRepresentationType::new();
        doc_rep_type.init("type1".to_string(), "doc1".to_string());
        assert_eq!(doc_rep_type.name(), Some("type1"));
        assert_eq!(doc_rep_type.represented_document(), Some("doc1"));
    }

    #[test]
    fn test_set_fields() {
        let mut doc_rep_type = DocumentRepresentationType::new();
        doc_rep_type.set_name("name1".to_string());
        doc_rep_type.set_represented_document("doc1".to_string());
        assert_eq!(doc_rep_type.name(), Some("name1"));
        assert_eq!(doc_rep_type.represented_document(), Some("doc1"));
    }
}
