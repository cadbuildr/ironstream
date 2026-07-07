// FILE: step_basic_document.rs
// occt: StepBasic_Document

/// Represents a STEP Document entity with ID, Name, optional Description, and Kind.
#[derive(Clone, Debug)]
pub struct StepBasicDocument {
    id: String,
    name: String,
    description: Option<String>,
    has_description: bool,
    kind: String, // StepBasic_DocumentType reference
}

impl StepBasicDocument {
    /// Create a new empty StepBasicDocument.
    pub fn new() -> Self {
        StepBasicDocument {
            id: String::new(),
            name: String::new(),
            description: None,
            has_description: false,
            kind: String::new(),
        }
    }

    /// Initialize all fields.
    pub fn init(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
        kind: String,
    ) {
        self.id = id;
        self.name = name;
        self.has_description = has_description;
        if has_description {
            self.description = description;
        } else {
            self.description = None;
        }
        self.kind = kind;
    }

    /// Returns the ID field.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the ID field.
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Returns the Name field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the Name field.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the Description field.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the Description field.
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Returns whether Description is defined.
    pub fn has_description(&self) -> bool {
        self.has_description
    }

    /// Returns the Kind field.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Set the Kind field.
    pub fn set_kind(&mut self, kind: String) {
        self.kind = kind;
    }
}

impl Default for StepBasicDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let doc = StepBasicDocument::new();
        assert_eq!(doc.id(), "");
        assert_eq!(doc.name(), "");
        assert_eq!(doc.description(), None);
        assert!(!doc.has_description());
        assert_eq!(doc.kind(), "");
    }

    #[test]
    fn test_init_with_description() {
        let mut doc = StepBasicDocument::new();
        doc.init(
            "DOC-001".to_string(),
            "My Document".to_string(),
            true,
            Some("A test document".to_string()),
            "TypeA".to_string(),
        );

        assert_eq!(doc.id(), "DOC-001");
        assert_eq!(doc.name(), "My Document");
        assert_eq!(doc.description(), Some("A test document"));
        assert!(doc.has_description());
        assert_eq!(doc.kind(), "TypeA");
    }

    #[test]
    fn test_init_without_description() {
        let mut doc = StepBasicDocument::new();
        doc.init(
            "DOC-002".to_string(),
            "Another Doc".to_string(),
            false,
            Some("This should be ignored".to_string()),
            "TypeB".to_string(),
        );

        assert_eq!(doc.id(), "DOC-002");
        assert_eq!(doc.name(), "Another Doc");
        assert_eq!(doc.description(), None);
        assert!(!doc.has_description());
        assert_eq!(doc.kind(), "TypeB");
    }

    #[test]
    fn test_setters() {
        let mut doc = StepBasicDocument::new();
        doc.set_id("ID123".to_string());
        doc.set_name("NewName".to_string());
        doc.set_description("Desc".to_string());
        doc.set_kind("Kind1".to_string());

        assert_eq!(doc.id(), "ID123");
        assert_eq!(doc.name(), "NewName");
        assert_eq!(doc.description(), Some("Desc"));
        assert_eq!(doc.kind(), "Kind1");
    }
}
