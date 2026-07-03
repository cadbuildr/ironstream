// FILE: stdselect_shape.rs
// occt: StdSelect_BRepOwner, StdSelect_ShapeTypeFilter, StdSelect_EdgeFilter,
//       StdSelect_FaceFilter

/// Type of shape used for filtering.
/// occt: TopAbs_ShapeEnum subset used in StdSelect filters
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StdSelectShapeType {
    #[default]
    Any,
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    CompSolid,
    Compound,
}

/// Owner of a B-Rep shape for interactive selection.
/// occt: StdSelect_BRepOwner
#[derive(Clone, Debug)]
pub struct StdSelectBRepOwner {
    pub shape_id: u32,
    pub shape_type: StdSelectShapeType,
    pub selectable_id: u32,
    pub is_selected: bool,
    pub priority: i32,
}

impl StdSelectBRepOwner {
    pub fn new(shape_id: u32, shape_type: StdSelectShapeType, priority: i32) -> Self {
        Self {
            shape_id,
            shape_type,
            selectable_id: 0,
            is_selected: false,
            priority,
        }
    }

    pub fn set_selectable(&mut self, id: u32) { self.selectable_id = id; }
    pub fn select(&mut self) { self.is_selected = true; }
    pub fn deselect(&mut self) { self.is_selected = false; }
    pub fn is_selected(&self) -> bool { self.is_selected }
    pub fn priority(&self) -> i32 { self.priority }
    pub fn has_selectable(&self) -> bool { self.selectable_id != 0 }
}

/// Filter accepting only shapes of a given type.
/// occt: StdSelect_ShapeTypeFilter
#[derive(Clone, Debug)]
pub struct StdSelectShapeTypeFilter {
    pub accepted_type: StdSelectShapeType,
}

impl StdSelectShapeTypeFilter {
    pub fn new(t: StdSelectShapeType) -> Self { Self { accepted_type: t } }

    pub fn is_ok(&self, owner: &StdSelectBRepOwner) -> bool {
        self.accepted_type == StdSelectShapeType::Any || owner.shape_type == self.accepted_type
    }
}

/// Filter accepting only edges.
/// occt: StdSelect_EdgeFilter
#[derive(Clone, Debug, Default)]
pub struct StdSelectEdgeFilter {
    pub only_linear: bool,
}

impl StdSelectEdgeFilter {
    pub fn new(only_linear: bool) -> Self { Self { only_linear } }

    pub fn is_ok(&self, owner: &StdSelectBRepOwner) -> bool {
        owner.shape_type == StdSelectShapeType::Edge
    }
}

/// Filter accepting only faces.
/// occt: StdSelect_FaceFilter
#[derive(Clone, Debug, Default)]
pub struct StdSelectFaceFilter {
    pub only_planar: bool,
}

impl StdSelectFaceFilter {
    pub fn new(only_planar: bool) -> Self { Self { only_planar } }

    pub fn is_ok(&self, owner: &StdSelectBRepOwner) -> bool {
        owner.shape_type == StdSelectShapeType::Face
    }
}

/// Pool of owners for selection presentation management.
/// occt: StdSelect_IndexedDataMapOfShapePresentation (simplified)
#[derive(Clone, Debug, Default)]
pub struct StdSelectOwnerPool {
    pub owners: Vec<StdSelectBRepOwner>,
}

impl StdSelectOwnerPool {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, owner: StdSelectBRepOwner) {
        self.owners.push(owner);
    }

    pub fn selected(&self) -> Vec<&StdSelectBRepOwner> {
        self.owners.iter().filter(|o| o.is_selected).collect()
    }

    pub fn clear_selection(&mut self) {
        for o in &mut self.owners { o.is_selected = false; }
    }

    pub fn nb_owners(&self) -> usize { self.owners.len() }

    pub fn find_by_shape(&self, shape_id: u32) -> Option<&StdSelectBRepOwner> {
        self.owners.iter().find(|o| o.shape_id == shape_id)
    }

    pub fn filter<F: Fn(&StdSelectBRepOwner) -> bool>(&self, f: F) -> Vec<&StdSelectBRepOwner> {
        self.owners.iter().filter(|o| f(o)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_select_deselect() {
        let mut o = StdSelectBRepOwner::new(1, StdSelectShapeType::Face, 5);
        assert!(!o.is_selected());
        o.select();
        assert!(o.is_selected());
        o.deselect();
        assert!(!o.is_selected());
    }

    #[test]
    fn shape_type_filter() {
        let f = StdSelectShapeTypeFilter::new(StdSelectShapeType::Edge);
        let edge = StdSelectBRepOwner::new(1, StdSelectShapeType::Edge, 0);
        let face = StdSelectBRepOwner::new(2, StdSelectShapeType::Face, 0);
        assert!(f.is_ok(&edge));
        assert!(!f.is_ok(&face));
    }

    #[test]
    fn any_filter_accepts_all() {
        let f = StdSelectShapeTypeFilter::new(StdSelectShapeType::Any);
        let v = StdSelectBRepOwner::new(1, StdSelectShapeType::Vertex, 0);
        assert!(f.is_ok(&v));
    }

    #[test]
    fn edge_filter() {
        let f = StdSelectEdgeFilter::new(false);
        let edge = StdSelectBRepOwner::new(1, StdSelectShapeType::Edge, 0);
        let face = StdSelectBRepOwner::new(2, StdSelectShapeType::Face, 0);
        assert!(f.is_ok(&edge));
        assert!(!f.is_ok(&face));
    }

    #[test]
    fn pool_selected() {
        let mut pool = StdSelectOwnerPool::new();
        let mut o1 = StdSelectBRepOwner::new(1, StdSelectShapeType::Edge, 0);
        o1.select();
        pool.add(o1);
        pool.add(StdSelectBRepOwner::new(2, StdSelectShapeType::Face, 0));
        assert_eq!(pool.selected().len(), 1);
        pool.clear_selection();
        assert_eq!(pool.selected().len(), 0);
    }
}
