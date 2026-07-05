// FILE: t_obj_t_model.rs
// occt: TObj_TModel

//! Attribute to store OCAF-based models in OCAF tree.
//! Faithful port of `TObj_TModel` (.hxx + .cxx): a TDF attribute holding
//! a handle to the TObj_Model object, with GUID
//! "bbdab6a6-dca9-11d4-ba37-0060b0ee18ea", Backup-on-Set, and the
//! NewEmpty/Restore/Paste OCAF protocol. The model is a local record.

use std::cell::RefCell;
use std::rc::Rc;

/// The GUID of TObj_TModel.
pub const TOBJ_TMODEL_GUID: &str = "bbdab6a6-dca9-11d4-ba37-0060b0ee18ea";

/// Local stand-in for `TObj_Model` (what the attribute stores).
#[derive(Debug, PartialEq)]
pub struct TObjModelRecTm {
    pub format: String,
    pub nb_objects: usize,
}

pub type HandleTObjModelTm = Rc<TObjModelRecTm>;

/// Attribute to store OCAF-based models in the OCAF tree.
#[derive(Default)]
pub struct TObjTModel {
    model: RefCell<Option<HandleTObjModelTm>>,
    backup: RefCell<Option<Option<HandleTObjModelTm>>>,
}

impl TObjTModel {
    /// Empty constructor (model handle is null).
    pub fn new() -> Self {
        TObjTModel::default()
    }

    /// TObj_TModel::GetID.
    pub fn get_id() -> &'static str {
        TOBJ_TMODEL_GUID
    }

    /// TObj_TModel::ID.
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// Sets the Model object (Backup() first).
    pub fn set(&self, model: HandleTObjModelTm) {
        *self.backup.borrow_mut() = Some(self.model.borrow().clone());
        *self.model.borrow_mut() = Some(model);
    }

    /// Returns the Model object (None = null handle).
    pub fn model(&self) -> Option<HandleTObjModelTm> {
        self.model.borrow().clone()
    }

    /// NewEmpty.
    pub fn new_empty(&self) -> TObjTModel {
        TObjTModel::new()
    }

    /// Restore from `with` (transaction abort).
    pub fn restore(&self, with: &TObjTModel) {
        *self.model.borrow_mut() = with.model();
    }

    /// Paste into `into`.
    pub fn paste(&self, into: &TObjTModel) {
        if let Some(m) = self.model() {
            into.set(m);
        }
    }

    /// Value saved by the last Backup() (observability).
    pub fn backup_value(&self) -> Option<Option<HandleTObjModelTm>> {
        self.backup.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_and_empty_state() {
        let attr = TObjTModel::new();
        assert_eq!(attr.id(), "bbdab6a6-dca9-11d4-ba37-0060b0ee18ea");
        assert!(attr.model().is_none(), "empty constructor leaves null model");
    }

    #[test]
    fn set_and_get_model() {
        let attr = TObjTModel::new();
        let model = Rc::new(TObjModelRecTm { format: "BinOcaf".into(), nb_objects: 12 });
        attr.set(model.clone());
        let got = attr.model().unwrap();
        assert!(Rc::ptr_eq(&got, &model));
        assert_eq!(got.format, "BinOcaf");
    }

    #[test]
    fn set_backs_up_previous_handle() {
        let attr = TObjTModel::new();
        let m1 = Rc::new(TObjModelRecTm { format: "XmlOcaf".into(), nb_objects: 1 });
        attr.set(m1.clone());
        assert_eq!(attr.backup_value(), Some(None), "first backup was the null state");
        let m2 = Rc::new(TObjModelRecTm { format: "BinOcaf".into(), nb_objects: 2 });
        attr.set(m2);
        let backup = attr.backup_value().unwrap().unwrap();
        assert!(Rc::ptr_eq(&backup, &m1));
    }

    #[test]
    fn restore_and_paste() {
        let src = TObjTModel::new();
        src.set(Rc::new(TObjModelRecTm { format: "BinOcaf".into(), nb_objects: 3 }));
        let dst = src.new_empty();
        assert!(dst.model().is_none());
        src.paste(&dst);
        assert_eq!(dst.model().unwrap().nb_objects, 3);
        let aborted = TObjTModel::new();
        aborted.restore(&src);
        assert!(Rc::ptr_eq(&aborted.model().unwrap(), &src.model().unwrap()));
    }
}
