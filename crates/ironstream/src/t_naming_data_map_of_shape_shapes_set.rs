// FILE: t_naming_data_map_of_shape_shapes_set.rs
// occt: TNaming_DataMapOfShapeShapesSet

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<TopoDS_Shape, TNaming_ShapesSet,
//!    TopTools_ShapeMapHasher> TNaming_DataMapOfShapeShapesSet;`
//!
//! `TNaming_ShapesSet` (used by the TNaming_Name resolution) is a set of
//! shapes with Add/Remove/Contains/Filter operations; it is modeled here
//! with the same IsSame shape identity as the outer map keys.

use std::collections::{HashMap, HashSet};

/// Local stand-in for `TopoDS_Shape` (IsSame identity: tshape + location).
#[derive(Clone, Debug)]
pub struct NamingShapeStubSss {
    pub tshape_id: u64,
    pub location_id: u32,
}

impl NamingShapeStubSss {
    pub fn new(tshape_id: u64, location_id: u32) -> Self {
        NamingShapeStubSss { tshape_id, location_id }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }
}

#[derive(Clone, Debug)]
struct NamingShapeKeySss(NamingShapeStubSss);

impl PartialEq for NamingShapeKeySss {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}
impl Eq for NamingShapeKeySss {}
impl std::hash::Hash for NamingShapeKeySss {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.tshape_id.hash(state);
        self.0.location_id.hash(state);
    }
}

/// Local `TNaming_ShapesSet`: a set of shapes (TopTools_MapOfShape inside).
#[derive(Default, Clone)]
pub struct TNamingShapesSetSss {
    shapes: HashSet<NamingShapeKeySss>,
}

impl TNamingShapesSetSss {
    pub fn new() -> Self {
        TNamingShapesSetSss { shapes: HashSet::new() }
    }

    /// TNaming_ShapesSet::Add — true if the shape was not present.
    pub fn add(&mut self, shape: NamingShapeStubSss) -> bool {
        self.shapes.insert(NamingShapeKeySss(shape))
    }

    /// TNaming_ShapesSet::Contains.
    pub fn contains(&self, shape: &NamingShapeStubSss) -> bool {
        self.shapes.contains(&NamingShapeKeySss(shape.clone()))
    }

    /// TNaming_ShapesSet::Remove — true if the shape was present.
    pub fn remove(&mut self, shape: &NamingShapeStubSss) -> bool {
        self.shapes.remove(&NamingShapeKeySss(shape.clone()))
    }

    /// TNaming_ShapesSet::Filter — keep only shapes also in `other`.
    pub fn filter(&mut self, other: &TNamingShapesSetSss) {
        self.shapes.retain(|k| other.shapes.contains(k));
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn n_shapes(&self) -> usize {
        self.shapes.len()
    }
}

/// `TNaming_DataMapOfShapeShapesSet` with NCollection_DataMap semantics.
#[derive(Default)]
pub struct TNamingDataMapOfShapeShapesSet {
    inner: HashMap<NamingShapeKeySss, TNamingShapesSetSss>,
}

impl TNamingDataMapOfShapeShapesSet {
    pub fn new() -> Self {
        TNamingDataMapOfShapeShapesSet { inner: HashMap::new() }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: NamingShapeStubSss, set: TNamingShapesSetSss) -> bool {
        self.inner.insert(NamingShapeKeySss(key), set).is_none()
    }

    pub fn is_bound(&self, key: &NamingShapeStubSss) -> bool {
        self.inner.contains_key(&NamingShapeKeySss(key.clone()))
    }

    pub fn find(&self, key: &NamingShapeStubSss) -> Option<&TNamingShapesSetSss> {
        self.inner.get(&NamingShapeKeySss(key.clone()))
    }

    /// ChangeFind — mutable access to the bound set.
    pub fn change_find(&mut self, key: &NamingShapeStubSss) -> Option<&mut TNamingShapesSetSss> {
        self.inner.get_mut(&NamingShapeKeySss(key.clone()))
    }

    pub fn un_bind(&mut self, key: &NamingShapeStubSss) -> bool {
        self.inner.remove(&NamingShapeKeySss(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_set_add_contains_remove() {
        let mut set = TNamingShapesSetSss::new();
        let e1 = NamingShapeStubSss::new(11, 0);
        assert!(set.add(e1.clone()));
        assert!(!set.add(NamingShapeStubSss::new(11, 0)), "IsSame duplicate rejected");
        assert!(set.contains(&e1));
        assert_eq!(set.n_shapes(), 1);
        assert!(set.remove(&e1));
        assert!(set.is_empty());
    }

    #[test]
    fn filter_keeps_intersection() {
        let mut a = TNamingShapesSetSss::new();
        a.add(NamingShapeStubSss::new(1, 0));
        a.add(NamingShapeStubSss::new(2, 0));
        a.add(NamingShapeStubSss::new(3, 0));
        let mut b = TNamingShapesSetSss::new();
        b.add(NamingShapeStubSss::new(2, 0));
        b.add(NamingShapeStubSss::new(3, 0));
        b.add(NamingShapeStubSss::new(4, 0));
        a.filter(&b);
        assert_eq!(a.n_shapes(), 2);
        assert!(a.contains(&NamingShapeStubSss::new(2, 0)));
        assert!(!a.contains(&NamingShapeStubSss::new(1, 0)));
    }

    #[test]
    fn map_binds_sets_per_shape() {
        let mut map = TNamingDataMapOfShapeShapesSet::new();
        let key = NamingShapeStubSss::new(50, 0);
        let mut set = TNamingShapesSetSss::new();
        set.add(NamingShapeStubSss::new(51, 0));
        assert!(map.bind(key.clone(), set));
        map.change_find(&key).unwrap().add(NamingShapeStubSss::new(52, 0));
        assert_eq!(map.find(&key).unwrap().n_shapes(), 2);
        assert!(map.un_bind(&key));
        assert_eq!(map.extent(), 0);
    }
}
