// FILE: iges_basic_external_ref_file.rs
// occt: IGESBasic_ExternalRefFile

/// ExternalRefFile, Type <416> Form <1>
/// Used when entire reference file is to be instanced.
pub struct IgesBasicExternalRefFile {
    file_identifier: String,
}

impl IgesBasicExternalRefFile {
    /// Create a new ExternalRefFile with default values.
    pub fn new() -> Self {
        Self {
            file_identifier: String::new(),
        }
    }

    /// Set the field of the class ExternalRefFile.
    /// - file_ident: External Reference File Identifier
    pub fn init(&mut self, file_ident: String) {
        self.file_identifier = file_ident;
    }

    /// Returns External Reference File Identifier.
    pub fn file_id(&self) -> &str {
        &self.file_identifier
    }
}

impl Default for IgesBasicExternalRefFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let erf = IgesBasicExternalRefFile::new();
        assert_eq!(erf.file_id(), "");
    }

    #[test]
    fn test_init() {
        let mut erf = IgesBasicExternalRefFile::new();
        erf.init("reference_file.igs".to_string());
        assert_eq!(erf.file_id(), "reference_file.igs");
    }

    #[test]
    fn test_default() {
        let erf = IgesBasicExternalRefFile::default();
        assert_eq!(erf.file_id(), "");
    }
}
