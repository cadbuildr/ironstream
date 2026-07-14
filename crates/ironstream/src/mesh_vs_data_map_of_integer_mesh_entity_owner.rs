// FILE: mesh_vs_data_map_of_integer_mesh_entity_owner.rs
// occt: MeshVS_DataMapOfIntegerMeshEntityOwner
// occt-ref: MeshVS_DataMapIteratorOfDataMapOfIntegerMeshEntityOwner

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// MeshVS_MeshEntityOwner represents the owner of a mesh entity in visualization.
#[derive(Clone, Debug)]
pub struct MeshVsMeshEntityOwner {
    id: i32,
    selected: bool,
    highlighted: bool,
}

impl MeshVsMeshEntityOwner {
    pub fn new(id: i32) -> Self {
        MeshVsMeshEntityOwner {
            id,
            selected: false,
            highlighted: false,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_highlighted(&mut self, highlighted: bool) {
        self.highlighted = highlighted;
    }

    pub fn is_highlighted(&self) -> bool {
        self.highlighted
    }
}

/// A handle/reference-counted wrapper for MeshVS_MeshEntityOwner.
pub type MeshVsMeshEntityOwnerHandle = Rc<RefCell<MeshVsMeshEntityOwner>>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<int, opencascade::handle<MeshVS_MeshEntityOwner>>`
pub type MeshVsDataMapOfIntegerMeshEntityOwner = HashMap<i32, MeshVsMeshEntityOwnerHandle>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<int, opencascade::handle<MeshVS_MeshEntityOwner>>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfIntegerMeshEntityOwner =
    std::collections::hash_map::IntoIter<i32, MeshVsMeshEntityOwnerHandle>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_owner_creation() {
        let owner = MeshVsMeshEntityOwner::new(42);
        assert_eq!(owner.id(), 42);
        assert!(!owner.is_selected());
        assert!(!owner.is_highlighted());
    }

    #[test]
    fn test_entity_owner_selection() {
        let mut owner = MeshVsMeshEntityOwner::new(1);
        assert!(!owner.is_selected());

        owner.set_selected(true);
        assert!(owner.is_selected());

        owner.set_selected(false);
        assert!(!owner.is_selected());
    }

    #[test]
    fn test_entity_owner_highlight() {
        let mut owner = MeshVsMeshEntityOwner::new(1);
        assert!(!owner.is_highlighted());

        owner.set_highlighted(true);
        assert!(owner.is_highlighted());

        owner.set_highlighted(false);
        assert!(!owner.is_highlighted());
    }

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfIntegerMeshEntityOwner = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfIntegerMeshEntityOwner = HashMap::new();

        let owner1 = Rc::new(RefCell::new(MeshVsMeshEntityOwner::new(1)));
        let owner2 = Rc::new(RefCell::new(MeshVsMeshEntityOwner::new(2)));

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
        let mut map: MeshVsDataMapOfIntegerMeshEntityOwner = HashMap::new();
        assert_eq!(map.len(), 0);

        let owner = Rc::new(RefCell::new(MeshVsMeshEntityOwner::new(1)));
        map.insert(1, owner.clone());
        assert_eq!(map.len(), 1);

        map.insert(2, owner.clone());
        assert_eq!(map.len(), 2);

        map.remove(&1);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfIntegerMeshEntityOwner = HashMap::new();

        let owner1 = Rc::new(RefCell::new(MeshVsMeshEntityOwner::new(1)));
        let owner2 = Rc::new(RefCell::new(MeshVsMeshEntityOwner::new(2)));

        map.insert(10, owner1.clone());
        map.insert(20, owner2.clone());

        let collected: Vec<(i32, MeshVsMeshEntityOwnerHandle)> = map.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}
