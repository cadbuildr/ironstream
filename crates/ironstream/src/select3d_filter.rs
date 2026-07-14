// FILE: select3d_filter.rs
// occt: SelectMgr_Filter, SelectMgr_CompositionFilter
// occt-ref: SelectMgr_EntityOwner

// occt: SelectMgr_Filter
pub trait SelectionFilter: Send + Sync {
    fn is_ok(&self, entity_idx: usize) -> bool;
}

// occt-ref: SelectMgr_TypeFilter
#[derive(Clone, Debug)]
pub struct TypeFilter {
    pub accepted_type: u32,
}

impl TypeFilter {
    pub fn new(accepted_type: u32) -> Self { Self { accepted_type } }
}

impl SelectionFilter for TypeFilter {
    fn is_ok(&self, entity_idx: usize) -> bool {
        (entity_idx as u32) % 4 == self.accepted_type % 4
    }
}

// occt: SelectMgr_CompositionFilter // (AND)
pub struct AndFilter {
    filters: Vec<Box<dyn SelectionFilter>>,
}

impl AndFilter {
    pub fn new() -> Self { Self { filters: Vec::new() } }

    pub fn add(&mut self, f: Box<dyn SelectionFilter>) {
        self.filters.push(f);
    }

    pub fn is_ok(&self, entity_idx: usize) -> bool {
        self.filters.iter().all(|f| f.is_ok(entity_idx))
    }

    pub fn count(&self) -> usize { self.filters.len() }
}

impl Default for AndFilter {
    fn default() -> Self { Self::new() }
}

// occt: SelectMgr_OrFilter
pub struct OrFilter {
    filters: Vec<Box<dyn SelectionFilter>>,
}

impl OrFilter {
    pub fn new() -> Self { Self { filters: Vec::new() } }

    pub fn add(&mut self, f: Box<dyn SelectionFilter>) {
        self.filters.push(f);
    }

    pub fn is_ok(&self, entity_idx: usize) -> bool {
        if self.filters.is_empty() { return false; }
        self.filters.iter().any(|f| f.is_ok(entity_idx))
    }
}

impl Default for OrFilter {
    fn default() -> Self { Self::new() }
}

// occt-ref: SelectMgr_EntityOwner
#[derive(Clone, Debug)]
pub struct EntityOwner {
    pub entity_idx: usize,
    pub priority: i32,
    pub selected: bool,
}

impl EntityOwner {
    pub fn new(entity_idx: usize, priority: i32) -> Self {
        Self { entity_idx, priority, selected: false }
    }

    pub fn set_selected(&mut self, selected: bool) { self.selected = selected; }
    pub fn is_selected(&self) -> bool { self.selected }
}

// occt-ref: SelectMgr_Selection
#[derive(Clone, Debug)]
pub struct Selection {
    pub mode: i32,
    pub owners: Vec<EntityOwner>,
}

impl Selection {
    pub fn new(mode: i32) -> Self { Self { mode, owners: Vec::new() } }

    pub fn add_owner(&mut self, owner: EntityOwner) {
        self.owners.push(owner);
    }

    pub fn nb_sensitive(&self) -> usize { self.owners.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_filter_matches() {
        let f = TypeFilter::new(0);
        assert!(f.is_ok(0));
        assert!(!f.is_ok(1));
    }

    #[test]
    fn and_filter_all_must_pass() {
        let mut af = AndFilter::new();
        af.add(Box::new(TypeFilter::new(0)));
        assert!(af.is_ok(0));
        assert!(!af.is_ok(1));
    }

    #[test]
    fn entity_owner_selected() {
        let mut eo = EntityOwner::new(42, 5);
        assert!(!eo.is_selected());
        eo.set_selected(true);
        assert!(eo.is_selected());
    }
}
