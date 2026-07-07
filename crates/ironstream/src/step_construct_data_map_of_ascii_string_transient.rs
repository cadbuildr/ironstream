// FILE: step_construct_data_map_of_ascii_string_transient.rs
// occt: STEPConstruct_DataMapOfAsciiStringTransient

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<TCollection_AsciiString,
//!    opencascade::handle<Standard_Transient>> STEPConstruct_DataMapOfAsciiStringTransient;`
//!
//! Keys are ASCII strings; values are handles to any transient entity.
//! The transient value is modeled locally as a reference-counted record
//! with a dynamic-type name (mirroring Standard_Transient::DynamicType).

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for a `Standard_Transient` payload.
#[derive(Debug)]
pub struct TransientRecAst {
    /// Dynamic type name, as DynamicType()->Name() would report.
    pub dynamic_type: String,
    /// Arbitrary payload carried by the entity.
    pub payload: String,
}

impl TransientRecAst {
    pub fn new(dynamic_type: &str, payload: &str) -> Self {
        TransientRecAst {
            dynamic_type: dynamic_type.to_string(),
            payload: payload.to_string(),
        }
    }

    /// Standard_Transient::IsKind by type name.
    pub fn is_kind(&self, type_name: &str) -> bool {
        self.dynamic_type == type_name
    }
}

pub type HandleTransientAst = Rc<TransientRecAst>;

/// `STEPConstruct_DataMapOfAsciiStringTransient` with NCollection semantics.
#[derive(Default)]
pub struct StepConstructDataMapOfAsciiStringTransient {
    inner: HashMap<String, HandleTransientAst>,
}

impl StepConstructDataMapOfAsciiStringTransient {
    pub fn new() -> Self {
        StepConstructDataMapOfAsciiStringTransient {
            inner: HashMap::new(),
        }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: &str, item: HandleTransientAst) -> bool {
        self.inner.insert(key.to_string(), item).is_none()
    }

    pub fn is_bound(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn find(&self, key: &str) -> Option<&HandleTransientAst> {
        self.inner.get(key)
    }

    pub fn un_bind(&mut self, key: &str) -> bool {
        self.inner.remove(key).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `STEPConstruct_DataMapIteratorOfDataMapOfAsciiStringTransient`.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &HandleTransientAst)> {
        self.inner.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_string_keys_are_case_sensitive() {
        let mut map = StepConstructDataMapOfAsciiStringTransient::new();
        map.bind("mm", Rc::new(TransientRecAst::new("StepBasic_SiUnit", "MILLI/METRE")));
        assert!(map.is_bound("mm"));
        assert!(!map.is_bound("MM"), "TCollection_AsciiString compare is case-sensitive");
    }

    #[test]
    fn bind_find_unbind_cycle() {
        let mut map = StepConstructDataMapOfAsciiStringTransient::new();
        assert!(map.bind("style1", Rc::new(TransientRecAst::new("StepVisual_StyledItem", "red"))));
        assert!(!map.bind("style1", Rc::new(TransientRecAst::new("StepVisual_StyledItem", "blue"))));
        let found = map.find("style1").unwrap();
        assert_eq!(found.payload, "blue");
        assert!(found.is_kind("StepVisual_StyledItem"));
        assert!(!found.is_kind("StepBasic_SiUnit"));
        assert!(map.un_bind("style1"));
        assert!(!map.un_bind("style1"));
        assert_eq!(map.extent(), 0);
    }

    #[test]
    fn shared_handle_semantics() {
        let mut map = StepConstructDataMapOfAsciiStringTransient::new();
        let ent = Rc::new(TransientRecAst::new("Shared", "x"));
        map.bind("a", ent.clone());
        map.bind("b", ent.clone());
        // Both keys share the same transient handle.
        assert!(Rc::ptr_eq(map.find("a").unwrap(), map.find("b").unwrap()));
    }
}
