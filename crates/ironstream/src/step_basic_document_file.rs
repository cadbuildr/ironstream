// FILE: step_basic_document_file.rs
// occt: StepBasic_DocumentFile

/// Represents a STEP DocumentFile entity, which extends Document with a CharacterizedObject.
#[derive(Clone, Debug)]
pub struct StepBasicDocumentFile {
    // Document fields
    id: String,
    name: String,
    description: Option<String>,
    has_description: bool,
    kind: String,
    // DocumentFile specific
    characterized_object_name: String,
    characterized_object_description: Option<String>,
    characterized_object_has_description: bool,
}

impl StepBasicDocumentFile {
    /// Create a new empty StepBasicDocumentFile.
    pub fn new() -> Self {
        StepBasicDocumentFile {
            id: String::new(),
            name: String::new(),
            description: None,
            has_description: false,
            kind: String::new(),
            characterized_object_name: String::new(),
            characterized_object_description: None,
            characterized_object_has_description: false,
        }
    }

    /// Initialize all fields for both Document and CharacterizedObject parts.
    pub fn init(
        &mut self,
        document_id: String,
        document_name: String,
        has_document_description: bool,
        document_description: Option<String>,
        document_kind: String,
        characterized_object_name: String,
        has_characterized_object_description: bool,
        characterized_object_description: Option<String>,
    ) {
        // Initialize Document fields
        self.id = document_id;
        self.name = document_name;
        self.has_description = has_document_description;
        if has_document_description {
            self.description = document_description;
        } else {
            self.description = None;
        }
        self.kind = document_kind;

        // Initialize CharacterizedObject fields
        self.characterized_object_name = characterized_object_name;
        self.characterized_object_has_description = has_characterized_object_description;
        if has_characterized_object_description {
            self.characterized_object_description = characterized_object_description;
        } else {
            self.characterized_object_description = None;
        }
    }

    /// Returns the Document ID field.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the Document ID field.
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Returns the Document Name field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the Document Name field.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the Document Description field.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the Document Description field.
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Returns whether Document Description is defined.
    pub fn has_description(&self) -> bool {
        self.has_description
    }

    /// Returns the Document Kind field.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Set the Document Kind field.
    pub fn set_kind(&mut self, kind: String) {
        self.kind = kind;
    }

    /// Returns the CharacterizedObject Name.
    pub fn characterized_object_name(&self) -> &str {
        &self.characterized_object_name
    }

    /// Set the CharacterizedObject Name.
    pub fn set_characterized_object_name(&mut self, name: String) {
        self.characterized_object_name = name;
    }

    /// Returns the CharacterizedObject Description.
    pub fn characterized_object_description(&self) -> Option<&str> {
        self.characterized_object_description.as_deref()
    }

    /// Set the CharacterizedObject Description.
    pub fn set_characterized_object_description(&mut self, description: String) {
        self.characterized_object_description = Some(description);
    }

    /// Returns whether CharacterizedObject Description is defined.
    pub fn has_characterized_object_description(&self) -> bool {
        self.characterized_object_has_description
    }
}

impl Default for StepBasicDocumentFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let doc = StepBasicDocumentFile::new();
        assert_eq!(doc.id(), "");
        assert_eq!(doc.name(), "");
        assert_eq!(doc.description(), None);
        assert!(!doc.has_description());
        assert_eq!(doc.kind(), "");
        assert_eq!(doc.characterized_object_name(), "");
        assert_eq!(doc.characterized_object_description(), None);
    }

    #[test]
    fn test_init_full() {
        let mut doc = StepBasicDocumentFile::new();
        doc.init(
            "FILE-001".to_string(),
            "Document".to_string(),
            true,
            Some("Doc desc".to_string()),
            "TypeA".to_string(),
            "ObjectName".to_string(),
            true,
            Some("Object desc".to_string()),
        );

        assert_eq!(doc.id(), "FILE-001");
        assert_eq!(doc.name(), "Document");
        assert_eq!(doc.description(), Some("Doc desc"));
        assert!(doc.has_description());
        assert_eq!(doc.kind(), "TypeA");
        assert_eq!(doc.characterized_object_name(), "ObjectName");
        assert_eq!(doc.characterized_object_description(), Some("Object desc"));
        assert!(doc.has_characterized_object_description());
    }

    #[test]
    fn test_init_partial_descriptions() {
        let mut doc = StepBasicDocumentFile::new();
        doc.init(
            "FILE-002".to_string(),
            "Document2".to_string(),
            false,
            Some("ignored".to_string()),
            "TypeB".to_string(),
            "Obj2".to_string(),
            false,
            Some("ignored".to_string()),
        );

        assert_eq!(doc.description(), None);
        assert!(!doc.has_description());
        assert_eq!(doc.characterized_object_description(), None);
        assert!(!doc.has_characterized_object_description());
    }

    #[test]
    fn test_setters() {
        let mut doc = StepBasicDocumentFile::new();
        doc.set_id("ID".to_string());
        doc.set_name("Name".to_string());
        doc.set_description("Desc".to_string());
        doc.set_kind("Kind".to_string());
        doc.set_characterized_object_name("ObjName".to_string());
        doc.set_characterized_object_description("ObjDesc".to_string());

        assert_eq!(doc.id(), "ID");
        assert_eq!(doc.name(), "Name");
        assert_eq!(doc.description(), Some("Desc"));
        assert_eq!(doc.kind(), "Kind");
        assert_eq!(doc.characterized_object_name(), "ObjName");
        assert_eq!(doc.characterized_object_description(), Some("ObjDesc"));
    }
}
