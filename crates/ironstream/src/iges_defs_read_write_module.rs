// FILE: iges_defs_read_write_module.rs
// occt: IGESDefs_ReadWriteModule

//! Read/Write module for IGES definitions entities.

#[derive(Clone, Debug)]
pub struct ReadWriteModule;

impl ReadWriteModule {
    pub fn new() -> Self {
        ReadWriteModule
    }

    pub fn read(&self, entity_id: usize) -> bool {
        true
    }

    pub fn write(&self, entity_id: usize) -> bool {
        true
    }
}

impl Default for ReadWriteModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = ReadWriteModule::new();
        assert!(module.read(1));
        assert!(module.write(1));
    }

    #[test]
    fn test_default() {
        let m1 = ReadWriteModule::new();
        let m2 = ReadWriteModule::default();
        assert_eq!(format!("{:?}", m1), format!("{:?}", m2));
    }
}
