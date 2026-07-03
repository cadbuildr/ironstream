// FILE: select3d_owner.rs

// occt: SelectBasics_EntityOwner
// occt: SelectMgr_EntityOwner
/// Represents an owner of a selectable entity, tracking selection state and priority.
/// Models SelectBasics_EntityOwner / SelectMgr_EntityOwner from OCCT.
#[derive(Clone, Debug)]
pub struct EntityOwner {
    // occt: SelectBasics_EntityOwner
    pub id: u32,
    pub priority: i32,
    pub selected: bool,
}

impl EntityOwner {
    // occt: SelectBasics_EntityOwner
    /// Create a new EntityOwner with the given id, default priority 0, not selected.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            priority: 0,
            selected: false,
        }
    }

    // occt: SelectMgr_EntityOwner
    /// Builder-style method to set priority.
    pub fn with_priority(mut self, p: i32) -> Self {
        self.priority = p;
        self
    }

    // occt: SelectMgr_EntityOwner
    /// Set the selected state of this owner.
    pub fn set_selected(&mut self, b: bool) {
        self.selected = b;
    }

    // occt: SelectMgr_EntityOwner
    /// Returns true if this owner is currently selected.
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    // occt: SelectBasics_EntityOwner
    /// Returns the unique identifier of this owner.
    pub fn id(&self) -> u32 {
        self.id
    }

    // occt: SelectBasics_EntityOwner
    /// Returns the selection priority of this owner.
    pub fn priority(&self) -> i32 {
        self.priority
    }

    // occt: SelectMgr_EntityOwner
    /// Returns true if this owner has an associated selectable object.
    /// In this stub, always returns false (no selectable attached).
    pub fn has_selectable() -> bool {
        false
    }
}

// occt: SelectMgr_EntityOwner
/// Manages a collection of EntityOwner instances, supporting add, lookup,
/// and querying of selected owners. Models the selection management layer
/// from SelectMgr_EntityOwner in OCCT.
#[derive(Debug, Default)]
pub struct SelectionManager {
    // occt: SelectMgr_EntityOwner
    pub owners: Vec<EntityOwner>,
}

impl SelectionManager {
    // occt: SelectMgr_EntityOwner
    /// Create a new, empty SelectionManager.
    pub fn new() -> Self {
        Self {
            owners: Vec::new(),
        }
    }

    // occt: SelectMgr_EntityOwner
    /// Add an EntityOwner to the manager.
    pub fn add(&mut self, owner: EntityOwner) {
        self.owners.push(owner);
    }

    // occt: SelectMgr_EntityOwner
    /// Find an EntityOwner by id, returning a reference if found.
    pub fn find(&self, id: u32) -> Option<&EntityOwner> {
        self.owners.iter().find(|o| o.id == id)
    }

    // occt: SelectMgr_EntityOwner
    /// Returns the count of currently selected owners.
    pub fn selected_count(&self) -> usize {
        self.owners.iter().filter(|o| o.selected).count()
    }

    // occt: SelectMgr_EntityOwner
    /// Remove all owners from the manager.
    pub fn clear(&mut self) {
        self.owners.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_owner_defaults() {
        let owner = EntityOwner::new(42);
        assert_eq!(owner.id(), 42);
        assert_eq!(owner.priority(), 0);
        assert!(!owner.is_selected());
    }

    #[test]
    fn test_with_priority() {
        let owner = EntityOwner::new(1).with_priority(5);
        assert_eq!(owner.priority(), 5);
    }

    #[test]
    fn test_set_selected() {
        let mut owner = EntityOwner::new(1);
        owner.set_selected(true);
        assert!(owner.is_selected());
        owner.set_selected(false);
        assert!(!owner.is_selected());
    }

    #[test]
    fn test_has_selectable() {
        assert!(!EntityOwner::has_selectable());
    }

    #[test]
    fn test_selection_manager_add_find() {
        let mut mgr = SelectionManager::new();
        mgr.add(EntityOwner::new(10).with_priority(2));
        mgr.add(EntityOwner::new(20).with_priority(3));
        assert!(mgr.find(10).is_some());
        assert!(mgr.find(99).is_none());
    }

    #[test]
    fn test_selected_count() {
        let mut mgr = SelectionManager::new();
        let mut o1 = EntityOwner::new(1);
        o1.set_selected(true);
        let o2 = EntityOwner::new(2);
        mgr.add(o1);
        mgr.add(o2);
        assert_eq!(mgr.selected_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut mgr = SelectionManager::new();
        mgr.add(EntityOwner::new(1));
        mgr.clear();
        assert!(mgr.owners.is_empty());
    }
}
