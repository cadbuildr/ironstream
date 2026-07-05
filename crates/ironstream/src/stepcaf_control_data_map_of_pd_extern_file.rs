// FILE: stepcaf_control_data_map_of_pd_extern_file.rs
// occt: STEPCAFControl_DataMapOfPDExternFile

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<opencascade::handle<StepBasic_ProductDefinition>,
//!    opencascade::handle<STEPCAFControl_ExternFile>> STEPCAFControl_DataMapOfPDExternFile;`
//!
//! Keys are handles (`StepBasic_ProductDefinition`), which OCCT hashes by
//! handle identity (the pointed-to entity address). This port keys on
//! `Rc` pointer identity for the same semantics.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `StepBasic_ProductDefinition`.
#[derive(Debug)]
pub struct StepPdEntityPde {
    pub id: String,
    pub description: String,
}

impl StepPdEntityPde {
    pub fn new(id: &str, description: &str) -> Self {
        StepPdEntityPde {
            id: id.to_string(),
            description: description.to_string(),
        }
    }
}

/// Local stand-in for `STEPCAFControl_ExternFile`.
#[derive(Debug)]
pub struct StepcafExternFileRecPde {
    pub name: String,
    pub done: bool,
}

impl StepcafExternFileRecPde {
    pub fn new(name: &str) -> Self {
        StepcafExternFileRecPde {
            name: name.to_string(),
            done: false,
        }
    }
}

pub type HandlePdPde = Rc<StepPdEntityPde>;
pub type HandleExternFilePde = Rc<StepcafExternFileRecPde>;

/// Identity key wrapper: OCCT hashes transient handles by pointer.
#[derive(Clone)]
struct PdIdentityKeyPde(HandlePdPde);

impl PartialEq for PdIdentityKeyPde {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for PdIdentityKeyPde {}
impl std::hash::Hash for PdIdentityKeyPde {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

/// `STEPCAFControl_DataMapOfPDExternFile` with NCollection_DataMap semantics.
#[derive(Default)]
pub struct StepcafControlDataMapOfPdExternFile {
    inner: HashMap<PdIdentityKeyPde, HandleExternFilePde>,
}

impl StepcafControlDataMapOfPdExternFile {
    pub fn new() -> Self {
        StepcafControlDataMapOfPdExternFile {
            inner: HashMap::new(),
        }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: HandlePdPde, item: HandleExternFilePde) -> bool {
        self.inner.insert(PdIdentityKeyPde(key), item).is_none()
    }

    pub fn is_bound(&self, key: &HandlePdPde) -> bool {
        self.inner.contains_key(&PdIdentityKeyPde(key.clone()))
    }

    pub fn find(&self, key: &HandlePdPde) -> Option<&HandleExternFilePde> {
        self.inner.get(&PdIdentityKeyPde(key.clone()))
    }

    pub fn un_bind(&mut self, key: &HandlePdPde) -> bool {
        self.inner.remove(&PdIdentityKeyPde(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `STEPCAFControl_DataMapIteratorOfDataMapOfPDExternFile`.
    pub fn iter(&self) -> impl Iterator<Item = (&HandlePdPde, &HandleExternFilePde)> {
        self.inner.iter().map(|(k, v)| (&k.0, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_identity_keying() {
        let mut map = StepcafControlDataMapOfPdExternFile::new();
        let pd_a = Rc::new(StepPdEntityPde::new("PD1", "bracket"));
        // Same field values but a distinct entity: must be a distinct key.
        let pd_b = Rc::new(StepPdEntityPde::new("PD1", "bracket"));
        map.bind(pd_a.clone(), Rc::new(StepcafExternFileRecPde::new("a.stp")));
        assert!(map.is_bound(&pd_a));
        assert!(!map.is_bound(&pd_b));
        map.bind(pd_b.clone(), Rc::new(StepcafExternFileRecPde::new("b.stp")));
        assert_eq!(map.extent(), 2);
    }

    #[test]
    fn bind_rebind_and_unbind() {
        let mut map = StepcafControlDataMapOfPdExternFile::new();
        let pd = Rc::new(StepPdEntityPde::new("PD2", "shaft"));
        assert!(map.bind(pd.clone(), Rc::new(StepcafExternFileRecPde::new("v1.stp"))));
        assert!(!map.bind(pd.clone(), Rc::new(StepcafExternFileRecPde::new("v2.stp"))));
        assert_eq!(map.find(&pd).unwrap().name, "v2.stp");
        assert!(map.un_bind(&pd));
        assert_eq!(map.extent(), 0);
    }
}
