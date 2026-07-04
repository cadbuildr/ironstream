// FILE: rw_header_section_general_module.rs
// occt: RWHeaderSection_GeneralModule

/// General module for reading/writing header section entities
pub struct RWHeaderSection_GeneralModule;

impl RWHeaderSection_GeneralModule {
    /// Creates a new general module instance
    pub fn new() -> Self {
        RWHeaderSection_GeneralModule
    }

    /// Returns whether this module handles a specific entity type
    pub fn handles(_entity_type: &str) -> bool {
        true
    }
}

impl Default for RWHeaderSection_GeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = RWHeaderSection_GeneralModule::new();
    }

    #[test]
    fn test_handles() {
        assert!(RWHeaderSection_GeneralModule::handles("FileName"));
    }
}
