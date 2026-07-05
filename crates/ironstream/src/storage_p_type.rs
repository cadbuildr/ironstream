// FILE: storage_p_type.rs
// occt: Storage_PType

/// Storage_PType: a persistent type descriptor.
///
/// This is a deprecated OCCT typedef for backward compatibility.
/// In Rust, we model this as a simple struct holding type information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage_PType {
    type_name: String,
    type_id: u64,
}

impl Storage_PType {
    pub fn new(name: String, id: u64) -> Self {
        Self {
            type_name: name,
            type_id: id,
        }
    }

    pub fn name(&self) -> &str {
        &self.type_name
    }

    pub fn id(&self) -> u64 {
        self.type_id
    }

    pub fn set_name(&mut self, name: String) {
        self.type_name = name;
    }

    pub fn set_id(&mut self, id: u64) {
        self.type_id = id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptype_creation() {
        let ptype = Storage_PType::new("MyType".to_string(), 42);
        assert_eq!(ptype.name(), "MyType");
        assert_eq!(ptype.id(), 42);
    }

    #[test]
    fn test_ptype_set_name() {
        let mut ptype = Storage_PType::new("Original".to_string(), 1);
        ptype.set_name("Modified".to_string());
        assert_eq!(ptype.name(), "Modified");
    }

    #[test]
    fn test_ptype_set_id() {
        let mut ptype = Storage_PType::new("Type".to_string(), 10);
        ptype.set_id(99);
        assert_eq!(ptype.id(), 99);
    }

    #[test]
    fn test_ptype_equality() {
        let ptype1 = Storage_PType::new("Same".to_string(), 5);
        let ptype2 = Storage_PType::new("Same".to_string(), 5);
        let ptype3 = Storage_PType::new("Different".to_string(), 5);

        assert_eq!(ptype1, ptype2);
        assert_ne!(ptype1, ptype3);
    }
}
