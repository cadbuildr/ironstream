// FILE: t_prs_std_data_map_of_guid_driver.rs
// occt: TPrsStd_DataMapOfGUIDDriver

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<Standard_GUID, opencascade::handle<TPrsStd_Driver>>
//!    TPrsStd_DataMapOfGUIDDriver;`
//! plus `TPrsStd_DataMapIteratorOfDataMapOfGUIDDriver`.
//!
//! This is the table behind `TPrsStd_DriverTable`: presentation drivers
//! registered under attribute GUIDs. GUID equality is by value.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `Standard_GUID` in its textual canonical form
/// ("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"), hashed/compared by value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StandardGuidPgd {
    pub text: String,
}

impl StandardGuidPgd {
    pub fn new(text: &str) -> Self {
        StandardGuidPgd { text: text.to_string() }
    }
}

/// Local stand-in for `TPrsStd_Driver` (builds an AIS presentation).
#[derive(Debug)]
pub struct TPrsStdDriverStubPgd {
    pub driver_name: String,
    /// Number of times Update() was called (behavior probe).
    pub updates: std::cell::Cell<u32>,
}

impl TPrsStdDriverStubPgd {
    pub fn new(name: &str) -> Rc<Self> {
        Rc::new(TPrsStdDriverStubPgd {
            driver_name: name.to_string(),
            updates: std::cell::Cell::new(0),
        })
    }

    /// TPrsStd_Driver::Update — returns true when a presentation is built.
    pub fn update(&self) -> bool {
        self.updates.set(self.updates.get() + 1);
        true
    }
}

pub type HandleTPrsStdDriverPgd = Rc<TPrsStdDriverStubPgd>;

/// `TPrsStd_DataMapOfGUIDDriver` with NCollection_DataMap semantics.
#[derive(Default)]
pub struct TPrsStdDataMapOfGuidDriver {
    inner: HashMap<StandardGuidPgd, HandleTPrsStdDriverPgd>,
}

impl TPrsStdDataMapOfGuidDriver {
    pub fn new() -> Self {
        TPrsStdDataMapOfGuidDriver { inner: HashMap::new() }
    }

    /// Bind — true when the GUID is new (TPrsStd_DriverTable::AddDriver
    /// returns this to signal replacement vs addition).
    pub fn bind(&mut self, guid: StandardGuidPgd, driver: HandleTPrsStdDriverPgd) -> bool {
        self.inner.insert(guid, driver).is_none()
    }

    pub fn is_bound(&self, guid: &StandardGuidPgd) -> bool {
        self.inner.contains_key(guid)
    }

    /// FindDriver-style lookup.
    pub fn find(&self, guid: &StandardGuidPgd) -> Option<&HandleTPrsStdDriverPgd> {
        self.inner.get(guid)
    }

    /// RemoveDriver-style unbind.
    pub fn un_bind(&mut self, guid: &StandardGuidPgd) -> bool {
        self.inner.remove(guid).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `TPrsStd_DataMapIteratorOfDataMapOfGUIDDriver`.
    pub fn iter(&self) -> impl Iterator<Item = (&StandardGuidPgd, &HandleTPrsStdDriverPgd)> {
        self.inner.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AXIS_GUID: &str = "b3aac909-5b78-11d1-8940-080009dc3333";
    const PLANE_GUID: &str = "b3aac90a-5b78-11d1-8940-080009dc3333";

    #[test]
    fn driver_registration_and_lookup() {
        let mut table = TPrsStdDataMapOfGuidDriver::new();
        assert!(table.bind(StandardGuidPgd::new(AXIS_GUID), TPrsStdDriverStubPgd::new("AxisDriver")));
        assert!(table.bind(StandardGuidPgd::new(PLANE_GUID), TPrsStdDriverStubPgd::new("PlaneDriver")));
        assert_eq!(table.extent(), 2);
        let d = table.find(&StandardGuidPgd::new(AXIS_GUID)).unwrap();
        assert_eq!(d.driver_name, "AxisDriver");
    }

    #[test]
    fn rebind_replaces_driver() {
        let mut table = TPrsStdDataMapOfGuidDriver::new();
        let g = StandardGuidPgd::new(AXIS_GUID);
        assert!(table.bind(g.clone(), TPrsStdDriverStubPgd::new("v1")));
        assert!(!table.bind(g.clone(), TPrsStdDriverStubPgd::new("v2")), "AddDriver over existing GUID returns false");
        assert_eq!(table.find(&g).unwrap().driver_name, "v2");
        assert_eq!(table.extent(), 1);
    }

    #[test]
    fn found_driver_updates() {
        let mut table = TPrsStdDataMapOfGuidDriver::new();
        let g = StandardGuidPgd::new(PLANE_GUID);
        table.bind(g.clone(), TPrsStdDriverStubPgd::new("PlaneDriver"));
        let d = table.find(&g).unwrap();
        assert!(d.update());
        assert!(d.update());
        assert_eq!(d.updates.get(), 2);
        assert!(table.un_bind(&g));
        assert!(table.find(&g).is_none());
    }
}
