// FILE: select_owner.rs
// occt: SelectMgr_EntityOwner, SelectMgr_SelectableObject
//       SelectMgr_SelectionManager

/// Selection priority.
/// occt: SelectMgr_EntityOwner // priority field
pub type SelectionPriority = i32;

/// Selection state of an entity owner.
/// occt: SelectMgr // (state)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionState {
    NotDetected,
    Detected,
    Selected,
}

impl Default for SelectionState {
    fn default() -> Self { Self::NotDetected }
}

/// An entity owner links a selectable object to a part of it.
/// occt: SelectMgr_EntityOwner
#[derive(Clone, Debug)]
pub struct SelectMgrEntityOwner {
    pub owner_id: u32,
    pub selectable_id: u32,
    pub priority: SelectionPriority,
    pub state: SelectionState,
    pub is_auto_hide_inf: bool,
    pub shape_id: u32,     // the sub-shape or 0 for whole object
}

impl SelectMgrEntityOwner {
    pub fn new(owner_id: u32, selectable_id: u32) -> Self {
        Self {
            owner_id, selectable_id,
            priority: 0,
            state: SelectionState::NotDetected,
            is_auto_hide_inf: false,
            shape_id: 0,
        }
    }

    pub fn with_priority(mut self, p: SelectionPriority) -> Self { self.priority = p; self }
    pub fn with_shape(mut self, s: u32) -> Self { self.shape_id = s; self }

    pub fn set_selected(&mut self) { self.state = SelectionState::Selected; }
    pub fn set_detected(&mut self) { self.state = SelectionState::Detected; }
    pub fn clear_state(&mut self) { self.state = SelectionState::NotDetected; }

    pub fn is_selected(&self) -> bool { self.state == SelectionState::Selected }
    pub fn is_detected(&self) -> bool { self.state == SelectionState::Detected }
    pub fn priority(&self) -> SelectionPriority { self.priority }
    pub fn selectable_id(&self) -> u32 { self.selectable_id }
    pub fn shape_id(&self) -> u32 { self.shape_id }
}

/// A selectable object has a list of selection modes, each with owners.
/// occt: SelectMgr_SelectableObject
#[derive(Clone, Debug)]
pub struct SelectMgrSelectableObject {
    pub object_id: u32,
    pub selection_modes: Vec<i32>,     // active selection modes
    pub owners: Vec<SelectMgrEntityOwner>,
    pub is_selectable: bool,
    pub global_sensitivity: f64,
}

impl SelectMgrSelectableObject {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            selection_modes: Vec::new(),
            owners: Vec::new(),
            is_selectable: true,
            global_sensitivity: 4.0,
        }
    }

    pub fn activate_mode(&mut self, mode: i32) {
        if !self.selection_modes.contains(&mode) {
            self.selection_modes.push(mode);
        }
    }

    pub fn deactivate_mode(&mut self, mode: i32) {
        self.selection_modes.retain(|&m| m != mode);
    }

    pub fn is_mode_active(&self, mode: i32) -> bool {
        self.selection_modes.contains(&mode)
    }

    pub fn add_owner(&mut self, o: SelectMgrEntityOwner) { self.owners.push(o); }

    pub fn nb_owners(&self) -> usize { self.owners.len() }
    pub fn is_selectable(&self) -> bool { self.is_selectable }
    pub fn set_selectable(&mut self, v: bool) { self.is_selectable = v; }
    pub fn set_sensitivity(&mut self, s: f64) { self.global_sensitivity = s.max(0.0); }

    pub fn clear_selection(&mut self) {
        for o in &mut self.owners { o.clear_state(); }
    }

    pub fn selected_owners(&self) -> Vec<&SelectMgrEntityOwner> {
        self.owners.iter().filter(|o| o.is_selected()).collect()
    }
}

/// Selection manager: manages selectable objects and their modes.
/// occt: SelectMgr_SelectionManager
#[derive(Clone, Debug, Default)]
pub struct SelectMgrSelectionMgr {
    pub objects: Vec<SelectMgrSelectableObject>,
}

impl SelectMgrSelectionMgr {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, obj: SelectMgrSelectableObject) {
        self.objects.push(obj);
    }

    pub fn remove(&mut self, object_id: u32) {
        self.objects.retain(|o| o.object_id != object_id);
    }

    pub fn find(&self, object_id: u32) -> Option<&SelectMgrSelectableObject> {
        self.objects.iter().find(|o| o.object_id == object_id)
    }

    pub fn find_mut(&mut self, object_id: u32) -> Option<&mut SelectMgrSelectableObject> {
        self.objects.iter_mut().find(|o| o.object_id == object_id)
    }

    pub fn nb_objects(&self) -> usize { self.objects.len() }

    pub fn activate(&mut self, object_id: u32, mode: i32) {
        if let Some(obj) = self.find_mut(object_id) {
            obj.activate_mode(mode);
        }
    }

    pub fn deactivate(&mut self, object_id: u32, mode: i32) {
        if let Some(obj) = self.find_mut(object_id) {
            obj.deactivate_mode(mode);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_owner_defaults() {
        let o = SelectMgrEntityOwner::new(1, 10);
        assert!(!o.is_selected());
        assert!(!o.is_detected());
        assert_eq!(o.selectable_id(), 10);
    }

    #[test]
    fn entity_owner_state() {
        let mut o = SelectMgrEntityOwner::new(1, 10);
        o.set_selected();
        assert!(o.is_selected());
        o.clear_state();
        assert!(!o.is_selected());
    }

    #[test]
    fn selectable_object_modes() {
        let mut obj = SelectMgrSelectableObject::new(1);
        obj.activate_mode(0);
        obj.activate_mode(1);
        obj.activate_mode(0); // duplicate
        assert!(obj.is_mode_active(0));
        assert!(obj.is_mode_active(1));
        assert_eq!(obj.selection_modes.len(), 2);
        obj.deactivate_mode(0);
        assert!(!obj.is_mode_active(0));
    }

    #[test]
    fn selectable_object_owners() {
        let mut obj = SelectMgrSelectableObject::new(1);
        let mut o1 = SelectMgrEntityOwner::new(1, 1).with_shape(10);
        o1.set_selected();
        obj.add_owner(o1);
        obj.add_owner(SelectMgrEntityOwner::new(2, 1).with_shape(20));
        assert_eq!(obj.nb_owners(), 2);
        assert_eq!(obj.selected_owners().len(), 1);
        obj.clear_selection();
        assert_eq!(obj.selected_owners().len(), 0);
    }

    #[test]
    fn selection_mgr_activate() {
        let mut mgr = SelectMgrSelectionMgr::new();
        mgr.add(SelectMgrSelectableObject::new(1));
        mgr.activate(1, 0);
        mgr.activate(1, 1);
        let obj = mgr.find(1).unwrap();
        assert!(obj.is_mode_active(0));
        assert!(obj.is_mode_active(1));
        mgr.deactivate(1, 0);
        let obj = mgr.find(1).unwrap();
        assert!(!obj.is_mode_active(0));
    }
}
