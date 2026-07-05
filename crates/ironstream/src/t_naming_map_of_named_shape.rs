// FILE: t_naming_map_of_named_shape.rs
// occt: TNaming_MapOfNamedShape

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_Map<opencascade::handle<TNaming_NamedShape>>
//!    TNaming_MapOfNamedShape;`
//! plus `TNaming_MapIteratorOfMapOfNamedShape`.
//!
//! OCCT hashes transient handles by pointer, so two attribute records
//! with equal contents are still distinct members. That identity keying
//! is reproduced with `Rc` pointer identity.

use std::collections::HashSet;
use std::rc::Rc;

/// Local stand-in for `TNaming_NamedShape`.
#[derive(Debug)]
pub struct TNamingNamedShapeRecMns {
    pub label_entry: String,
    pub version: i32,
}

pub type HandleNamedShapeMns = Rc<TNamingNamedShapeRecMns>;

#[derive(Clone)]
struct NamedShapeIdentityMns(HandleNamedShapeMns);

impl PartialEq for NamedShapeIdentityMns {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for NamedShapeIdentityMns {}
impl std::hash::Hash for NamedShapeIdentityMns {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

/// `TNaming_MapOfNamedShape` with NCollection_Map semantics.
#[derive(Default)]
pub struct TNamingMapOfNamedShape {
    inner: HashSet<NamedShapeIdentityMns>,
}

impl TNamingMapOfNamedShape {
    pub fn new() -> Self {
        TNamingMapOfNamedShape { inner: HashSet::new() }
    }

    /// Add — true when the item was not in the map.
    pub fn add(&mut self, ns: HandleNamedShapeMns) -> bool {
        self.inner.insert(NamedShapeIdentityMns(ns))
    }

    pub fn contains(&self, ns: &HandleNamedShapeMns) -> bool {
        self.inner.contains(&NamedShapeIdentityMns(ns.clone()))
    }

    /// Remove — true when the item was in the map.
    pub fn remove(&mut self, ns: &HandleNamedShapeMns) -> bool {
        self.inner.remove(&NamedShapeIdentityMns(ns.clone()))
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `TNaming_MapIteratorOfMapOfNamedShape`.
    pub fn iter(&self) -> impl Iterator<Item = &HandleNamedShapeMns> {
        self.inner.iter().map(|k| &k.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ns(entry: &str) -> HandleNamedShapeMns {
        Rc::new(TNamingNamedShapeRecMns { label_entry: entry.into(), version: 0 })
    }

    #[test]
    fn identity_membership() {
        let mut map = TNamingMapOfNamedShape::new();
        let a = ns("0:1:1");
        let a_twin = ns("0:1:1"); // same contents, distinct handle
        assert!(map.add(a.clone()));
        assert!(!map.add(a.clone()), "same handle rejected");
        assert!(map.add(a_twin.clone()), "distinct handle accepted");
        assert_eq!(map.extent(), 2);
        assert!(map.contains(&a));
        assert!(map.remove(&a));
        assert!(!map.contains(&a));
        assert!(map.contains(&a_twin));
    }

    #[test]
    fn iterate_and_clear() {
        let mut map = TNamingMapOfNamedShape::new();
        let handles: Vec<_> = (0..3).map(|i| ns(&format!("0:{i}"))).collect();
        for h in &handles {
            map.add(h.clone());
        }
        assert_eq!(map.iter().count(), 3);
        map.clear();
        assert!(map.is_empty());
    }
}
