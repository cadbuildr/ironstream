// FILE: t_naming_data_map_of_shape_ptr_ref_shape.rs
// occt: TNaming_DataMapOfShapePtrRefShape

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<TopoDS_Shape, TNaming_PtrRefShape,
//!    TopTools_ShapeMapHasher> TNaming_DataMapOfShapePtrRefShape;`
//!
//! `TNaming_PtrRefShape` is a raw pointer to a `TNaming_RefShape`
//! (the shapes-to-labels back-reference node of the naming structure).
//! It is modeled as an `Rc` to a local ref-shape record. Shape keys use
//! TopTools_ShapeMapHasher semantics (IsSame: TShape + Location).

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `TopoDS_Shape` under ShapeMapHasher.
#[derive(Clone, Debug)]
pub struct NamingShapeStubPrs {
    pub tshape_id: u64,
    pub location_id: u32,
}

impl NamingShapeStubPrs {
    pub fn new(tshape_id: u64, location_id: u32) -> Self {
        NamingShapeStubPrs { tshape_id, location_id }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }
}

#[derive(Clone, Debug)]
struct NamingShapeKeyPrs(NamingShapeStubPrs);

impl PartialEq for NamingShapeKeyPrs {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}
impl Eq for NamingShapeKeyPrs {}
impl std::hash::Hash for NamingShapeKeyPrs {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.tshape_id.hash(state);
        self.0.location_id.hash(state);
    }
}

/// Local stand-in for `TNaming_RefShape` (target of TNaming_PtrRefShape).
#[derive(Debug)]
pub struct TNamingRefShapeRecPrs {
    /// The shape held by the ref-shape node.
    pub shape: NamingShapeStubPrs,
    /// Entry of the label of the first use (TNaming_RefShape::FirstUse).
    pub first_use_entry: String,
}

/// `TNaming_PtrRefShape` modeled as a shared pointer.
pub type TNamingPtrRefShapePrs = Rc<TNamingRefShapeRecPrs>;

/// `TNaming_DataMapOfShapePtrRefShape` with NCollection_DataMap semantics.
#[derive(Default)]
pub struct TNamingDataMapOfShapePtrRefShape {
    inner: HashMap<NamingShapeKeyPrs, TNamingPtrRefShapePrs>,
}

impl TNamingDataMapOfShapePtrRefShape {
    pub fn new() -> Self {
        TNamingDataMapOfShapePtrRefShape { inner: HashMap::new() }
    }

    /// Bind — true when the shape was not bound yet.
    pub fn bind(&mut self, key: NamingShapeStubPrs, ptr: TNamingPtrRefShapePrs) -> bool {
        self.inner.insert(NamingShapeKeyPrs(key), ptr).is_none()
    }

    pub fn is_bound(&self, key: &NamingShapeStubPrs) -> bool {
        self.inner.contains_key(&NamingShapeKeyPrs(key.clone()))
    }

    pub fn find(&self, key: &NamingShapeStubPrs) -> Option<&TNamingPtrRefShapePrs> {
        self.inner.get(&NamingShapeKeyPrs(key.clone()))
    }

    pub fn un_bind(&mut self, key: &NamingShapeStubPrs) -> bool {
        self.inner.remove(&NamingShapeKeyPrs(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `TNaming_DataMapIteratorOfDataMapOfShapePtrRefShape`.
    pub fn iter(&self) -> impl Iterator<Item = (&NamingShapeStubPrs, &TNamingPtrRefShapePrs)> {
        self.inner.iter().map(|(k, v)| (&k.0, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_to_refshape_backref() {
        let mut map = TNamingDataMapOfShapePtrRefShape::new();
        let face = NamingShapeStubPrs::new(100, 0);
        let rs = Rc::new(TNamingRefShapeRecPrs {
            shape: face.clone(),
            first_use_entry: "0:1:1:2".into(),
        });
        assert!(map.bind(face.clone(), rs.clone()));
        let found = map.find(&face).unwrap();
        assert_eq!(found.first_use_entry, "0:1:1:2");
        assert!(Rc::ptr_eq(found, &rs), "PtrRefShape is a pointer, not a copy");
    }

    #[test]
    fn is_same_keying() {
        let mut map = TNamingDataMapOfShapePtrRefShape::new();
        let a = NamingShapeStubPrs::new(5, 2);
        map.bind(
            a.clone(),
            Rc::new(TNamingRefShapeRecPrs { shape: a.clone(), first_use_entry: "0:1".into() }),
        );
        // Same TShape+Location built independently: IsSame -> same key.
        assert!(map.is_bound(&NamingShapeStubPrs::new(5, 2)));
        // Moved shape: different key.
        assert!(!map.is_bound(&NamingShapeStubPrs::new(5, 3)));
    }

    #[test]
    fn unbind_and_extent() {
        let mut map = TNamingDataMapOfShapePtrRefShape::new();
        let s = NamingShapeStubPrs::new(1, 0);
        map.bind(
            s.clone(),
            Rc::new(TNamingRefShapeRecPrs { shape: s.clone(), first_use_entry: "0:2".into() }),
        );
        assert_eq!(map.extent(), 1);
        assert!(map.un_bind(&s));
        assert!(!map.un_bind(&s));
        assert_eq!(map.extent(), 0);
    }
}
