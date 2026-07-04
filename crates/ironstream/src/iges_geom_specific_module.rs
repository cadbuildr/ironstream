// FILE: iges_geom_specific_module.rs
// occt: IGESGeom_SpecificModule

/// SpecificModule for IGESGeom entities.
pub struct SpecificModule;

impl SpecificModule {
    pub fn new() -> Self {
        SpecificModule
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
        let _ = SpecificModule::new();
    }
}
