// FILE: iges_draw_specific_module.rs
// occt: IGESDraw_SpecificModule

/// Specific module for IGESDraw
pub struct IgesDrawSpecificModule;

impl IgesDrawSpecificModule {
    pub fn new() -> Self {
        IgesDrawSpecificModule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _sm = IgesDrawSpecificModule::new();
    }
}
