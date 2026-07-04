// FILE: iges_draw_general_module.rs
// occt: IGESDraw_GeneralModule

/// General module for IGESDraw operations
pub struct IgesDrawGeneralModule;

impl IgesDrawGeneralModule {
    pub fn new() -> Self {
        IgesDrawGeneralModule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _gm = IgesDrawGeneralModule::new();
    }
}
