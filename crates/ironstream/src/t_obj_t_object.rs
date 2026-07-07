// FILE: t_obj_t_object.rs
// occt: TObj_TObject

//! Purpose: OCAF Attribute to storing objects (interfaces) of OCAF-based
//! modelers in the OCAF tree.
//! Faithful port of `TObj_TObject` (.hxx + .cxx): GUID
//! "bbdab6a7-dca9-11d4-ba37-0060b0ee18ea", find-or-create Set on a label,
//! Backup-on-Set, NewEmpty/Restore/Paste, and the BeforeForget behavior
//! that kills the stored object: forget sub-label attributes, remove
//! back references (TObj_Forced) and null the object's label so that
//! `IsAlive()` becomes false. The TObj object and label are local records.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The GUID of TObj_TObject.
pub const TOBJ_TOBJECT_GUID: &str = "bbdab6a7-dca9-11d4-ba37-0060b0ee18ea";

/// Local stand-in for the object's OCAF label with its sub-labels.
#[derive(Debug, Default)]
pub struct TObjLabelNodeTo {
    pub entry: String,
    /// Attribute payloads stored on sub-labels (forgettable).
    pub sub_label_attributes: Vec<String>,
}

/// Local stand-in for `TObj_Object`.
#[derive(Debug)]
pub struct TObjObjectRecTo {
    pub name: String,
    /// The object's label; None models a null TDF_Label (object dead).
    pub label: RefCell<Option<TObjLabelNodeTo>>,
    /// Names of objects holding back references to this one.
    pub back_references: RefCell<Vec<String>>,
}

impl TObjObjectRecTo {
    pub fn new(name: &str, label_entry: &str) -> Rc<Self> {
        Rc::new(TObjObjectRecTo {
            name: name.to_string(),
            label: RefCell::new(Some(TObjLabelNodeTo {
                entry: label_entry.to_string(),
                sub_label_attributes: Vec::new(),
            })),
            back_references: RefCell::new(Vec::new()),
        })
    }

    /// TObj_Object::IsAlive — true while the label is not null.
    pub fn is_alive(&self) -> bool {
        self.label.borrow().is_some()
    }

    /// TObj_Object::RemoveBackReferences (TObj_Forced drops them all).
    pub fn remove_back_references_forced(&self) {
        self.back_references.borrow_mut().clear();
    }
}

pub type HandleTObjObjectTo = Rc<TObjObjectRecTo>;

/// OCAF attribute storing a TObj object.
#[derive(Default)]
pub struct TObjTObject {
    elem: RefCell<Option<HandleTObjObjectTo>>,
    backup: RefCell<Option<Option<HandleTObjObjectTo>>>,
}

pub type HandleTObjTObjectTo = Rc<TObjTObject>;

impl TObjTObject {
    /// Empty constructor.
    pub fn new() -> Self {
        TObjTObject::default()
    }

    /// TObj_TObject::GetID.
    pub fn get_id() -> &'static str {
        TOBJ_TOBJECT_GUID
    }

    /// TObj_TObject::ID.
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// Sets the element (Backup() first).
    pub fn set(&self, elem: HandleTObjObjectTo) {
        *self.backup.borrow_mut() = Some(self.elem.borrow().clone());
        *self.elem.borrow_mut() = Some(elem);
    }

    /// Static Set on a label: find-or-create then Set.
    pub fn set_on_label(
        label: &mut OcafAttributeSlotTo,
        elem: HandleTObjObjectTo,
    ) -> HandleTObjTObjectTo {
        let attr = if let Some(existing) = label.find_attribute(Self::get_id()) {
            existing
        } else {
            let a: HandleTObjTObjectTo = Rc::new(TObjTObject::new());
            label.add_attribute(a.clone());
            a
        };
        attr.set(elem);
        attr
    }

    /// Returns the stored element.
    pub fn get(&self) -> Option<HandleTObjObjectTo> {
        self.elem.borrow().clone()
    }

    /// NewEmpty.
    pub fn new_empty(&self) -> TObjTObject {
        TObjTObject::new()
    }

    /// Restore from `with`.
    pub fn restore(&self, with: &TObjTObject) {
        *self.elem.borrow_mut() = with.get();
    }

    /// Paste into `into`.
    pub fn paste(&self, into: &TObjTObject) {
        if let Some(e) = self.get() {
            into.set(e);
        }
    }

    /// BeforeForget: tell the TObj object to die —
    /// `elem.is_alive() == false` afterwards.
    pub fn before_forget(&self) {
        if let Some(elem) = self.elem.borrow().clone() {
            // Forget all attributes on the object's sub-labels.
            if let Some(label) = elem.label.borrow_mut().as_mut() {
                label.sub_label_attributes.clear();
            }
            // Remove back references before the document dies.
            elem.remove_back_references_forced();
            // Null the object's label.
            *elem.label.borrow_mut() = None;
        }
    }
}

/// Local stand-in for a TDF label's attribute set (GUID -> attribute).
#[derive(Default)]
pub struct OcafAttributeSlotTo {
    attributes: HashMap<String, HandleTObjTObjectTo>,
}

impl OcafAttributeSlotTo {
    pub fn new() -> Self {
        OcafAttributeSlotTo::default()
    }

    pub fn find_attribute(&self, guid: &str) -> Option<HandleTObjTObjectTo> {
        self.attributes.get(guid).cloned()
    }

    pub fn add_attribute(&mut self, attr: HandleTObjTObjectTo) {
        self.attributes.insert(attr.id().to_string(), attr);
    }

    pub fn nb_attributes(&self) -> usize {
        self.attributes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_is_fixed() {
        assert_eq!(TObjTObject::get_id(), "bbdab6a7-dca9-11d4-ba37-0060b0ee18ea");
    }

    #[test]
    fn set_on_label_find_or_create() {
        let mut label = OcafAttributeSlotTo::new();
        let obj1 = TObjObjectRecTo::new("bolt", "0:1:5");
        let a1 = TObjTObject::set_on_label(&mut label, obj1.clone());
        let obj2 = TObjObjectRecTo::new("nut", "0:1:6");
        let a2 = TObjTObject::set_on_label(&mut label, obj2.clone());
        assert!(Rc::ptr_eq(&a1, &a2));
        assert_eq!(label.nb_attributes(), 1);
        assert!(Rc::ptr_eq(&a1.get().unwrap(), &obj2), "second Set replaced the element");
        // Backup keeps the previous element.
        let backup = a1.backup_value_for_test();
        assert!(Rc::ptr_eq(&backup.unwrap().unwrap(), &obj1));
    }

    impl TObjTObject {
        fn backup_value_for_test(&self) -> Option<Option<HandleTObjObjectTo>> {
            self.backup.borrow().clone()
        }
    }

    #[test]
    fn before_forget_kills_object() {
        let attr = TObjTObject::new();
        let obj = TObjObjectRecTo::new("gear", "0:2:1");
        obj.label
            .borrow_mut()
            .as_mut()
            .unwrap()
            .sub_label_attributes
            .push("TNaming_NamedShape".to_string());
        obj.back_references.borrow_mut().push("assembly".to_string());
        attr.set(obj.clone());
        assert!(obj.is_alive());
        attr.before_forget();
        assert!(!obj.is_alive(), "label nulled -> object dead");
        assert!(obj.back_references.borrow().is_empty());
    }

    #[test]
    fn restore_and_paste() {
        let src = TObjTObject::new();
        let obj = TObjObjectRecTo::new("shaft", "0:3");
        src.set(obj.clone());
        let dst = src.new_empty();
        src.paste(&dst);
        assert!(Rc::ptr_eq(&dst.get().unwrap(), &obj));
        let restored = TObjTObject::new();
        restored.restore(&src);
        assert!(Rc::ptr_eq(&restored.get().unwrap(), &obj));
    }
}
