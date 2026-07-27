// FILE: select_mgr.rs
// occt: SelectMgr_SensitiveEntitySet
// occt-ref: SelectMgr_Selection, SelectMgr_SelectionManager

/// State of a selection entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionState {
    Unknown,
    On,
    Off,
    Waiting,
}

impl Default for SelectionState {
    fn default() -> Self { Self::Unknown }
}

/// Selection mode for interactive objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SelectionMode(pub i32);

impl SelectionMode {
    pub const SHAPE: Self = Self(0);
    pub const VERTEX: Self = Self(1);
    pub const EDGE: Self = Self(2);
    pub const WIRE: Self = Self(3);
    pub const FACE: Self = Self(4);
    pub const SHELL: Self = Self(5);
    pub const SOLID: Self = Self(6);
    pub const COMPOUND: Self = Self(7);
    pub fn value(self) -> i32 { self.0 }
}

/// A single selectable entity within a selection.
#[derive(Clone, Debug)]
pub struct SensitiveEntity {
    pub entity_id: u32,
    pub mode: SelectionMode,
    pub bvh_idx: i32,
    pub is_active: bool,
}

impl SensitiveEntity {
    pub fn new(entity_id: u32, mode: SelectionMode) -> Self {
        Self { entity_id, mode, bvh_idx: -1, is_active: true }
    }
    pub fn deactivate(&mut self) { self.is_active = false; }
    pub fn activate(&mut self) { self.is_active = true; }
}

// occt-ref: SelectMgr_Selection // — a set of sensitive entities for one mode
#[derive(Clone, Debug)]
pub struct SelectMgrSelection {
    pub mode: SelectionMode,
    pub entities: Vec<SensitiveEntity>,
    pub state: SelectionState,
    pub bvh_update_status: u8,
}

impl SelectMgrSelection {
    pub fn new(mode: SelectionMode) -> Self {
        Self {
            mode,
            entities: Vec::new(),
            state: SelectionState::Off,
            bvh_update_status: 0,
        }
    }

    pub fn add(&mut self, entity: SensitiveEntity) { self.entities.push(entity); }
    pub fn nb_sensitive(&self) -> usize { self.entities.len() }
    pub fn is_empty(&self) -> bool { self.entities.is_empty() }
    pub fn sensitive(&self, i: usize) -> Option<&SensitiveEntity> { self.entities.get(i) }
    pub fn state(&self) -> SelectionState { self.state }
    pub fn set_state(&mut self, s: SelectionState) { self.state = s; }
}

// occt: SelectMgr_SensitiveEntitySet // — stores all active sensitivity entities
#[derive(Clone, Debug, Default)]
pub struct SensitiveEntitySet {
    pub entities: Vec<SensitiveEntity>,
    pub bvh_built: bool,
}

impl SensitiveEntitySet {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, entity: SensitiveEntity) {
        self.bvh_built = false;
        self.entities.push(entity);
    }

    pub fn remove_by_id(&mut self, entity_id: u32) -> bool {
        if let Some(pos) = self.entities.iter().position(|e| e.entity_id == entity_id) {
            self.entities.remove(pos);
            self.bvh_built = false;
            true
        } else {
            false
        }
    }

    pub fn build_bvh(&mut self) { self.bvh_built = true; }
    pub fn size(&self) -> usize { self.entities.len() }
    pub fn is_empty(&self) -> bool { self.entities.is_empty() }
    pub fn clear(&mut self) { self.entities.clear(); self.bvh_built = false; }
}

// occt-ref: SelectMgr_SelectionManager // — manages selections for AIS objects
#[derive(Clone, Debug, Default)]
pub struct SelectMgrSelectionManager {
    /// Map: object_id → list of SelectMgrSelection
    selections: std::collections::HashMap<u32, Vec<SelectMgrSelection>>,
}

impl SelectMgrSelectionManager {
    pub fn new() -> Self { Self::default() }

    pub fn activate(&mut self, object_id: u32, mode: SelectionMode) {
        let list = self.selections.entry(object_id).or_default();
        if let Some(sel) = list.iter_mut().find(|s| s.mode == mode) {
            sel.set_state(SelectionState::On);
        } else {
            let mut sel = SelectMgrSelection::new(mode);
            sel.set_state(SelectionState::On);
            list.push(sel);
        }
    }

    pub fn deactivate(&mut self, object_id: u32, mode: SelectionMode) {
        if let Some(list) = self.selections.get_mut(&object_id) {
            if let Some(sel) = list.iter_mut().find(|s| s.mode == mode) {
                sel.set_state(SelectionState::Off);
            }
        }
    }

    pub fn nb_selected_modes(&self, object_id: u32) -> usize {
        self.selections.get(&object_id).map_or(0, |l| l.len())
    }

    pub fn is_activated(&self, object_id: u32, mode: SelectionMode) -> bool {
        self.selections.get(&object_id).map_or(false, |l| {
            l.iter().any(|s| s.mode == mode && s.state == SelectionState::On)
        })
    }

    pub fn remove(&mut self, object_id: u32) -> bool {
        self.selections.remove(&object_id).is_some()
    }

    pub fn nb_objects(&self) -> usize { self.selections.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_add_sensitive() {
        let mut sel = SelectMgrSelection::new(SelectionMode::FACE);
        sel.add(SensitiveEntity::new(1, SelectionMode::FACE));
        sel.add(SensitiveEntity::new(2, SelectionMode::FACE));
        assert_eq!(sel.nb_sensitive(), 2);
    }

    #[test]
    fn selection_state_toggle() {
        let mut sel = SelectMgrSelection::new(SelectionMode::EDGE);
        assert_eq!(sel.state(), SelectionState::Off);
        sel.set_state(SelectionState::On);
        assert_eq!(sel.state(), SelectionState::On);
    }

    #[test]
    fn sensitive_entity_set_bvh() {
        let mut set = SensitiveEntitySet::new();
        set.add(SensitiveEntity::new(10, SelectionMode::FACE));
        assert!(!set.bvh_built);
        set.build_bvh();
        assert!(set.bvh_built);
        set.remove_by_id(10);
        assert!(set.is_empty());
        assert!(!set.bvh_built);
    }

    #[test]
    fn selection_manager_activate() {
        let mut mgr = SelectMgrSelectionManager::new();
        mgr.activate(1, SelectionMode::FACE);
        assert!(mgr.is_activated(1, SelectionMode::FACE));
        assert!(!mgr.is_activated(1, SelectionMode::EDGE));
    }

    #[test]
    fn selection_manager_deactivate() {
        let mut mgr = SelectMgrSelectionManager::new();
        mgr.activate(2, SelectionMode::SOLID);
        mgr.deactivate(2, SelectionMode::SOLID);
        assert!(!mgr.is_activated(2, SelectionMode::SOLID));
    }

    #[test]
    fn selection_mode_constants() {
        assert_eq!(SelectionMode::FACE.value(), 4);
        assert_eq!(SelectionMode::VERTEX.value(), 1);
        assert_eq!(SelectionMode::SHAPE.value(), 0);
    }
}
