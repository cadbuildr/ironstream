// FILE: iges_appli_specific_module.rs
// occt: IGESAppli_SpecificModule

/// Module for application-specific entity handling.
#[derive(Clone, Debug)]
pub struct IgesAppliSpecificModule {
    module_id: i32,
}

impl IgesAppliSpecificModule {
    pub fn new() -> Self {
        Self { module_id: 0 }
    }
}

impl Default for IgesAppliSpecificModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IgesAppliSpecificModule::new();
        assert_eq!(module.module_id, 0);
    }
}
