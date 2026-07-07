// FILE: mesh_vs_data_map_of_integer_owner.rs
// occt: MeshVS_DataMapOfIntegerOwner, MeshVS_DataMapIteratorOfDataMapOfIntegerOwner

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// SelectMgr_EntityOwner represents an entity owner in selection management.
#[derive(Clone, Debug)]
pub struct SelectMgrEntityOwner {
    id: i32,
    priority: i32,
    selected: bool,
}

impl SelectMgrEntityOwner {
    pub fn new(id: i32) -> Self {
        SelectMgrEntityOwner {
            id,
            priority: 0,
            selected: false,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

/// A handle/reference-counted wrapper for SelectMgr_EntityOwner.
pub type SelectMgrEntityOwnerHandle = Rc<RefCell<SelectMgrEntityOwner>>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<int, opencascade::handle<SelectMgr_EntityOwner>>`
pub type MeshVsDataMapOfIntegerOwner = HashMap<i32, SelectMgrEntityOwnerHandle>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<int, opencascade::handle<SelectMgr_EntityOwner>>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfIntegerOwner =
    std::collections::hash_map::IntoIter<i32, SelectMgrEntityOwnerHandle>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_owner_creation() {
        let owner = SelectMgrEntityOwner::new(42);
        assert_eq!(owner.id(), 42);
        assert_eq!(owner.priority(), 0);
        assert!(!owner.is_selected());
    }

    #[test]
    fn test_entity_owner_priority() {
        let mut owner = SelectMgrEntityOwner::new(1);
        assert_eq!(owner.priority(), 0);

        owner.set_priority(5);
        assert_eq!(owner.priority(), 5);

        owner.set_priority(-1);
        assert_eq!(owner.priority(), -1);
    }

    #[test]
    fn test_entity_owner_selection() {
        let mut owner = SelectMgrEntityOwner::new(1);
        assert!(!owner.is_selected());

        owner.set_selected(true);
        assert!(owner.is_selected());

        owner.set_selected(false);
        assert!(!owner.is_selected());
    }

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfIntegerOwner = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfIntegerOwner = HashMap::new();

        let owner1 = Rc::new(RefCell::new(SelectMgrEntityOwner::new(1)));
        let owner2 = Rc::new(RefCell::new(SelectMgrEntityOwner::new(2)));

        map.insert(10, owner1.clone());
        map.insert(20, owner2.clone());

        assert!(map.contains_key(&10));
        assert!(map.contains_key(&20));
        assert!(!map.contains_key(&30));

        let retrieved = map.get(&10).unwrap();
        assert_eq!(retrieved.borrow().id(), 1);
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MeshVsDataMapOfIntegerOwner = HashMap::new();
        assert_eq!(map.len(), 0);

        let owner = Rc::new(RefCell::new(SelectMgrEntityOwner::new(1)));
        map.insert(1, owner.clone());
        assert_eq!(map.len(), 1);

        map.insert(2, owner.clone());
        assert_eq!(map.len(), 2);

        map.remove(&1);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfIntegerOwner = HashMap::new();

        let owner1 = Rc::new(RefCell::new(SelectMgrEntityOwner::new(1)));
        let owner2 = Rc::new(RefCell::new(SelectMgrEntityOwner::new(2)));

        map.insert(10, owner1.clone());
        map.insert(20, owner2.clone());

        let collected: Vec<(i32, SelectMgrEntityOwnerHandle)> = map.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}
