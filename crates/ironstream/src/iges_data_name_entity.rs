// FILE: iges_data_name_entity.rs
// occt: IGESData_NameEntity

//! Name entity for IGES.

#[derive(Clone, Debug)]
pub struct NameEntity {
    name: String,
}

impl NameEntity {
    pub fn new(name: &str) -> Self {
        NameEntity {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Default for NameEntity {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = NameEntity::new("TestName");
        assert_eq!(entity.name(), "TestName");
    }

    #[test]
    fn test_set_name() {
        let mut entity = NameEntity::new("Old");
        entity.set_name("New");
        assert_eq!(entity.name(), "New");
    }

    #[test]
    fn test_default() {
        let entity = NameEntity::default();
        assert_eq!(entity.name(), "");
    }
}
