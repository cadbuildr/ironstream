// FILE: step_basic_document_usage_constraint.rs
// occt: StepBasic_DocumentUsageConstraint

/// Representation of STEP entity DocumentUsageConstraint
#[derive(Clone, Debug)]
pub struct DocumentUsageConstraint {
    source: Option<String>,
    subject_element: Option<String>,
    subject_element_value: Option<String>,
}

impl DocumentUsageConstraint {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            source: None,
            subject_element: None,
            subject_element_value: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        source: String,
        subject_element: String,
        subject_element_value: String,
    ) {
        self.source = Some(source);
        self.subject_element = Some(subject_element);
        self.subject_element_value = Some(subject_element_value);
    }

    /// Get source
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Set source
    pub fn set_source(&mut self, source: String) {
        self.source = Some(source);
    }

    /// Get subject element
    pub fn subject_element(&self) -> Option<&str> {
        self.subject_element.as_deref()
    }

    /// Set subject element
    pub fn set_subject_element(&mut self, subject_element: String) {
        self.subject_element = Some(subject_element);
    }

    /// Get subject element value
    pub fn subject_element_value(&self) -> Option<&str> {
        self.subject_element_value.as_deref()
    }

    /// Set subject element value
    pub fn set_subject_element_value(&mut self, subject_element_value: String) {
        self.subject_element_value = Some(subject_element_value);
    }
}

impl Default for DocumentUsageConstraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let constraint = DocumentUsageConstraint::new();
        assert!(constraint.source().is_none());
        assert!(constraint.subject_element().is_none());
        assert!(constraint.subject_element_value().is_none());
    }

    #[test]
    fn test_init() {
        let mut constraint = DocumentUsageConstraint::new();
        constraint.init(
            "src1".to_string(),
            "elem1".to_string(),
            "val1".to_string(),
        );
        assert_eq!(constraint.source(), Some("src1"));
        assert_eq!(constraint.subject_element(), Some("elem1"));
        assert_eq!(constraint.subject_element_value(), Some("val1"));
    }

    #[test]
    fn test_set_fields() {
        let mut constraint = DocumentUsageConstraint::new();
        constraint.set_source("src2".to_string());
        constraint.set_subject_element("elem2".to_string());
        constraint.set_subject_element_value("val2".to_string());
        assert_eq!(constraint.source(), Some("src2"));
        assert_eq!(constraint.subject_element(), Some("elem2"));
        assert_eq!(constraint.subject_element_value(), Some("val2"));
    }
}
