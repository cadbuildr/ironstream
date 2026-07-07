// FILE: rw_header_section_rw_file_description.rs
// occt: RWHeaderSection_RWFileDescription

/// Reader/Writer for FileDescription entities
pub struct RWHeaderSection_RWFileDescription;

impl RWHeaderSection_RWFileDescription {
    /// Creates a new FileDescription R/W instance
    pub fn new() -> Self {
        RWHeaderSection_RWFileDescription
    }

    /// Returns the type name handled by this reader/writer
    pub fn type_name() -> &'static str {
        "FileDescription"
    }
}

impl Default for RWHeaderSection_RWFileDescription {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = RWHeaderSection_RWFileDescription::new();
    }

    #[test]
    fn test_type_name() {
        assert_eq!(
            RWHeaderSection_RWFileDescription::type_name(),
            "FileDescription"
        );
    }
}
