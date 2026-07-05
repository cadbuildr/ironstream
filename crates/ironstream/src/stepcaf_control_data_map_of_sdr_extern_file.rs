// FILE: stepcaf_control_data_map_of_sdr_extern_file.rs
// occt: STEPCAFControl_DataMapOfSDRExternFile

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<opencascade::handle<StepShape_ShapeDefinitionRepresentation>,
//!    opencascade::handle<STEPCAFControl_ExternFile>> STEPCAFControl_DataMapOfSDRExternFile;`
//!
//! Handle keys hash by entity identity, mirrored here via `Rc` pointer identity.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `StepShape_ShapeDefinitionRepresentation`.
#[derive(Debug)]
pub struct StepSdrEntitySdre {
    /// Name of the used representation.
    pub used_representation: String,
}

impl StepSdrEntitySdre {
    pub fn new(used_representation: &str) -> Self {
        StepSdrEntitySdre {
            used_representation: used_representation.to_string(),
        }
    }
}

/// Local stand-in for `STEPCAFControl_ExternFile`.
#[derive(Debug)]
pub struct StepcafExternFileRecSdre {
    pub name: String,
}

pub type HandleSdrSdre = Rc<StepSdrEntitySdre>;
pub type HandleExternFileSdre = Rc<StepcafExternFileRecSdre>;

#[derive(Clone)]
struct SdrIdentityKeySdre(HandleSdrSdre);

impl PartialEq for SdrIdentityKeySdre {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for SdrIdentityKeySdre {}
impl std::hash::Hash for SdrIdentityKeySdre {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

/// `STEPCAFControl_DataMapOfSDRExternFile` with NCollection_DataMap semantics.
#[derive(Default)]
pub struct StepcafControlDataMapOfSdrExternFile {
    inner: HashMap<SdrIdentityKeySdre, HandleExternFileSdre>,
}

impl StepcafControlDataMapOfSdrExternFile {
    pub fn new() -> Self {
        StepcafControlDataMapOfSdrExternFile {
            inner: HashMap::new(),
        }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: HandleSdrSdre, item: HandleExternFileSdre) -> bool {
        self.inner.insert(SdrIdentityKeySdre(key), item).is_none()
    }

    pub fn is_bound(&self, key: &HandleSdrSdre) -> bool {
        self.inner.contains_key(&SdrIdentityKeySdre(key.clone()))
    }

    pub fn find(&self, key: &HandleSdrSdre) -> Option<&HandleExternFileSdre> {
        self.inner.get(&SdrIdentityKeySdre(key.clone()))
    }

    pub fn un_bind(&mut self, key: &HandleSdrSdre) -> bool {
        self.inner.remove(&SdrIdentityKeySdre(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `STEPCAFControl_DataMapIteratorOfDataMapOfSDRExternFile`.
    pub fn iter(&self) -> impl Iterator<Item = (&HandleSdrSdre, &HandleExternFileSdre)> {
        self.inner.iter().map(|(k, v)| (&k.0, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_keys_and_lookup() {
        let mut map = StepcafControlDataMapOfSdrExternFile::new();
        let sdr1 = Rc::new(StepSdrEntitySdre::new("advanced_brep"));
        let sdr2 = Rc::new(StepSdrEntitySdre::new("advanced_brep"));
        map.bind(
            sdr1.clone(),
            Rc::new(StepcafExternFileRecSdre {
                name: "part1.stp".into(),
            }),
        );
        assert!(map.is_bound(&sdr1));
        assert!(!map.is_bound(&sdr2), "identity keying: equal fields but different entity");
        assert_eq!(map.find(&sdr1).unwrap().name, "part1.stp");
    }

    #[test]
    fn clear_and_extent() {
        let mut map = StepcafControlDataMapOfSdrExternFile::new();
        for i in 0..4 {
            map.bind(
                Rc::new(StepSdrEntitySdre::new("rep")),
                Rc::new(StepcafExternFileRecSdre {
                    name: format!("f{i}.stp"),
                }),
            );
        }
        assert_eq!(map.extent(), 4);
        map.clear();
        assert_eq!(map.extent(), 0);
    }
}
