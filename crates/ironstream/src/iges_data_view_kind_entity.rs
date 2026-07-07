// FILE: iges_data_view_kind_entity.rs
// occt: IGESData_ViewKindEntity

//! View kind entity for IGES.

#[derive(Clone, Debug)]
pub struct ViewKindEntity {
    view_id: usize,
}

impl ViewKindEntity {
    pub fn new(view_id: usize) -> Self {
        ViewKindEntity { view_id }
    }

    pub fn view_id(&self) -> usize {
        self.view_id
    }

    pub fn set_view_id(&mut self, view_id: usize) {
        self.view_id = view_id;
    }
}

impl Default for ViewKindEntity {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = ViewKindEntity::new(42);
        assert_eq!(entity.view_id(), 42);
    }

    #[test]
    fn test_set_view_id() {
        let mut entity = ViewKindEntity::new(1);
        entity.set_view_id(100);
        assert_eq!(entity.view_id(), 100);
    }

    #[test]
    fn test_default() {
        let entity = ViewKindEntity::default();
        assert_eq!(entity.view_id(), 0);
    }
}
