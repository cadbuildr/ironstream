// FILE: iges_basic_external_ref_lib_name.rs
// occt: IGESBasic_ExternalRefLibName

/// ExternalRefLibName, Type <416> Form <4>
/// Used when it is assumed that a copy of the subfigure exists
/// in native form in a library on the receiving system.
pub struct IgesBasicExternalRefLibName {
    library_name: String,
    reference_name: String,
}

impl IgesBasicExternalRefLibName {
    /// Create a new ExternalRefLibName with default values.
    pub fn new() -> Self {
        Self {
            library_name: String::new(),
            reference_name: String::new(),
        }
    }

    /// Set the fields of the class ExternalRefLibName.
    /// - lib_name: Name of library in which ext_name resides
    /// - ext_name: External Reference Entity Symbolic Name
    pub fn init(&mut self, lib_name: String, ext_name: String) {
        self.library_name = lib_name;
        self.reference_name = ext_name;
    }

    /// Returns name of library in which External Reference Entity
    /// Symbolic Name resides.
    pub fn library_name(&self) -> &str {
        &self.library_name
    }

    /// Returns External Reference Entity Symbolic Name.
    pub fn reference_name(&self) -> &str {
        &self.reference_name
    }
}

impl Default for IgesBasicExternalRefLibName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lib = IgesBasicExternalRefLibName::new();
        assert_eq!(lib.library_name(), "");
        assert_eq!(lib.reference_name(), "");
    }

    #[test]
    fn test_init() {
        let mut lib = IgesBasicExternalRefLibName::new();
        lib.init("stdlib".to_string(), "component".to_string());
        assert_eq!(lib.library_name(), "stdlib");
        assert_eq!(lib.reference_name(), "component");
    }

    #[test]
    fn test_default() {
        let lib = IgesBasicExternalRefLibName::default();
        assert_eq!(lib.library_name(), "");
        assert_eq!(lib.reference_name(), "");
    }
}
