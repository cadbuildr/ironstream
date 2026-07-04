// FILE: iges_dimen_read_write_module.rs
// occt: IGESDimen_ReadWriteModule

pub struct IgesDimen_ReadWriteModule;

impl IgesDimen_ReadWriteModule {
    pub fn new() -> Self {
        IgesDimen_ReadWriteModule
    }
}

impl Default for IgesDimen_ReadWriteModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_module_creation() {
        let _module = IgesDimen_ReadWriteModule::new();
    }
}
