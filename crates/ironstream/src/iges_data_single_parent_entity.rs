// FILE: iges_data_single_parent_entity.rs
// occt: IGESData_SingleParentEntity

//! Entity with a single parent reference.

#[derive(Clone, Debug)]
pub struct SingleParentEntity {
    parent_id: Option<usize>,
}

impl SingleParentEntity {
    pub fn new() -> Self {
        SingleParentEntity { parent_id: None }
    }

    pub fn set_parent(&mut self, parent_id: usize) {
        self.parent_id = Some(parent_id);
    }

    pub fn parent(&self) -> Option<usize> {
        self.parent_id
    }

    pub fn clear_parent(&mut self) {
        self.parent_id = None;
    }
}

impl Default for SingleParentEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = SingleParentEntity::new();
        assert_eq!(entity.parent(), None);
    }

    #[test]
    fn test_set_parent() {
        let mut entity = SingleParentEntity::new();
        entity.set_parent(42);
        assert_eq!(entity.parent(), Some(42));
    }

    #[test]
    fn test_clear_parent() {
        let mut entity = SingleParentEntity::new();
        entity.set_parent(10);
        assert_eq!(entity.parent(), Some(10));
        entity.clear_parent();
        assert_eq!(entity.parent(), None);
    }
}
