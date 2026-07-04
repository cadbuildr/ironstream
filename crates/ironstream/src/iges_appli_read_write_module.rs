// FILE: iges_appli_read_write_module.rs
// occt: IGESAppli_ReadWriteModule

/// Module for reading and writing IGESAppli entities.
#[derive(Clone, Debug)]
pub struct IgesAppliReadWriteModule {
    module_version: i32,
}

impl IgesAppliReadWriteModule {
    pub fn new() -> Self {
        Self { module_version: 1 }
    }

    pub fn module_version(&self) -> i32 {
        self.module_version
    }
}

impl Default for IgesAppliReadWriteModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IgesAppliReadWriteModule::new();
        assert_eq!(module.module_version(), 1);
    }
}
