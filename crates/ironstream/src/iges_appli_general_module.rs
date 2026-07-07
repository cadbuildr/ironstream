// FILE: iges_appli_general_module.rs
// occt: IGESAppli_GeneralModule

/// General module for handling IGESAppli entity I/O and checking.
#[derive(Clone, Debug)]
pub struct IgesAppliGeneralModule {
    module_id: i32,
}

impl IgesAppliGeneralModule {
    pub fn new() -> Self {
        Self { module_id: 0 }
    }
}

impl Default for IgesAppliGeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IgesAppliGeneralModule::new();
        assert_eq!(module.module_id, 0);
    }
}
