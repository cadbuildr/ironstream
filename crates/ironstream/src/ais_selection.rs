// FILE: ais_selection.rs
// occt: AIS_Selection, AIS_SelectionScheme, AIS_NListOfEntityOwner
//       SelectMgr_EntityOwner, SelectMgr_Selection, SelectMgr_IndexedMapOfOwner

/// Selection scheme (how new picks combine with existing selection).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionScheme {
    Replace,   // replaces current selection
    Add,       // adds to current selection
    Remove,    // removes from current selection
    XOR,       // toggles — adds if not selected, removes if selected
    Clear,     // clears selection
}

/// Entity owner — associates a selectable entity with its presentation object.
#[derive(Clone, Debug)]
pub struct EntityOwner {
    pub owner_id: u32,
    pub selectable_id: u32,
    pub priority: i32,
    pub is_selected: bool,
    pub has_z_layer: bool,
    pub location: [[f64; 4]; 4],   // placement transform
}

impl Default for SelectionScheme {
    fn default() -> Self { Self::Replace }
}

impl EntityOwner {
    pub fn new(owner_id: u32, selectable_id: u32) -> Self {
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Self { owner_id, selectable_id, priority: 0, is_selected: false, has_z_layer: false, location: identity }
    }

    pub fn with_priority(mut self, p: i32) -> Self { self.priority = p; self }

    pub fn select(&mut self) { self.is_selected = true; }
    pub fn deselect(&mut self) { self.is_selected = false; }
    pub fn toggle(&mut self) { self.is_selected = !self.is_selected; }
}

/// A single selection mode bucket — typically one for each "mode" (e.g., 0=whole shape, 1=vertex, 2=edge).
#[derive(Clone, Debug)]
pub struct SelectionMode {
    pub mode: i32,
    pub owners: Vec<EntityOwner>,
    pub is_active: bool,
    pub sensitivity: u32,
}

impl SelectionMode {
    pub fn new(mode: i32) -> Self {
        Self { mode, owners: Vec::new(), is_active: true, sensitivity: 2 }
    }

    pub fn add_owner(&mut self, o: EntityOwner) { self.owners.push(o); }
    pub fn nb_owners(&self) -> usize { self.owners.len() }
    pub fn is_empty(&self) -> bool { self.owners.is_empty() }
}

// occt: SelectMgr_Selection
#[derive(Clone, Debug, Default)]
pub struct SelectMgrSelection {
    pub modes: Vec<SelectionMode>,
    pub is_update_needed: bool,
}

impl SelectMgrSelection {
    pub fn new() -> Self { Self::default() }

    pub fn add_mode(&mut self, mode: SelectionMode) {
        self.modes.push(mode);
    }

    pub fn mode(&self, m: i32) -> Option<&SelectionMode> {
        self.modes.iter().find(|x| x.mode == m)
    }

    pub fn mode_mut(&mut self, m: i32) -> Option<&mut SelectionMode> {
        self.modes.iter_mut().find(|x| x.mode == m)
    }

    pub fn activate_mode(&mut self, m: i32) {
        if let Some(mode) = self.mode_mut(m) { mode.is_active = true; }
    }

    pub fn deactivate_mode(&mut self, m: i32) {
        if let Some(mode) = self.mode_mut(m) { mode.is_active = false; }
    }

    pub fn nb_modes(&self) -> usize { self.modes.len() }
    pub fn total_owners(&self) -> usize { self.modes.iter().map(|m| m.nb_owners()).sum() }
}

// occt: AIS_Selection
#[derive(Clone, Debug, Default)]
pub struct AisSelection {
    pub selected: Vec<EntityOwner>,
    pub scheme: SelectionScheme,
}

impl AisSelection {
    pub fn new() -> Self { Self { selected: Vec::new(), scheme: SelectionScheme::Replace } }

    pub fn select(&mut self, owner: EntityOwner, scheme: SelectionScheme) {
        match scheme {
            SelectionScheme::Replace => {
                for o in &mut self.selected { o.deselect(); }
                self.selected.clear();
                let mut o = owner;
                o.select();
                self.selected.push(o);
            }
            SelectionScheme::Add => {
                let already = self.selected.iter().any(|o| o.owner_id == owner.owner_id);
                if !already {
                    let mut o = owner;
                    o.select();
                    self.selected.push(o);
                }
            }
            SelectionScheme::Remove => {
                self.selected.retain(|o| o.owner_id != owner.owner_id);
            }
            SelectionScheme::XOR => {
                if self.selected.iter().any(|o| o.owner_id == owner.owner_id) {
                    self.selected.retain(|o| o.owner_id != owner.owner_id);
                } else {
                    let mut o = owner;
                    o.select();
                    self.selected.push(o);
                }
            }
            SelectionScheme::Clear => {
                for o in &mut self.selected { o.deselect(); }
                self.selected.clear();
            }
        }
    }

    pub fn clear(&mut self) {
        for o in &mut self.selected { o.deselect(); }
        self.selected.clear();
    }

    pub fn is_selected(&self, owner_id: u32) -> bool {
        self.selected.iter().any(|o| o.owner_id == owner_id)
    }

    pub fn nb_selected(&self) -> usize { self.selected.len() }
    pub fn is_empty(&self) -> bool { self.selected.is_empty() }

    pub fn selected_owners(&self) -> Vec<&EntityOwner> {
        self.selected.iter().filter(|o| o.is_selected).collect()
    }
}

// occt-ref: SelectMgr_IndexedMapOfOwner
#[derive(Clone, Debug, Default)]
pub struct IndexedMapOfOwner {
    pub entries: Vec<EntityOwner>,
}

impl IndexedMapOfOwner {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, o: EntityOwner) -> u32 {
        let idx = self.entries.len() as u32 + 1;
        self.entries.push(o);
        idx
    }

    pub fn find_from_index(&self, idx: u32) -> Option<&EntityOwner> {
        self.entries.get((idx as usize).saturating_sub(1))
    }

    pub fn find_index(&self, owner_id: u32) -> Option<u32> {
        self.entries.iter().position(|o| o.owner_id == owner_id).map(|i| i as u32 + 1)
    }

    pub fn extent(&self) -> usize { self.entries.len() }
    pub fn contains(&self, owner_id: u32) -> bool { self.entries.iter().any(|o| o.owner_id == owner_id) }
    pub fn clear(&mut self) { self.entries.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_replace() {
        let mut sel = AisSelection::new();
        sel.select(EntityOwner::new(1, 0), SelectionScheme::Replace);
        sel.select(EntityOwner::new(2, 0), SelectionScheme::Replace);
        assert_eq!(sel.nb_selected(), 1);
        assert!(sel.is_selected(2));
        assert!(!sel.is_selected(1));
    }

    #[test]
    fn selection_add() {
        let mut sel = AisSelection::new();
        sel.select(EntityOwner::new(1, 0), SelectionScheme::Add);
        sel.select(EntityOwner::new(2, 0), SelectionScheme::Add);
        sel.select(EntityOwner::new(1, 0), SelectionScheme::Add); // duplicate
        assert_eq!(sel.nb_selected(), 2);
    }

    #[test]
    fn selection_xor() {
        let mut sel = AisSelection::new();
        sel.select(EntityOwner::new(1, 0), SelectionScheme::XOR);
        assert!(sel.is_selected(1));
        sel.select(EntityOwner::new(1, 0), SelectionScheme::XOR);
        assert!(!sel.is_selected(1));
    }

    #[test]
    fn selection_clear() {
        let mut sel = AisSelection::new();
        sel.select(EntityOwner::new(1, 0), SelectionScheme::Add);
        sel.select(EntityOwner::new(2, 0), SelectionScheme::Add);
        sel.clear();
        assert!(sel.is_empty());
    }

    #[test]
    fn indexed_map_of_owner() {
        let mut m = IndexedMapOfOwner::new();
        m.add(EntityOwner::new(10, 0));
        m.add(EntityOwner::new(20, 0));
        assert_eq!(m.extent(), 2);
        assert!(m.contains(10));
        assert_eq!(m.find_index(20), Some(2));
    }
}
