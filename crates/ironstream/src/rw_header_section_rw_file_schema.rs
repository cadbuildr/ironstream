// FILE: rw_header_section_rw_file_schema.rs
// occt: RWHeaderSection_RWFileSchema

/// Reader/Writer for FileSchema entities
pub struct RWHeaderSection_RWFileSchema;

impl RWHeaderSection_RWFileSchema {
    /// Creates a new FileSchema R/W instance
    pub fn new() -> Self {
        RWHeaderSection_RWFileSchema
    }

    /// Returns the type name handled by this reader/writer
    pub fn type_name() -> &'static str {
        "FileSchema"
    }
}

impl Default for RWHeaderSection_RWFileSchema {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = RWHeaderSection_RWFileSchema::new();
    }

    #[test]
    fn test_type_name() {
        assert_eq!(RWHeaderSection_RWFileSchema::type_name(), "FileSchema");
    }
}
