// FILE: ais_local.rs
// occt: AIS_LocalContext (deprecated in OCC7+ but widely used),
//       AIS_LocalStatus, AIS_Selection

use std::collections::HashMap;

/// Selection mode for local context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AisSelMode {
    Shape = 0,
    Vertex = 1,
    Edge = 2,
    Wire = 3,
    Face = 4,
    Shell = 5,
    Solid = 6,
    CompSolid = 7,
    Compound = 8,
}

impl Default for AisSelMode {
    fn default() -> Self { Self::Shape }
}

/// Activity state of a local selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisLocalState {
    Inactive,
    Active,
    Highlighted,
    Selected,
}

impl Default for AisLocalState {
    fn default() -> Self { Self::Inactive }
}

// occt: AIS_LocalStatus — per-object selection status in a local context
#[derive(Clone, Debug)]
pub struct AisLocalStatus {
    pub is_temporary: bool,
    pub display_mode: i32,
    pub selection_modes: Vec<AisSelMode>,
    pub state: AisLocalState,
    pub decomposed: bool,
    pub sub_int_computed: bool,
}

impl Default for AisLocalStatus {
    fn default() -> Self {
        Self {
            is_temporary: false,
            display_mode: 0,
            selection_modes: Vec::new(),
            state: AisLocalState::Inactive,
            decomposed: false,
            sub_int_computed: false,
        }
    }
}

impl AisLocalStatus {
    pub fn new(is_temporary: bool, display_mode: i32) -> Self {
        Self { is_temporary, display_mode, ..Default::default() }
    }

    pub fn add_selection_mode(&mut self, mode: AisSelMode) {
        if !self.selection_modes.contains(&mode) {
            self.selection_modes.push(mode);
        }
    }

    pub fn remove_selection_mode(&mut self, mode: AisSelMode) {
        self.selection_modes.retain(|&m| m != mode);
    }

    pub fn is_activated(&self, mode: AisSelMode) -> bool {
        self.selection_modes.contains(&mode)
    }

    pub fn nb_selection_modes(&self) -> usize { self.selection_modes.len() }
    pub fn is_selected(&self) -> bool { self.state == AisLocalState::Selected }
    pub fn set_state(&mut self, s: AisLocalState) { self.state = s; }
}

/// Result of a pick/detect operation.
#[derive(Clone, Debug)]
pub struct AisPickResult {
    pub object_id: u32,
    pub sub_shape_id: u32,
    pub depth: f64,
    pub mode: AisSelMode,
}

impl AisPickResult {
    pub fn new(object_id: u32, sub_shape_id: u32, depth: f64, mode: AisSelMode) -> Self {
        Self { object_id, sub_shape_id, depth, mode }
    }
}

// occt: AIS_Selection — a named selection list
#[derive(Clone, Debug)]
pub struct AisSelection {
    pub name: String,
    pub objects: Vec<u32>,
    pub is_current: bool,
}

impl AisSelection {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned(), objects: Vec::new(), is_current: false }
    }

    pub fn select(&mut self, obj_id: u32) {
        if !self.objects.contains(&obj_id) {
            self.objects.push(obj_id);
        }
    }

    pub fn deselect(&mut self, obj_id: u32) {
        self.objects.retain(|&o| o != obj_id);
    }

    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn nb_selected(&self) -> usize { self.objects.len() }
    pub fn is_selected(&self, obj_id: u32) -> bool { self.objects.contains(&obj_id) }
    pub fn objects(&self) -> &[u32] { &self.objects }
}

// occt: AIS_LocalContext — manages local selections on a subset of objects
#[derive(Clone, Debug, Default)]
pub struct AisLocalContext {
    pub is_open: bool,
    pub index: i32,
    pub statuses: HashMap<u32, AisLocalStatus>,
    pub pick_results: Vec<AisPickResult>,
    pub selections: Vec<AisSelection>,
    pub current_selection_idx: Option<usize>,
}

impl AisLocalContext {
    pub fn new(index: i32) -> Self {
        Self { index, is_open: true, ..Default::default() }
    }

    pub fn open(&mut self) { self.is_open = true; }
    pub fn close(&mut self) { self.is_open = false; }
    pub fn is_open(&self) -> bool { self.is_open }

    pub fn load_object(&mut self, obj_id: u32, is_temporary: bool) {
        self.statuses.insert(obj_id, AisLocalStatus::new(is_temporary, 0));
    }

    pub fn activate_mode(&mut self, obj_id: u32, mode: AisSelMode) {
        if let Some(status) = self.statuses.get_mut(&obj_id) {
            status.add_selection_mode(mode);
            status.state = AisLocalState::Active;
        }
    }

    pub fn deactivate_mode(&mut self, obj_id: u32, mode: AisSelMode) {
        if let Some(status) = self.statuses.get_mut(&obj_id) {
            status.remove_selection_mode(mode);
        }
    }

    pub fn status(&self, obj_id: u32) -> Option<&AisLocalStatus> {
        self.statuses.get(&obj_id)
    }

    pub fn add_pick_result(&mut self, result: AisPickResult) {
        self.pick_results.push(result);
    }

    pub fn nb_detected(&self) -> usize { self.pick_results.len() }
    pub fn clear_pick_results(&mut self) { self.pick_results.clear(); }

    pub fn detected_object(&self, i: usize) -> Option<&AisPickResult> {
        self.pick_results.get(i)
    }

    pub fn add_selection(&mut self, sel: AisSelection) {
        self.selections.push(sel);
    }

    pub fn nb_loaded(&self) -> usize { self.statuses.len() }
    pub fn has_object(&self, obj_id: u32) -> bool { self.statuses.contains_key(&obj_id) }

    pub fn select_detected(&mut self) {
        for r in &self.pick_results {
            if let Some(status) = self.statuses.get_mut(&r.object_id) {
                status.state = AisLocalState::Selected;
            }
        }
    }

    pub fn nb_selected_objects(&self) -> usize {
        self.statuses.values().filter(|s| s.is_selected()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_status_selection_modes() {
        let mut s = AisLocalStatus::new(false, 0);
        s.add_selection_mode(AisSelMode::Face);
        s.add_selection_mode(AisSelMode::Edge);
        s.add_selection_mode(AisSelMode::Face); // duplicate
        assert_eq!(s.nb_selection_modes(), 2);
        assert!(s.is_activated(AisSelMode::Face));
        s.remove_selection_mode(AisSelMode::Face);
        assert!(!s.is_activated(AisSelMode::Face));
    }

    #[test]
    fn ais_selection_basic() {
        let mut sel = AisSelection::new("current");
        sel.select(10);
        sel.select(20);
        sel.select(10); // duplicate
        assert_eq!(sel.nb_selected(), 2);
        assert!(sel.is_selected(10));
        sel.deselect(10);
        assert!(!sel.is_selected(10));
    }

    #[test]
    fn local_context_load_and_activate() {
        let mut ctx = AisLocalContext::new(1);
        ctx.load_object(100, false);
        ctx.activate_mode(100, AisSelMode::Edge);
        assert!(ctx.has_object(100));
        let s = ctx.status(100).unwrap();
        assert!(s.is_activated(AisSelMode::Edge));
        assert_eq!(s.state, AisLocalState::Active);
    }

    #[test]
    fn local_context_pick_results() {
        let mut ctx = AisLocalContext::new(1);
        ctx.add_pick_result(AisPickResult::new(1, 5, 10.0, AisSelMode::Face));
        ctx.add_pick_result(AisPickResult::new(2, 3, 5.0, AisSelMode::Edge));
        assert_eq!(ctx.nb_detected(), 2);
        assert_eq!(ctx.detected_object(0).unwrap().object_id, 1);
        ctx.clear_pick_results();
        assert_eq!(ctx.nb_detected(), 0);
    }

    #[test]
    fn local_context_select_detected() {
        let mut ctx = AisLocalContext::new(1);
        ctx.load_object(50, false);
        ctx.activate_mode(50, AisSelMode::Shape);
        ctx.add_pick_result(AisPickResult::new(50, 0, 1.0, AisSelMode::Shape));
        ctx.select_detected();
        assert!(ctx.status(50).unwrap().is_selected());
        assert_eq!(ctx.nb_selected_objects(), 1);
    }

    #[test]
    fn local_context_open_close() {
        let mut ctx = AisLocalContext::new(0);
        assert!(ctx.is_open());
        ctx.close();
        assert!(!ctx.is_open());
        ctx.open();
        assert!(ctx.is_open());
    }
}
