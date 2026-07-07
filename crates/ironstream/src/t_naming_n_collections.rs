// FILE: t_naming_n_collections.rs
// occt: TNaming_NCollections

//! Deprecated typedef header (deprecated since OCCT 8.0.0):
//! `typedef NCollection_Map<TopoDS_Shape> TNaming_MapOfShape;`
//! `typedef NCollection_DataMap<TopoDS_Shape, TNaming_MapOfShape>
//!    TNaming_DataMapOfShapeMapOfShape;`
//! plus the two companion iterator typedefs.
//!
//! Note: these use the DEFAULT NCollection hasher on TopoDS_Shape (not
//! TopTools_ShapeMapHasher); the default shape hasher also compares with
//! IsSame ignoring orientation, so identity is TShape + Location here too.

use std::collections::{HashMap, HashSet};

/// Local stand-in for `TopoDS_Shape` (IsSame identity).
#[derive(Clone, Debug)]
pub struct NamingShapeStubNc {
    pub tshape_id: u64,
    pub location_id: u32,
}

impl NamingShapeStubNc {
    pub fn new(tshape_id: u64, location_id: u32) -> Self {
        NamingShapeStubNc { tshape_id, location_id }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }
}

#[derive(Clone, Debug)]
struct NamingShapeKeyNc(NamingShapeStubNc);

impl PartialEq for NamingShapeKeyNc {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}
impl Eq for NamingShapeKeyNc {}
impl std::hash::Hash for NamingShapeKeyNc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.tshape_id.hash(state);
        self.0.location_id.hash(state);
    }
}

/// `TNaming_MapOfShape`.
#[derive(Default, Clone)]
pub struct TNamingMapOfShapeNc {
    inner: HashSet<NamingShapeKeyNc>,
}

impl TNamingMapOfShapeNc {
    pub fn new() -> Self {
        TNamingMapOfShapeNc { inner: HashSet::new() }
    }

    /// Add — true when new.
    pub fn add(&mut self, s: NamingShapeStubNc) -> bool {
        self.inner.insert(NamingShapeKeyNc(s))
    }

    pub fn contains(&self, s: &NamingShapeStubNc) -> bool {
        self.inner.contains(&NamingShapeKeyNc(s.clone()))
    }

    pub fn remove(&mut self, s: &NamingShapeStubNc) -> bool {
        self.inner.remove(&NamingShapeKeyNc(s.clone()))
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// `TNaming_MapIteratorOfMapOfShape`.
    pub fn iter(&self) -> impl Iterator<Item = &NamingShapeStubNc> {
        self.inner.iter().map(|k| &k.0)
    }
}

/// `TNaming_DataMapOfShapeMapOfShape`.
#[derive(Default)]
pub struct TNamingDataMapOfShapeMapOfShapeNc {
    inner: HashMap<NamingShapeKeyNc, TNamingMapOfShapeNc>,
}

impl TNamingDataMapOfShapeMapOfShapeNc {
    pub fn new() -> Self {
        TNamingDataMapOfShapeMapOfShapeNc { inner: HashMap::new() }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: NamingShapeStubNc, val: TNamingMapOfShapeNc) -> bool {
        self.inner.insert(NamingShapeKeyNc(key), val).is_none()
    }

    pub fn is_bound(&self, key: &NamingShapeStubNc) -> bool {
        self.inner.contains_key(&NamingShapeKeyNc(key.clone()))
    }

    pub fn find(&self, key: &NamingShapeStubNc) -> Option<&TNamingMapOfShapeNc> {
        self.inner.get(&NamingShapeKeyNc(key.clone()))
    }

    pub fn change_find(&mut self, key: &NamingShapeStubNc) -> Option<&mut TNamingMapOfShapeNc> {
        self.inner.get_mut(&NamingShapeKeyNc(key.clone()))
    }

    pub fn un_bind(&mut self, key: &NamingShapeStubNc) -> bool {
        self.inner.remove(&NamingShapeKeyNc(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    /// `TNaming_DataMapIteratorOfDataMapOfShapeMapOfShape`.
    pub fn iter(&self) -> impl Iterator<Item = (&NamingShapeStubNc, &TNamingMapOfShapeNc)> {
        self.inner.iter().map(|(k, v)| (&k.0, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sh(id: u64) -> NamingShapeStubNc {
        NamingShapeStubNc::new(id, 0)
    }

    #[test]
    fn map_of_shape_membership() {
        let mut m = TNamingMapOfShapeNc::new();
        assert!(m.add(sh(1)));
        assert!(!m.add(sh(1)));
        assert!(m.contains(&sh(1)));
        assert_eq!(m.extent(), 1);
        assert!(m.remove(&sh(1)));
        assert!(m.is_empty());
    }

    #[test]
    fn data_map_of_shape_map_of_shape() {
        let mut dm = TNamingDataMapOfShapeMapOfShapeNc::new();
        let old_face = sh(10);
        let mut news = TNamingMapOfShapeNc::new();
        news.add(sh(11));
        news.add(sh(12));
        assert!(dm.bind(old_face.clone(), news));
        // Extend the bound set in place.
        dm.change_find(&old_face).unwrap().add(sh(13));
        let got = dm.find(&old_face).unwrap();
        assert_eq!(got.extent(), 3);
        assert!(got.contains(&sh(12)));
        assert!(dm.un_bind(&old_face));
        assert_eq!(dm.extent(), 0);
    }

    #[test]
    fn nested_iteration_counts() {
        let mut dm = TNamingDataMapOfShapeMapOfShapeNc::new();
        for i in 0..3u64 {
            let mut set = TNamingMapOfShapeNc::new();
            for j in 0..=i {
                set.add(sh(100 + i * 10 + j));
            }
            dm.bind(sh(i), set);
        }
        let total: usize = dm.iter().map(|(_, v)| v.extent()).sum();
        assert_eq!(total, 1 + 2 + 3);
    }
}
