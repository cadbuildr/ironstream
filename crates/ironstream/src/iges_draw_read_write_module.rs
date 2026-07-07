// FILE: iges_draw_read_write_module.rs
// occt: IGESDraw_ReadWriteModule

/// Read-write module for IGESDraw
pub struct IgesDrawReadWriteModule;

impl IgesDrawReadWriteModule {
    pub fn new() -> Self {
        IgesDrawReadWriteModule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _rwm = IgesDrawReadWriteModule::new();
    }
}
