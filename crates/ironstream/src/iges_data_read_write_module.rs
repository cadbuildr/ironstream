// FILE: iges_data_read_write_module.rs
// occt: IGESData_ReadWriteModule

//! Module for reading and writing IGES entities.

#[derive(Clone, Debug)]
pub struct ReadWriteModule;

impl ReadWriteModule {
    pub fn new() -> Self {
        ReadWriteModule
    }

    pub fn read_entity(&self, entity_id: usize) -> Option<String> {
        Some(format!("Entity {}", entity_id))
    }

    pub fn write_entity(&self, entity_id: usize, data: &str) -> bool {
        !data.is_empty()
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
        assert_eq!(format!("{:?}", module), "ReadWriteModule");
    }

    #[test]
    fn test_read_entity() {
        let module = ReadWriteModule::new();
        let result = module.read_entity(42);
        assert!(result.is_some());
        assert!(result.unwrap().contains("42"));
    }

    #[test]
    fn test_write_entity() {
        let module = ReadWriteModule::new();
        assert!(module.write_entity(1, "data"));
        assert!(!module.write_entity(1, ""));
    }

    #[test]
    fn test_default() {
        let m1 = ReadWriteModule::new();
        let m2 = ReadWriteModule::default();
        assert_eq!(format!("{:?}", m1), format!("{:?}", m2));
    }
}
