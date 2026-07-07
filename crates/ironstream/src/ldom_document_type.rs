// FILE: ldom_document_type.rs
// occt: LDOM_DocumentType

/// Represents a DOCTYPE declaration in an XML document.
#[derive(Clone, Default)]
pub struct LDOMDocumentType;

impl LDOMDocumentType {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMDocumentType
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_type_creation() {
        let _dt = LDOMDocumentType::new();
    }
}
