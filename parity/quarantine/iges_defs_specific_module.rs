// FILE: iges_defs_specific_module.rs
// occt: IGESDefs_SpecificModule

//! Specific module for IGES definitions entities.

#[derive(Clone, Debug)]
pub struct SpecificModule;

impl SpecificModule {
    pub fn new() -> Self {
        SpecificModule
    }

    pub fn handle(&self, entity_id: usize) -> bool {
        true
    }
}

impl Default for SpecificModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = SpecificModule::new();
        assert!(module.handle(1));
    }

    #[test]
    fn test_default() {
        let m1 = SpecificModule::new();
        let m2 = SpecificModule::default();
        assert_eq!(format!("{:?}", m1), format!("{:?}", m2));
    }
}
