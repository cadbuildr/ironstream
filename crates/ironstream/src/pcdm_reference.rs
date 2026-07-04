// FILE: pcdm_reference.rs
// occt: PCDM_Reference

/// Reference to another document with version information
#[derive(Clone, Default)]
pub struct PCDMReference {
    reference_identifier: i32,
    file_name: String,
    document_version: i32,
}

impl PCDMReference {
    /// Create an empty reference
    pub fn new() -> Self {
        PCDMReference {
            reference_identifier: 0,
            file_name: String::new(),
            document_version: 0,
        }
    }

    /// Create a reference with values
    pub fn with_data(reference_id: i32, file_name: &str, version: i32) -> Self {
        PCDMReference {
            reference_identifier: reference_id,
            file_name: file_name.to_string(),
            document_version: version,
        }
    }

    /// Get the reference identifier
    pub fn reference_identifier(&self) -> i32 {
        self.reference_identifier
    }

    /// Get the file name
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Get the document version
    pub fn document_version(&self) -> i32 {
        self.document_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_reference() {
        let ref_obj = PCDMReference::new();
        assert_eq!(ref_obj.reference_identifier(), 0);
        assert_eq!(ref_obj.file_name(), "");
        assert_eq!(ref_obj.document_version(), 0);
    }

    #[test]
    fn test_reference_with_data() {
        let ref_obj = PCDMReference::with_data(1, "doc.xml", 2);
        assert_eq!(ref_obj.reference_identifier(), 1);
        assert_eq!(ref_obj.file_name(), "doc.xml");
        assert_eq!(ref_obj.document_version(), 2);
    }
}
