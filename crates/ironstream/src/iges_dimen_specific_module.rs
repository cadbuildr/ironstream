// FILE: iges_dimen_specific_module.rs
// occt: IGESDimen_SpecificModule

pub struct IgesDimen_SpecificModule;

impl IgesDimen_SpecificModule {
    pub fn new() -> Self {
        IgesDimen_SpecificModule
    }
}

impl Default for IgesDimen_SpecificModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_module_creation() {
        let _module = IgesDimen_SpecificModule::new();
    }
}
