// FILE: iges_basic_external_ref_name.rs
// occt: IGESBasic_ExternalRefName

/// ExternalRefName, Type <416> Form <3>
/// Used when it is assumed that a copy of the subfigure
/// exists in native form on the receiving system.
pub struct IgesBasicExternalRefName {
    reference_name: String,
}

impl IgesBasicExternalRefName {
    /// Create a new ExternalRefName with default values.
    pub fn new() -> Self {
        Self {
            reference_name: String::new(),
        }
    }

    /// Set the field of the class ExternalRefName.
    /// - ext_name: External Reference Entity Symbolic Name
    pub fn init(&mut self, ext_name: String) {
        self.reference_name = ext_name;
    }

    /// Returns External Reference Entity Symbolic Name.
    pub fn reference_name(&self) -> &str {
        &self.reference_name
    }
}

impl Default for IgesBasicExternalRefName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = IgesBasicExternalRefName::new();
        assert_eq!(name.reference_name(), "");
    }

    #[test]
    fn test_init() {
        let mut name = IgesBasicExternalRefName::new();
        name.init("external_entity".to_string());
        assert_eq!(name.reference_name(), "external_entity");
    }

    #[test]
    fn test_default() {
        let name = IgesBasicExternalRefName::default();
        assert_eq!(name.reference_name(), "");
    }
}
