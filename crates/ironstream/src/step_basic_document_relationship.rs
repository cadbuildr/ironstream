// FILE: step_basic_document_relationship.rs
// occt: StepBasic_DocumentRelationship

/// Representation of STEP entity DocumentRelationship
#[derive(Clone, Debug)]
pub struct DocumentRelationship {
    name: Option<String>,
    description: Option<String>,
    relating_document: Option<String>,
    related_document: Option<String>,
}

impl DocumentRelationship {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            relating_document: None,
            related_document: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        relating: String,
        related: String,
    ) {
        self.name = Some(name);
        self.description = Some(description);
        self.relating_document = Some(relating);
        self.related_document = Some(related);
    }

    /// Get name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Get relating document
    pub fn relating_document(&self) -> Option<&str> {
        self.relating_document.as_deref()
    }

    /// Set relating document
    pub fn set_relating_document(&mut self, relating: String) {
        self.relating_document = Some(relating);
    }

    /// Get related document
    pub fn related_document(&self) -> Option<&str> {
        self.related_document.as_deref()
    }

    /// Set related document
    pub fn set_related_document(&mut self, related: String) {
        self.related_document = Some(related);
    }
}

impl Default for DocumentRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let doc_rel = DocumentRelationship::new();
        assert!(doc_rel.name().is_none());
        assert!(doc_rel.description().is_none());
        assert!(doc_rel.relating_document().is_none());
        assert!(doc_rel.related_document().is_none());
    }

    #[test]
    fn test_init() {
        let mut doc_rel = DocumentRelationship::new();
        doc_rel.init(
            "rel1".to_string(),
            "desc1".to_string(),
            "doc1".to_string(),
            "doc2".to_string(),
        );
        assert_eq!(doc_rel.name(), Some("rel1"));
        assert_eq!(doc_rel.description(), Some("desc1"));
        assert_eq!(doc_rel.relating_document(), Some("doc1"));
        assert_eq!(doc_rel.related_document(), Some("doc2"));
    }

    #[test]
    fn test_set_fields() {
        let mut doc_rel = DocumentRelationship::new();
        doc_rel.set_name("name1".to_string());
        doc_rel.set_description("desc1".to_string());
        doc_rel.set_relating_document("rdoc1".to_string());
        doc_rel.set_related_document("rdoc2".to_string());

        assert_eq!(doc_rel.name(), Some("name1"));
        assert_eq!(doc_rel.description(), Some("desc1"));
        assert_eq!(doc_rel.relating_document(), Some("rdoc1"));
        assert_eq!(doc_rel.related_document(), Some("rdoc2"));
    }
}
