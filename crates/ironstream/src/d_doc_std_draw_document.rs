// FILE: d_doc_std_draw_document.rs
// occt: DDocStd_DrawDocument

//! Draw document representation.

/// DDocStd_DrawDocument: drawable document.
#[derive(Clone, Debug)]
pub struct DDocStdDrawDocument {
    name: String,
}

impl DDocStdDrawDocument {
    /// Create a new drawable document.
    pub fn new(name: &str) -> Self {
        DDocStdDrawDocument {
            name: name.to_string(),
        }
    }

    /// Get document name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = DDocStdDrawDocument::new("mydoc");
        assert_eq!(doc.name(), "mydoc");
    }
}
