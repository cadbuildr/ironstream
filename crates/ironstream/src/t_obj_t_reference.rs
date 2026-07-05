// FILE: t_obj_t_reference.rs
// occt: TObj_TReference

//! Attribute for storing references to the objects which implement
//! TObj_Object interface in the OCAF tree.
//! Faithful port of `TObj_TReference` (.hxx + .cxx): GUID
//! "3bbefb44-e618-11d4-ba38-0060b0ee18ea"; the attribute stores the
//! referred label and the master label. The static Set creates (or
//! reuses) the attribute, unregisters the back reference from a
//! previously referred object and registers the master as a back
//! reference on the newly referred object. Get() resolves the object
//! through the TObj_TObject attribute at the referred label.
//! BeforeForget removes the back reference again.
//!
//! Labels and the label -> object registry are local records.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The GUID of TObj_TReference.
pub const TOBJ_TREFERENCE_GUID: &str = "3bbefb44-e618-11d4-ba38-0060b0ee18ea";

/// Local stand-in for TDF_Label (entry string; empty = null label).
pub type LabelEntryTr = String;

/// Local stand-in for `TObj_Object` with back-reference support.
#[derive(Debug)]
pub struct TObjObjectRecTr {
    pub name: String,
    pub label: LabelEntryTr,
    /// Back references: master objects referring to this one.
    pub back_references: RefCell<Vec<String>>,
}

impl TObjObjectRecTr {
    pub fn new(name: &str, label: &str) -> Rc<Self> {
        Rc::new(TObjObjectRecTr {
            name: name.to_string(),
            label: label.to_string(),
            back_references: RefCell::new(Vec::new()),
        })
    }

    /// TObj_Object::AddBackReference.
    pub fn add_back_reference(&self, master: &TObjObjectRecTr) {
        self.back_references.borrow_mut().push(master.name.clone());
    }

    /// TObj_Object::RemoveBackReference — first occurrence.
    pub fn remove_back_reference(&self, master: &TObjObjectRecTr) {
        let mut refs = self.back_references.borrow_mut();
        if let Some(pos) = refs.iter().position(|n| n == &master.name) {
            refs.remove(pos);
        }
    }
}

pub type HandleTObjObjectTr = Rc<TObjObjectRecTr>;

/// Local document registry: label entry -> object (TObj_TObject lookup).
#[derive(Default)]
pub struct TObjDocumentRegistryTr {
    objects_by_label: HashMap<LabelEntryTr, HandleTObjObjectTr>,
    /// TObj_TReference attributes stored per label.
    references_by_label: HashMap<LabelEntryTr, Rc<TObjTReference>>,
}

impl TObjDocumentRegistryTr {
    pub fn new() -> Self {
        TObjDocumentRegistryTr::default()
    }

    /// Bind an object at its label (models TObj_TObject::Set).
    pub fn register_object(&mut self, obj: HandleTObjObjectTr) {
        self.objects_by_label.insert(obj.label.clone(), obj);
    }

    pub fn object_at(&self, label: &str) -> Option<HandleTObjObjectTr> {
        self.objects_by_label.get(label).cloned()
    }
}

/// Attribute for storing references between TObj objects.
#[derive(Default)]
pub struct TObjTReference {
    /// Label that indicates the referenced object (empty = null).
    label: RefCell<LabelEntryTr>,
    /// Label of the object that holds this reference.
    master_label: RefCell<LabelEntryTr>,
}

impl TObjTReference {
    /// Empty constructor.
    pub fn new() -> Self {
        TObjTReference::default()
    }

    /// TObj_TReference::GetID.
    pub fn get_id() -> &'static str {
        TOBJ_TREFERENCE_GUID
    }

    /// TObj_TReference::ID.
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// Static Set: creates the reference at `at_label` towards `object`,
    /// registering the back reference from `object` to `master`.
    /// Re-setting an existing reference first removes the old back
    /// reference (as the C++ Set does).
    pub fn set_reference(
        doc: &mut TObjDocumentRegistryTr,
        at_label: &str,
        object: &HandleTObjObjectTr,
        master: &HandleTObjObjectTr,
    ) -> Rc<TObjTReference> {
        let attr = if let Some(existing) = doc.references_by_label.get(at_label).cloned() {
            // Existing attribute: unregister the previous back reference.
            if let Some(prev_obj) = existing.get(doc) {
                prev_obj.remove_back_reference(master);
            }
            existing
        } else {
            let a = Rc::new(TObjTReference::new());
            doc.references_by_label.insert(at_label.to_string(), a.clone());
            a
        };
        attr.set(object, &master.label);
        object.add_back_reference(master);
        attr
    }

    /// Sets the reference to `elem` with the master label (Backup elided:
    /// the pre-modification value is the previous label pair).
    pub fn set(&self, elem: &HandleTObjObjectTr, master_label: &str) {
        *self.label.borrow_mut() = elem.label.clone();
        *self.master_label.borrow_mut() = master_label.to_string();
    }

    /// Persistence-only Set by raw labels.
    pub fn set_labels(&self, label: &str, master_label: &str) {
        *self.label.borrow_mut() = label.to_string();
        *self.master_label.borrow_mut() = master_label.to_string();
    }

    /// Returns the referenced object, resolved via TObj_TObject at the
    /// referred label; None when the label is null or holds no object.
    pub fn get(&self, doc: &TObjDocumentRegistryTr) -> Option<HandleTObjObjectTr> {
        let label = self.label.borrow().clone();
        if label.is_empty() {
            return None;
        }
        doc.object_at(&label)
    }

    /// Returns the label of the master object.
    pub fn get_master_label(&self) -> LabelEntryTr {
        self.master_label.borrow().clone()
    }

    /// Returns the referred label.
    pub fn get_label(&self) -> LabelEntryTr {
        self.label.borrow().clone()
    }

    /// NewEmpty.
    pub fn new_empty(&self) -> TObjTReference {
        TObjTReference::new()
    }

    /// Restore from `with` (transaction abort).
    pub fn restore(&self, with: &TObjTReference) {
        *self.label.borrow_mut() = with.get_label();
        *self.master_label.borrow_mut() = with.get_master_label();
    }

    /// BeforeForget: removes the back reference held by the referred
    /// object towards the master.
    pub fn before_forget(&self, doc: &TObjDocumentRegistryTr) {
        if let (Some(obj), Some(master)) = (
            self.get(doc),
            doc.object_at(&self.get_master_label()),
        ) {
            obj.remove_back_reference(&master);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_with(names: &[(&str, &str)]) -> (TObjDocumentRegistryTr, Vec<HandleTObjObjectTr>) {
        let mut doc = TObjDocumentRegistryTr::new();
        let mut objs = Vec::new();
        for (name, label) in names {
            let o = TObjObjectRecTr::new(name, label);
            doc.register_object(o.clone());
            objs.push(o);
        }
        (doc, objs)
    }

    #[test]
    fn guid_is_fixed() {
        assert_eq!(TObjTReference::get_id(), "3bbefb44-e618-11d4-ba38-0060b0ee18ea");
    }

    #[test]
    fn set_creates_back_reference() {
        let (mut doc, objs) = doc_with(&[("wheel", "0:1:1"), ("car", "0:1:2")]);
        let wheel = objs[0].clone();
        let car = objs[1].clone();
        let r = TObjTReference::set_reference(&mut doc, "0:1:2:1", &wheel, &car);
        assert_eq!(r.get_label(), "0:1:1");
        assert_eq!(r.get_master_label(), "0:1:2");
        assert!(Rc::ptr_eq(&r.get(&doc).unwrap(), &wheel));
        assert_eq!(*wheel.back_references.borrow(), vec!["car".to_string()]);
    }

    #[test]
    fn re_set_moves_back_reference() {
        let (mut doc, objs) =
            doc_with(&[("wheel_a", "0:1:1"), ("wheel_b", "0:1:2"), ("car", "0:1:3")]);
        let (wa, wb, car) = (objs[0].clone(), objs[1].clone(), objs[2].clone());
        let r1 = TObjTReference::set_reference(&mut doc, "0:1:3:1", &wa, &car);
        let r2 = TObjTReference::set_reference(&mut doc, "0:1:3:1", &wb, &car);
        assert!(Rc::ptr_eq(&r1, &r2), "attribute at the label reused");
        assert!(wa.back_references.borrow().is_empty(), "old back ref removed");
        assert_eq!(*wb.back_references.borrow(), vec!["car".to_string()]);
        assert!(Rc::ptr_eq(&r2.get(&doc).unwrap(), &wb));
    }

    #[test]
    fn before_forget_removes_back_reference() {
        let (mut doc, objs) = doc_with(&[("bolt", "0:2:1"), ("plate", "0:2:2")]);
        let (bolt, plate) = (objs[0].clone(), objs[1].clone());
        let r = TObjTReference::set_reference(&mut doc, "0:2:2:1", &bolt, &plate);
        assert_eq!(bolt.back_references.borrow().len(), 1);
        r.before_forget(&doc);
        assert!(bolt.back_references.borrow().is_empty());
    }

    #[test]
    fn null_label_resolves_to_none() {
        let doc = TObjDocumentRegistryTr::new();
        let r = TObjTReference::new();
        assert!(r.get(&doc).is_none());
        assert_eq!(r.get_label(), "");
    }

    #[test]
    fn restore_copies_both_labels() {
        let src = TObjTReference::new();
        src.set_labels("0:5:1", "0:5:2");
        let dst = src.new_empty();
        dst.restore(&src);
        assert_eq!(dst.get_label(), "0:5:1");
        assert_eq!(dst.get_master_label(), "0:5:2");
    }
}
