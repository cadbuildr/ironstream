// FILE: t_obj_check_model.rs
// occt: TObj_CheckModel

//! This class provides consistency check of the TObj model: it collects
//! all inconsistencies in status bits and supports a fix mode in which
//! some inconsistencies are corrected.
//! Faithful port of `TObj_CheckModel` (.hxx + .cxx):
//! - Perform(): clears status, fails (Message_Fail1) on a null model,
//!   then checks references;
//! - checkReferences(): for each object, every forward reference must
//!   point to an alive object (Alarm2) and have a matching back
//!   reference — missing ones are Alarm4, or Warn1 + AddBackReference
//!   in fix mode; every back reference must be alive (Alarm3) and be
//!   confirmed by a forward reference — unconfirmed ones are Alarm5, or
//!   Warn2 + removal in fix mode;
//! - result is "no alarms and no fails".
//! The model/objects/reference structure is a local in-memory graph.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// Message_Status bits used by the checker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CheckStatusCm {
    Fail1,
    Alarm2,
    Alarm3,
    Alarm4,
    Alarm5,
    Warn1,
    Warn2,
}

impl CheckStatusCm {
    pub fn is_alarm(&self) -> bool {
        matches!(
            self,
            CheckStatusCm::Alarm2 | CheckStatusCm::Alarm3 | CheckStatusCm::Alarm4 | CheckStatusCm::Alarm5
        )
    }

    pub fn is_fail(&self) -> bool {
        matches!(self, CheckStatusCm::Fail1)
    }
}

/// Local stand-in for `TObj_Object` with references/back references.
#[derive(Debug)]
pub struct TObjObjectRecCm {
    pub name: String,
    pub alive: RefCell<bool>,
    pub references: RefCell<Vec<Rc<TObjObjectRecCm>>>,
    pub back_references: RefCell<Vec<Rc<TObjObjectRecCm>>>,
}

impl TObjObjectRecCm {
    pub fn new(name: &str) -> Rc<Self> {
        Rc::new(TObjObjectRecCm {
            name: name.to_string(),
            alive: RefCell::new(true),
            references: RefCell::new(Vec::new()),
            back_references: RefCell::new(Vec::new()),
        })
    }

    pub fn is_alive(&self) -> bool {
        *self.alive.borrow()
    }

    pub fn add_back_reference(self: &Rc<Self>, obj: &Rc<TObjObjectRecCm>) {
        self.back_references.borrow_mut().push(obj.clone());
    }

    pub fn remove_back_reference(self: &Rc<Self>, obj: &Rc<TObjObjectRecCm>) {
        let mut refs = self.back_references.borrow_mut();
        if let Some(pos) = refs.iter().position(|r| Rc::ptr_eq(r, obj)) {
            refs.remove(pos);
        }
    }
}

pub type HandleTObjObjectCm = Rc<TObjObjectRecCm>;

/// Local stand-in for `TObj_Model` (objects + null-label flag).
#[derive(Debug, Default)]
pub struct TObjModelRecCm {
    pub objects: Vec<HandleTObjObjectCm>,
    pub label_is_null: bool,
}

/// The consistency checker.
pub struct TObjCheckModel {
    model: Option<Rc<TObjModelRecCm>>,
    to_fix: bool,
    status: HashSet<CheckStatusCm>,
    /// Messages attached to statuses: (status, object name).
    status_messages: Vec<(CheckStatusCm, String)>,
}

impl TObjCheckModel {
    /// Initialize checker by model (`myToFix = false`).
    pub fn new(model: Option<Rc<TObjModelRecCm>>) -> Self {
        TObjCheckModel {
            model,
            to_fix: false,
            status: HashSet::new(),
            status_messages: Vec::new(),
        }
    }

    /// SetToFix.
    pub fn set_to_fix(&mut self, to_fix: bool) {
        self.to_fix = to_fix;
    }

    /// IsToFix.
    pub fn is_to_fix(&self) -> bool {
        self.to_fix
    }

    /// GetModel.
    pub fn get_model(&self) -> Option<Rc<TObjModelRecCm>> {
        self.model.clone()
    }

    fn set_status(&mut self, status: CheckStatusCm, name: &str) {
        self.status.insert(status);
        self.status_messages.push((status, name.to_string()));
    }

    pub fn has_status(&self, status: CheckStatusCm) -> bool {
        self.status.contains(&status)
    }

    pub fn status_messages(&self) -> &[(CheckStatusCm, String)] {
        &self.status_messages
    }

    /// Perform — returns true if no inconsistencies found.
    pub fn perform(&mut self) -> bool {
        self.status.clear();
        self.status_messages.clear();
        let model = match &self.model {
            Some(m) if !m.label_is_null => m.clone(),
            _ => {
                self.set_status(CheckStatusCm::Fail1, "");
                return false;
            }
        };
        self.check_references(&model)
    }

    /// checkReferences.
    fn check_references(&mut self, model: &Rc<TObjModelRecCm>) -> bool {
        for obj in &model.objects {
            // Check forward references.
            let refs: Vec<HandleTObjObjectCm> = obj.references.borrow().clone();
            for referred in refs {
                if !referred.is_alive() {
                    self.set_status(CheckStatusCm::Alarm2, &obj.name);
                    continue;
                }
                let has_back = referred
                    .back_references
                    .borrow()
                    .iter()
                    .any(|b| Rc::ptr_eq(b, obj));
                if has_back {
                    continue; // ok, back reference found
                }
                if self.is_to_fix() {
                    self.set_status(CheckStatusCm::Warn1, &obj.name);
                    referred.add_back_reference(obj);
                } else {
                    self.set_status(CheckStatusCm::Alarm4, &obj.name);
                }
            }

            // Check back references.
            let back_refs: Vec<HandleTObjObjectCm> = obj.back_references.borrow().clone();
            let mut bad_back_refs: Vec<HandleTObjObjectCm> = Vec::new();
            for referring in back_refs {
                if !referring.is_alive() {
                    self.set_status(CheckStatusCm::Alarm3, &obj.name);
                    continue;
                }
                let has_forward = referring
                    .references
                    .borrow()
                    .iter()
                    .any(|r| Rc::ptr_eq(r, obj));
                if has_forward {
                    continue; // ok, reference found
                }
                if self.is_to_fix() {
                    self.set_status(CheckStatusCm::Warn2, &obj.name);
                    bad_back_refs.push(referring);
                } else {
                    self.set_status(CheckStatusCm::Alarm5, &obj.name);
                }
            }
            // Remove back references that are not confirmed.
            for bad in &bad_back_refs {
                obj.remove_back_reference(bad);
            }
        }

        !self.status.iter().any(|s| s.is_alarm() || s.is_fail())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linked_pair() -> (Rc<TObjModelRecCm>, HandleTObjObjectCm, HandleTObjObjectCm) {
        let a = TObjObjectRecCm::new("a");
        let b = TObjObjectRecCm::new("b");
        a.references.borrow_mut().push(b.clone());
        b.back_references.borrow_mut().push(a.clone());
        let model = Rc::new(TObjModelRecCm {
            objects: vec![a.clone(), b.clone()],
            label_is_null: false,
        });
        (model, a, b)
    }

    #[test]
    fn consistent_model_passes() {
        let (model, _, _) = linked_pair();
        let mut checker = TObjCheckModel::new(Some(model));
        assert!(checker.perform());
        assert!(checker.status_messages().is_empty());
    }

    #[test]
    fn null_model_fails() {
        let mut checker = TObjCheckModel::new(None);
        assert!(!checker.perform());
        assert!(checker.has_status(CheckStatusCm::Fail1));
        let model = Rc::new(TObjModelRecCm { objects: vec![], label_is_null: true });
        let mut checker2 = TObjCheckModel::new(Some(model));
        assert!(!checker2.perform());
    }

    #[test]
    fn missing_back_reference_is_alarm4() {
        let (model, _a, b) = linked_pair();
        b.back_references.borrow_mut().clear(); // break consistency
        let mut checker = TObjCheckModel::new(Some(model));
        assert!(!checker.perform());
        assert!(checker.has_status(CheckStatusCm::Alarm4));
        assert_eq!(checker.status_messages()[0].1, "a");
    }

    #[test]
    fn fix_mode_restores_back_reference() {
        let (model, a, b) = linked_pair();
        b.back_references.borrow_mut().clear();
        let mut checker = TObjCheckModel::new(Some(model.clone()));
        checker.set_to_fix(true);
        assert!(checker.perform(), "warnings only -> success");
        assert!(checker.has_status(CheckStatusCm::Warn1));
        assert!(b.back_references.borrow().iter().any(|x| Rc::ptr_eq(x, &a)));
        // Second pass is clean.
        let mut checker2 = TObjCheckModel::new(Some(model));
        assert!(checker2.perform());
    }

    #[test]
    fn stale_back_reference_alarm5_and_fix() {
        let (model, a, b) = linked_pair();
        a.references.borrow_mut().clear(); // b's back ref now unconfirmed
        let mut checker = TObjCheckModel::new(Some(model.clone()));
        assert!(!checker.perform());
        assert!(checker.has_status(CheckStatusCm::Alarm5));
        // Fix mode removes the stale back reference from b.
        let mut fixer = TObjCheckModel::new(Some(model));
        fixer.set_to_fix(true);
        assert!(fixer.perform());
        assert!(fixer.has_status(CheckStatusCm::Warn2));
        assert!(b.back_references.borrow().is_empty());
    }

    #[test]
    fn dead_reference_is_alarm2() {
        let (model, _a, b) = linked_pair();
        *b.alive.borrow_mut() = false;
        let mut checker = TObjCheckModel::new(Some(model));
        assert!(!checker.perform());
        assert!(checker.has_status(CheckStatusCm::Alarm2));
    }
}
