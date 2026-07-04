// FILE: step_basic_document_product_association.rs
// occt: StepBasic_DocumentProductAssociation

/// Represents a STEP DocumentProductAssociation entity with Name, optional Description,
/// RelatingDocument, and RelatedProduct.
#[derive(Clone, Debug)]
pub struct StepBasicDocumentProductAssociation {
    name: String,
    description: Option<String>,
    has_description: bool,
    relating_document_id: String, // Simplified: using ID string
    related_product_id: String,   // Simplified: using ID string
}

impl StepBasicDocumentProductAssociation {
    /// Create a new empty StepBasicDocumentProductAssociation.
    pub fn new() -> Self {
        StepBasicDocumentProductAssociation {
            name: String::new(),
            description: None,
            has_description: false,
            relating_document_id: String::new(),
            related_product_id: String::new(),
        }
    }

    /// Initialize all fields.
    pub fn init(
        &mut self,
        name: String,
        has_description: bool,
        description: Option<String>,
        relating_document_id: String,
        related_product_id: String,
    ) {
        self.name = name;
        self.has_description = has_description;
        if has_description {
            self.description = description;
        } else {
            self.description = None;
        }
        self.relating_document_id = relating_document_id;
        self.related_product_id = related_product_id;
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

    /// Returns the RelatingDocument ID.
    pub fn relating_document(&self) -> &str {
        &self.relating_document_id
    }

    /// Set the RelatingDocument ID.
    pub fn set_relating_document(&mut self, id: String) {
        self.relating_document_id = id;
    }

    /// Returns the RelatedProduct ID.
    pub fn related_product(&self) -> &str {
        &self.related_product_id
    }

    /// Set the RelatedProduct ID.
    pub fn set_related_product(&mut self, id: String) {
        self.related_product_id = id;
    }
}

impl Default for StepBasicDocumentProductAssociation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assoc = StepBasicDocumentProductAssociation::new();
        assert_eq!(assoc.name(), "");
        assert_eq!(assoc.description(), None);
        assert!(!assoc.has_description());
        assert_eq!(assoc.relating_document(), "");
        assert_eq!(assoc.related_product(), "");
    }

    #[test]
    fn test_init_with_description() {
        let mut assoc = StepBasicDocumentProductAssociation::new();
        assoc.init(
            "Assoc1".to_string(),
            true,
            Some("Describes association".to_string()),
            "DOC-001".to_string(),
            "PROD-001".to_string(),
        );

        assert_eq!(assoc.name(), "Assoc1");
        assert_eq!(assoc.description(), Some("Describes association"));
        assert!(assoc.has_description());
        assert_eq!(assoc.relating_document(), "DOC-001");
        assert_eq!(assoc.related_product(), "PROD-001");
    }

    #[test]
    fn test_init_without_description() {
        let mut assoc = StepBasicDocumentProductAssociation::new();
        assoc.init(
            "Assoc2".to_string(),
            false,
            Some("ignored".to_string()),
            "DOC-002".to_string(),
            "PROD-002".to_string(),
        );

        assert_eq!(assoc.name(), "Assoc2");
        assert_eq!(assoc.description(), None);
        assert!(!assoc.has_description());
    }

    #[test]
    fn test_setters() {
        let mut assoc = StepBasicDocumentProductAssociation::new();
        assoc.set_name("Name".to_string());
        assoc.set_description("Desc".to_string());
        assoc.set_relating_document("DOC".to_string());
        assoc.set_related_product("PROD".to_string());

        assert_eq!(assoc.name(), "Name");
        assert_eq!(assoc.description(), Some("Desc"));
        assert_eq!(assoc.relating_document(), "DOC");
        assert_eq!(assoc.related_product(), "PROD");
    }
}
