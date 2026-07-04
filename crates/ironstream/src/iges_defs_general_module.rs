// FILE: iges_defs_general_module.rs
// occt: IGESDefs_GeneralModule

//! General module for IGES definitions.

#[derive(Clone, Debug)]
pub struct GeneralModule;

impl GeneralModule {
    pub fn new() -> Self {
        GeneralModule
    }

    pub fn process(&self, entity_id: usize) -> bool {
        true
    }
}

impl Default for GeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = GeneralModule::new();
        assert!(module.process(1));
    }

    #[test]
    fn test_default() {
        let m1 = GeneralModule::new();
        let m2 = GeneralModule::default();
        assert_eq!(format!("{:?}", m1), format!("{:?}", m2));
    }
}
