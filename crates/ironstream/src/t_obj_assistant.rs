// FILE: t_obj_assistant.rs
// occt: TObj_Assistant

//! Interface to the static data used during save or load of TObj models:
//! 1. sequence of models used when loading references (FindModel by name),
//! 2. indexed map of types used to persist object types by index,
//! 3. the current model handle and application version.
//! Faithful port of `TObj_Assistant` (.hxx + .cxx). The C++ function-local
//! statics are grouped into an explicit assistant state object; the
//! find/bind/clear semantics (including the backwards model search, the
//! 1-based type indices, the 0/None misses and the UnknownType binding
//! for null types) follow the C++ code exactly.

use std::rc::Rc;

/// Local stand-in for `TObj_Model` (identified by its modeller name).
#[derive(Debug)]
pub struct TObjModelRecAs {
    pub model_name: String,
}

pub type HandleTObjModelAs = Rc<TObjModelRecAs>;

/// Local stand-in for `Standard_Type` (a dynamic type descriptor).
#[derive(Debug)]
pub struct StandardTypeRecAs {
    pub type_name: String,
}

pub type HandleStandardTypeAs = Rc<StandardTypeRecAs>;

/// Entry of the type map: a real type or the UnknownType placeholder
/// bound for null type handles.
#[derive(Debug)]
enum TypeSlotAs {
    Known(HandleStandardTypeAs),
    Unknown,
}

/// The static data of TObj_Assistant.
#[derive(Default)]
pub struct TObjAssistant {
    models: Vec<HandleTObjModelAs>,
    types: Vec<TypeSlotAs>,
    current_model: Option<HandleTObjModelAs>,
    version: i32,
}

impl TObjAssistant {
    pub fn new() -> Self {
        TObjAssistant::default()
    }

    // ---- Interface for DataMap of Modeller name ----

    /// FindModel — searches backwards (most recently bound first);
    /// None models the null handle.
    pub fn find_model(&self, name: &str) -> Option<HandleTObjModelAs> {
        self.models
            .iter()
            .rev()
            .find(|m| m.model_name == name)
            .cloned()
    }

    /// BindModel — appends to the sequence.
    pub fn bind_model(&mut self, model: HandleTObjModelAs) {
        self.models.push(model);
    }

    /// ClearModelMap.
    pub fn clear_model_map(&mut self) {
        self.models.clear();
    }

    // ---- Interface for Map of Standard Types ----

    /// FindType by 1-based index; None when out of range or the slot
    /// holds the UnknownType placeholder.
    pub fn find_type(&self, type_index: i32) -> Option<HandleStandardTypeAs> {
        if type_index > 0 && type_index as usize <= self.types.len() {
            match &self.types[(type_index - 1) as usize] {
                TypeSlotAs::Known(t) => Some(t.clone()),
                TypeSlotAs::Unknown => None,
            }
        } else {
            None
        }
    }

    /// FindTypeIndex — 0 when not found (identity comparison of handles).
    pub fn find_type_index(&self, ty: &HandleStandardTypeAs) -> i32 {
        for (i, slot) in self.types.iter().enumerate() {
            if let TypeSlotAs::Known(t) = slot {
                if Rc::ptr_eq(t, ty) {
                    return (i + 1) as i32;
                }
            }
        }
        0
    }

    /// BindType — returns the (1-based) index of the bound type; an
    /// already-bound type keeps its index (IndexedMap::Add semantics).
    /// A null type (None) binds a fresh UnknownType placeholder.
    pub fn bind_type(&mut self, ty: Option<HandleStandardTypeAs>) -> i32 {
        match ty {
            Some(t) => {
                let existing = self.find_type_index(&t);
                if existing != 0 {
                    return existing;
                }
                self.types.push(TypeSlotAs::Known(t));
                self.types.len() as i32
            }
            None => {
                self.types.push(TypeSlotAs::Unknown);
                self.types.len() as i32
            }
        }
    }

    /// ClearTypeMap.
    pub fn clear_type_map(&mut self) {
        self.types.clear();
    }

    // ---- Interface to the current model ----

    /// SetCurrentModel.
    pub fn set_current_model(&mut self, model: HandleTObjModelAs) {
        self.current_model = Some(model);
    }

    /// GetCurrentModel.
    pub fn get_current_model(&self) -> Option<HandleTObjModelAs> {
        self.current_model.clone()
    }

    /// UnSetCurrentModel.
    pub fn unset_current_model(&mut self) {
        self.current_model = None;
        self.version = 0;
    }

    /// GetAppVersion — 0 until set for the current document.
    pub fn get_app_version(&self) -> i32 {
        self.version
    }

    /// Sets the application version of the read document.
    pub fn set_app_version(&mut self, version: i32) {
        self.version = version;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(name: &str) -> HandleTObjModelAs {
        Rc::new(TObjModelRecAs { model_name: name.into() })
    }

    fn ty(name: &str) -> HandleStandardTypeAs {
        Rc::new(StandardTypeRecAs { type_name: name.into() })
    }

    #[test]
    fn model_map_find_backwards() {
        let mut asst = TObjAssistant::new();
        let old = model("engine");
        let newer = model("engine"); // same name, bound later
        asst.bind_model(old);
        asst.bind_model(newer.clone());
        let found = asst.find_model("engine").unwrap();
        assert!(Rc::ptr_eq(&found, &newer), "search goes from the end");
        assert!(asst.find_model("ghost").is_none());
        asst.clear_model_map();
        assert!(asst.find_model("engine").is_none());
    }

    #[test]
    fn type_map_one_based_indices() {
        let mut asst = TObjAssistant::new();
        let ta = ty("TObj_Partition");
        let tb = ty("TObj_HiddenPartition");
        assert_eq!(asst.bind_type(Some(ta.clone())), 1);
        assert_eq!(asst.bind_type(Some(tb.clone())), 2);
        // Rebinding an existing type keeps its index.
        assert_eq!(asst.bind_type(Some(ta.clone())), 1);
        assert_eq!(asst.find_type_index(&ta), 1);
        assert!(Rc::ptr_eq(&asst.find_type(2).unwrap(), &tb));
        assert!(asst.find_type(0).is_none());
        assert!(asst.find_type(3).is_none());
        assert_eq!(asst.find_type_index(&ty("TObj_Partition")), 0, "identity, not name");
    }

    #[test]
    fn null_type_binds_unknown_placeholder() {
        let mut asst = TObjAssistant::new();
        let idx = asst.bind_type(None);
        assert_eq!(idx, 1);
        assert!(asst.find_type(1).is_none(), "unknown placeholder is not a real type");
        // Placeholders are never deduplicated.
        assert_eq!(asst.bind_type(None), 2);
        asst.clear_type_map();
        assert_eq!(asst.bind_type(Some(ty("X"))), 1);
    }

    #[test]
    fn current_model_and_version() {
        let mut asst = TObjAssistant::new();
        assert!(asst.get_current_model().is_none());
        assert_eq!(asst.get_app_version(), 0);
        let m = model("doc1");
        asst.set_current_model(m.clone());
        asst.set_app_version(7);
        assert!(Rc::ptr_eq(&asst.get_current_model().unwrap(), &m));
        assert_eq!(asst.get_app_version(), 7);
        asst.unset_current_model();
        assert!(asst.get_current_model().is_none());
        assert_eq!(asst.get_app_version(), 0);
    }
}
