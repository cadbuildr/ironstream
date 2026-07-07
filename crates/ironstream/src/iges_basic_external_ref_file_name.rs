// FILE: iges_basic_external_ref_file_name.rs
// occt: IGESBasic_ExternalRefFileName

/// ExternalRefFileName, Type <416> Form <0-2>
/// Used when single definition from the reference file is required
/// or for external logical references where an entity in one file
/// relates to an entity in another file.
pub struct IgesBasicExternalRefFileName {
    file_identifier: String,
    reference_name: String,
    form_for_entity: bool,
}

impl IgesBasicExternalRefFileName {
    /// Create a new ExternalRefFileName with default values.
    pub fn new() -> Self {
        Self {
            file_identifier: String::new(),
            reference_name: String::new(),
            form_for_entity: false,
        }
    }

    /// Set the fields of the class ExternalRefFileName.
    /// - file_ident: External Reference File Identifier
    /// - ext_name: External Reference Entity Symbolic Name
    pub fn init(&mut self, file_ident: String, ext_name: String) {
        self.file_identifier = file_ident;
        self.reference_name = ext_name;
    }

    /// Changes FormNumber to be 2 if mode is true (For Entity)
    /// or 0 if mode is false (For Definition).
    pub fn set_for_entity(&mut self, mode: bool) {
        self.form_for_entity = mode;
    }

    /// Returns External Reference File Identifier.
    pub fn file_id(&self) -> &str {
        &self.file_identifier
    }

    /// Returns External Reference Entity Symbolic Name.
    pub fn reference_name(&self) -> &str {
        &self.reference_name
    }

    /// Returns form type: true if for entity, false if for definition.
    pub fn is_for_entity(&self) -> bool {
        self.form_for_entity
    }
}

impl Default for IgesBasicExternalRefFileName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let erf = IgesBasicExternalRefFileName::new();
        assert_eq!(erf.file_id(), "");
        assert_eq!(erf.reference_name(), "");
        assert!(!erf.is_for_entity());
    }

    #[test]
    fn test_init() {
        let mut erf = IgesBasicExternalRefFileName::new();
        erf.init("file.igs".to_string(), "entity_name".to_string());
        assert_eq!(erf.file_id(), "file.igs");
        assert_eq!(erf.reference_name(), "entity_name");
    }

    #[test]
    fn test_set_for_entity() {
        let mut erf = IgesBasicExternalRefFileName::new();
        assert!(!erf.is_for_entity());
        erf.set_for_entity(true);
        assert!(erf.is_for_entity());
        erf.set_for_entity(false);
        assert!(!erf.is_for_entity());
    }

    #[test]
    fn test_default() {
        let erf = IgesBasicExternalRefFileName::default();
        assert_eq!(erf.file_id(), "");
        assert_eq!(erf.reference_name(), "");
        assert!(!erf.is_for_entity());
    }
}
