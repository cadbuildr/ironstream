// FILE: step_basic_document_type.rs
// occt: StepBasic_DocumentType

/// Representation of STEP entity DocumentType
#[derive(Clone, Debug)]
pub struct DocumentType {
    product_data_type: Option<String>,
}

impl DocumentType {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            product_data_type: None,
        }
    }

    /// Initialize with product data type
    pub fn init(&mut self, product_data_type: String) {
        self.product_data_type = Some(product_data_type);
    }

    /// Get product data type
    pub fn product_data_type(&self) -> Option<&str> {
        self.product_data_type.as_deref()
    }

    /// Set product data type
    pub fn set_product_data_type(&mut self, product_data_type: String) {
        self.product_data_type = Some(product_data_type);
    }
}

impl Default for DocumentType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let doc_type = DocumentType::new();
        assert!(doc_type.product_data_type().is_none());
    }

    #[test]
    fn test_init() {
        let mut doc_type = DocumentType::new();
        doc_type.init("PDF".to_string());
        assert_eq!(doc_type.product_data_type(), Some("PDF"));
    }

    #[test]
    fn test_set_product_data_type() {
        let mut doc_type = DocumentType::new();
        doc_type.set_product_data_type("XML".to_string());
        assert_eq!(doc_type.product_data_type(), Some("XML"));
    }

    #[test]
    fn test_default() {
        let doc_type = DocumentType::default();
        assert!(doc_type.product_data_type().is_none());
    }
}
