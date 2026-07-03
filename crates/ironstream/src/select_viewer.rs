// FILE: select_viewer.rs
// occt: SelectMgr_ViewerSelector, SelectMgr_Selection,
//       SelectMgr_SensitiveEntity, SelectMgr_SelectingVolumeManager

/// Selection state of an entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SelectMgrSelectionState {
    #[default]
    Unknown,
    Loaded,
    Activated,
    Deactivated,
    Outdated,
}

/// Selection mode (0 = whole object, 1 = vertex, 2 = edge, 4 = face, etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct SelectionMode(pub i32);

impl SelectionMode {
    pub fn whole() -> Self { Self(0) }
    pub fn vertex() -> Self { Self(1) }
    pub fn edge() -> Self { Self(2) }
    pub fn face() -> Self { Self(4) }
    pub fn shell() -> Self { Self(5) }
    pub fn solid() -> Self { Self(6) }
    pub fn compound() -> Self { Self(7) }
}

/// Sensitive entity (a selectable sub-shape).
/// occt: SelectMgr_SensitiveEntity
#[derive(Clone, Debug)]
pub struct SelectMgrSensitiveEntity {
    pub entity_id: u32,
    pub owner_id: u32,
    pub priority: i32,
    pub is_active: bool,
    pub bounding_box: Option<[[f64; 3]; 2]>,  // [min, max]
}

impl SelectMgrSensitiveEntity {
    pub fn new(entity_id: u32, owner_id: u32, priority: i32) -> Self {
        Self { entity_id, owner_id, priority, is_active: true, bounding_box: None }
    }

    pub fn set_bounding_box(&mut self, min: [f64; 3], max: [f64; 3]) {
        self.bounding_box = Some([min, max]);
    }

    pub fn is_inside_box(&self, query_min: [f64; 3], query_max: [f64; 3]) -> bool {
        if let Some([bmin, bmax]) = self.bounding_box {
            for i in 0..3 {
                if bmax[i] < query_min[i] || bmin[i] > query_max[i] { return false; }
            }
            true
        } else {
            false
        }
    }
}

/// A selection (set of sensitive entities for one mode).
/// occt: SelectMgr_Selection
#[derive(Clone, Debug, Default)]
pub struct SelectMgrSelection {
    pub mode: SelectionMode,
    pub state: SelectMgrSelectionState,
    pub entities: Vec<SelectMgrSensitiveEntity>,
    pub selection_change_idx: u32,
}

impl SelectMgrSelection {
    pub fn new(mode: SelectionMode) -> Self {
        Self { mode, state: SelectMgrSelectionState::Loaded, entities: Vec::new(), selection_change_idx: 0 }
    }

    pub fn add(&mut self, entity: SelectMgrSensitiveEntity) {
        self.entities.push(entity);
        self.selection_change_idx += 1;
    }

    pub fn activate(&mut self) { self.state = SelectMgrSelectionState::Activated; }
    pub fn deactivate(&mut self) { self.state = SelectMgrSelectionState::Deactivated; }
    pub fn is_active(&self) -> bool { self.state == SelectMgrSelectionState::Activated }

    pub fn nb_sensitive(&self) -> usize { self.entities.len() }
}

/// Result of a pick operation.
#[derive(Clone, Debug)]
pub struct SelectMgrPickResult {
    pub owner_id: u32,
    pub entity_id: u32,
    pub priority: i32,
    pub depth: f64,
}

impl SelectMgrPickResult {
    pub fn new(owner_id: u32, entity_id: u32, priority: i32, depth: f64) -> Self {
        Self { owner_id, entity_id, priority, depth }
    }
}

/// Volume for selection queries (box/polyline/point).
/// occt: SelectMgr_SelectingVolumeManager (simplified)
#[derive(Clone, Debug)]
pub struct SelectMgrVolume {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl SelectMgrVolume {
    pub fn from_point(p: [f64; 3], tolerance: f64) -> Self {
        Self {
            min: [p[0] - tolerance, p[1] - tolerance, p[2] - tolerance],
            max: [p[0] + tolerance, p[1] + tolerance, p[2] + tolerance],
        }
    }

    pub fn from_box(min: [f64; 3], max: [f64; 3]) -> Self { Self { min, max } }
}

/// Viewer-side selector: manages selections for all interactive objects.
/// occt: SelectMgr_ViewerSelector
#[derive(Clone, Debug, Default)]
pub struct SelectMgrViewerSelector {
    pub selections: Vec<SelectMgrSelection>,
    pub picked: Vec<SelectMgrPickResult>,
    pub tolerance: f64,
}

impl SelectMgrViewerSelector {
    pub fn new() -> Self { Self { tolerance: 2.0, ..Self::default() } }

    pub fn add_selection(&mut self, sel: SelectMgrSelection) { self.selections.push(sel); }

    pub fn activate(&mut self, mode: SelectionMode) {
        for s in self.selections.iter_mut().filter(|s| s.mode == mode) {
            s.activate();
        }
    }

    pub fn deactivate_all(&mut self) {
        for s in &mut self.selections { s.deactivate(); }
    }

    /// Stub pick: collect entities whose bounding boxes overlap the volume.
    pub fn pick(&mut self, volume: &SelectMgrVolume) {
        self.picked.clear();
        for sel in self.selections.iter().filter(|s| s.is_active()) {
            for entity in &sel.entities {
                if entity.is_active && entity.is_inside_box(volume.min, volume.max) {
                    self.picked.push(SelectMgrPickResult::new(
                        entity.owner_id, entity.entity_id, entity.priority, 0.0,
                    ));
                }
            }
        }
        self.picked.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn nb_picked(&self) -> usize { self.picked.len() }
    pub fn closest_picked(&self) -> Option<&SelectMgrPickResult> { self.picked.first() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_activate_deactivate() {
        let mut sel = SelectMgrSelection::new(SelectionMode::face());
        assert!(!sel.is_active());
        sel.activate();
        assert!(sel.is_active());
        sel.deactivate();
        assert!(!sel.is_active());
    }

    #[test]
    fn sensitive_bounding_box() {
        let mut e = SelectMgrSensitiveEntity::new(1, 100, 5);
        e.set_bounding_box([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert!(e.is_inside_box([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]));
        assert!(!e.is_inside_box([2.0, 2.0, 2.0], [3.0, 3.0, 3.0]));
    }

    #[test]
    fn viewer_selector_pick() {
        let mut vs = SelectMgrViewerSelector::new();
        let mut sel = SelectMgrSelection::new(SelectionMode::whole());
        let mut e = SelectMgrSensitiveEntity::new(1, 10, 5);
        e.set_bounding_box([0.0; 3], [1.0, 1.0, 1.0]);
        sel.add(e);
        sel.activate();
        vs.add_selection(sel);

        let vol = SelectMgrVolume::from_box([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]);
        vs.pick(&vol);
        assert_eq!(vs.nb_picked(), 1);
        assert_eq!(vs.closest_picked().unwrap().owner_id, 10);
    }

    #[test]
    fn pick_outside_volume() {
        let mut vs = SelectMgrViewerSelector::new();
        let mut sel = SelectMgrSelection::new(SelectionMode::whole());
        let mut e = SelectMgrSensitiveEntity::new(1, 10, 0);
        e.set_bounding_box([0.0; 3], [1.0, 1.0, 1.0]);
        sel.add(e);
        sel.activate();
        vs.add_selection(sel);

        let vol = SelectMgrVolume::from_box([5.0; 3], [10.0, 10.0, 10.0]);
        vs.pick(&vol);
        assert_eq!(vs.nb_picked(), 0);
    }

    #[test]
    fn deactivate_prevents_pick() {
        let mut vs = SelectMgrViewerSelector::new();
        let mut sel = SelectMgrSelection::new(SelectionMode::whole());
        let mut e = SelectMgrSensitiveEntity::new(1, 10, 0);
        e.set_bounding_box([0.0; 3], [1.0, 1.0, 1.0]);
        sel.add(e);
        sel.activate();
        vs.add_selection(sel);
        vs.deactivate_all();

        let vol = SelectMgrVolume::from_point([0.5, 0.5, 0.5], 1.0);
        vs.pick(&vol);
        assert_eq!(vs.nb_picked(), 0);
    }

    #[test]
    fn selection_modes() {
        assert_eq!(SelectionMode::whole().0, 0);
        assert_eq!(SelectionMode::face().0, 4);
        assert_eq!(SelectionMode::edge().0, 2);
    }
}
