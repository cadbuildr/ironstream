// FILE: iges_dimen_general_module.rs
// occt: IGESDimen_GeneralModule

pub struct IgesDimen_GeneralModule;

impl IgesDimen_GeneralModule {
    pub fn new() -> Self {
        IgesDimen_GeneralModule
    }
}

impl Default for IgesDimen_GeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_module_creation() {
        let _module = IgesDimen_GeneralModule::new();
    }
}
