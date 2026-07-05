// FILE: t_obj_container.rs
// occt: TObj_Container

//! Deprecated typedef header (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<opencascade::handle<TCollection_HExtendedString>, TDF_Label>
//!    TObj_DataMapOfNameLabel;`
//! `typedef NCollection_DataMap<opencascade::handle<TObj_Object>,
//!    opencascade::handle<TObj_HSequenceOfObject>> TObj_DataMapOfObjectHSequenceOcafObjects;`
//! `typedef NCollection_DataMap<TCollection_AsciiString, void*> TObj_DataMapOfStringPointer;`
//!
//! Note: `TObj_DataMapOfNameLabel` keys are HExtendedString HANDLES —
//! in OCCT this map is used with a hasher comparing string CONTENT
//! (extended strings), so this port keys by the string value.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `TDF_Label` (tag path).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OcafLabelStubTc {
    pub tags: Vec<i32>,
}

impl OcafLabelStubTc {
    pub fn new(tags: &[i32]) -> Self {
        OcafLabelStubTc { tags: tags.to_vec() }
    }
}

/// Local stand-in for `TObj_Object` (an object of the TObj model).
#[derive(Debug)]
pub struct TObjObjectStubTc {
    pub name: String,
}

pub type HandleTObjObjectTc = Rc<TObjObjectStubTc>;

/// Local stand-in for `TObj_HSequenceOfObject`.
pub type HandleTObjHSequenceTc = Rc<Vec<HandleTObjObjectTc>>;

/// `TObj_DataMapOfNameLabel`: name (extended string content) -> label.
#[derive(Default)]
pub struct TObjDataMapOfNameLabel {
    inner: HashMap<String, OcafLabelStubTc>,
}

impl TObjDataMapOfNameLabel {
    pub fn new() -> Self {
        TObjDataMapOfNameLabel { inner: HashMap::new() }
    }

    /// Bind — true when the name was not registered.
    pub fn bind(&mut self, name: &str, label: OcafLabelStubTc) -> bool {
        self.inner.insert(name.to_string(), label).is_none()
    }

    pub fn is_bound(&self, name: &str) -> bool {
        self.inner.contains_key(name)
    }

    pub fn find(&self, name: &str) -> Option<&OcafLabelStubTc> {
        self.inner.get(name)
    }

    pub fn un_bind(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }
}

/// Identity key for object handles (OCCT hashes handles by pointer).
#[derive(Clone)]
struct TObjObjectIdentityTc(HandleTObjObjectTc);

impl PartialEq for TObjObjectIdentityTc {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for TObjObjectIdentityTc {}
impl std::hash::Hash for TObjObjectIdentityTc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

/// `TObj_DataMapOfObjectHSequenceOcafObjects`.
#[derive(Default)]
pub struct TObjDataMapOfObjectHSequenceOcafObjects {
    inner: HashMap<TObjObjectIdentityTc, HandleTObjHSequenceTc>,
}

impl TObjDataMapOfObjectHSequenceOcafObjects {
    pub fn new() -> Self {
        TObjDataMapOfObjectHSequenceOcafObjects { inner: HashMap::new() }
    }

    pub fn bind(&mut self, key: HandleTObjObjectTc, seq: HandleTObjHSequenceTc) -> bool {
        self.inner.insert(TObjObjectIdentityTc(key), seq).is_none()
    }

    pub fn is_bound(&self, key: &HandleTObjObjectTc) -> bool {
        self.inner.contains_key(&TObjObjectIdentityTc(key.clone()))
    }

    pub fn find(&self, key: &HandleTObjObjectTc) -> Option<&HandleTObjHSequenceTc> {
        self.inner.get(&TObjObjectIdentityTc(key.clone()))
    }

    pub fn un_bind(&mut self, key: &HandleTObjObjectTc) -> bool {
        self.inner.remove(&TObjObjectIdentityTc(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }
}

/// `TObj_DataMapOfStringPointer` (`void*` modeled as usize address value).
#[derive(Default)]
pub struct TObjDataMapOfStringPointer {
    inner: HashMap<String, usize>,
}

impl TObjDataMapOfStringPointer {
    pub fn new() -> Self {
        TObjDataMapOfStringPointer { inner: HashMap::new() }
    }

    pub fn bind(&mut self, key: &str, ptr: usize) -> bool {
        self.inner.insert(key.to_string(), ptr).is_none()
    }

    pub fn is_bound(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn find(&self, key: &str) -> Option<usize> {
        self.inner.get(key).copied()
    }

    pub fn un_bind(&mut self, key: &str) -> bool {
        self.inner.remove(key).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_label_registry() {
        let mut map = TObjDataMapOfNameLabel::new();
        assert!(map.bind("Wheel", OcafLabelStubTc::new(&[0, 1, 5])));
        assert!(!map.bind("Wheel", OcafLabelStubTc::new(&[0, 1, 6])), "rebind returns false");
        assert_eq!(map.find("Wheel").unwrap().tags, vec![0, 1, 6]);
        assert!(!map.is_bound("wheel"), "extended-string compare is case-sensitive");
        assert!(map.un_bind("Wheel"));
        assert_eq!(map.extent(), 0);
    }

    #[test]
    fn object_to_sequence_map_uses_identity() {
        let mut map = TObjDataMapOfObjectHSequenceOcafObjects::new();
        let obj = Rc::new(TObjObjectStubTc { name: "asm".into() });
        let clone_val = Rc::new(TObjObjectStubTc { name: "asm".into() });
        let children: HandleTObjHSequenceTc = Rc::new(vec![
            Rc::new(TObjObjectStubTc { name: "child1".into() }),
            Rc::new(TObjObjectStubTc { name: "child2".into() }),
        ]);
        map.bind(obj.clone(), children);
        assert!(map.is_bound(&obj));
        assert!(!map.is_bound(&clone_val), "value-equal object is a different key");
        assert_eq!(map.find(&obj).unwrap().len(), 2);
    }

    #[test]
    fn string_pointer_map() {
        let mut map = TObjDataMapOfStringPointer::new();
        let sentinel: usize = 0xDEAD_BEEF;
        assert!(map.bind("driver_slot", sentinel));
        assert_eq!(map.find("driver_slot"), Some(sentinel));
        assert!(map.un_bind("driver_slot"));
        assert_eq!(map.find("driver_slot"), None);
    }
}
