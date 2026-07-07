// FILE: t_obj_persistence.rs
// occt: TObj_Persistence

//! Root of tools (one per class) to manage persistence of objects
//! inherited from TObj_Object: recovers correctly typed objects out of
//! their persistent type names.
//! Faithful port of `TObj_Persistence` (.hxx + .cxx): a registry of
//! type-name -> factory tools; a tool registers itself on construction
//! and unregisters on destruction; `CreateNewObject` dispatches to the
//! registered tool's `New(label)`; `DumpTypes` lists registered names.
//! The C++ function-local static map is modeled as an explicit registry
//! object (the registration/unregistration mechanics are identical).

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for a TDF_Label passed to factories.
pub type PersistLabelEntryPe = String;

/// Local stand-in for `TObj_Object` created by persistence tools.
#[derive(Debug)]
pub struct TObjObjectRecPe {
    pub type_name: String,
    pub label_entry: PersistLabelEntryPe,
}

pub type HandleTObjObjectPe = Rc<TObjObjectRecPe>;

/// Factory trait — the redefined `New` of each Persistence_ subclass.
pub trait TObjPersistenceToolPe {
    /// Returns a new object of the proper type at the given label.
    fn new_object(&self, label: &str) -> HandleTObjObjectPe;
}

/// Simple factory used by IMPLEMENT_TOBJOCAF_PERSISTENCE-style tools:
/// creates a TObjObjectRecPe tagged with the managed type name.
pub struct TypedFactoryPe {
    pub managed_type: String,
}

impl TObjPersistenceToolPe for TypedFactoryPe {
    fn new_object(&self, label: &str) -> HandleTObjObjectPe {
        Rc::new(TObjObjectRecPe {
            type_name: self.managed_type.clone(),
            label_entry: label.to_string(),
        })
    }
}

/// The dictionary of registered types (`getMapOfTypes`).
#[derive(Default)]
pub struct TObjPersistenceRegistryPe {
    map_of_types: HashMap<String, Rc<dyn TObjPersistenceToolPe>>,
}

impl TObjPersistenceRegistryPe {
    pub fn new() -> Self {
        TObjPersistenceRegistryPe::default()
    }

    /// TObj_Persistence constructor effect: registers the tool.
    pub fn register_tool(&mut self, type_name: &str, tool: Rc<dyn TObjPersistenceToolPe>) {
        self.map_of_types.insert(type_name.to_string(), tool);
    }

    /// TObj_Persistence destructor effect: unregisters the tool.
    pub fn unregister_tool(&mut self, type_name: &str) {
        self.map_of_types.remove(type_name);
    }

    /// TObj_Persistence::CreateNewObject — Null handle (None) when the
    /// type is not registered.
    pub fn create_new_object(&self, type_name: &str, label: &str) -> Option<HandleTObjObjectPe> {
        self.map_of_types
            .get(type_name)
            .map(|tool| tool.new_object(label))
    }

    /// TObj_Persistence::DumpTypes — one type name per line.
    pub fn dump_types(&self) -> String {
        let mut names: Vec<&String> = self.map_of_types.keys().collect();
        names.sort();
        let mut out = String::new();
        for n in names {
            out.push_str(n);
            out.push('\n');
        }
        out
    }

    pub fn is_registered(&self, type_name: &str) -> bool {
        self.map_of_types.contains_key(type_name)
    }

    pub fn nb_types(&self) -> usize {
        self.map_of_types.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry_with(types: &[&str]) -> TObjPersistenceRegistryPe {
        let mut reg = TObjPersistenceRegistryPe::new();
        for t in types {
            reg.register_tool(
                t,
                Rc::new(TypedFactoryPe { managed_type: t.to_string() }),
            );
        }
        reg
    }

    #[test]
    fn create_new_object_of_registered_type() {
        let reg = registry_with(&["TObj_Partition", "TObj_HiddenPartition"]);
        let obj = reg.create_new_object("TObj_Partition", "0:1:4").unwrap();
        assert_eq!(obj.type_name, "TObj_Partition");
        assert_eq!(obj.label_entry, "0:1:4");
    }

    #[test]
    fn unknown_type_returns_null_handle() {
        let reg = registry_with(&["TObj_Partition"]);
        assert!(reg.create_new_object("Unknown_Type", "0:1").is_none());
    }

    #[test]
    fn unregister_removes_factory() {
        let mut reg = registry_with(&["A_Type", "B_Type"]);
        assert_eq!(reg.nb_types(), 2);
        reg.unregister_tool("A_Type");
        assert!(!reg.is_registered("A_Type"));
        assert!(reg.create_new_object("A_Type", "0:1").is_none());
        assert!(reg.is_registered("B_Type"));
    }

    #[test]
    fn dump_types_lists_names() {
        let reg = registry_with(&["Zeta_Type", "Alpha_Type"]);
        assert_eq!(reg.dump_types(), "Alpha_Type\nZeta_Type\n");
    }

    #[test]
    fn custom_tool_dispatch() {
        struct SpecialToolPe;
        impl TObjPersistenceToolPe for SpecialToolPe {
            fn new_object(&self, label: &str) -> HandleTObjObjectPe {
                Rc::new(TObjObjectRecPe {
                    type_name: "Special".to_string(),
                    label_entry: format!("{label}!"),
                })
            }
        }
        let mut reg = TObjPersistenceRegistryPe::new();
        reg.register_tool("Special", Rc::new(SpecialToolPe));
        let obj = reg.create_new_object("Special", "0:9").unwrap();
        assert_eq!(obj.label_entry, "0:9!");
    }
}
