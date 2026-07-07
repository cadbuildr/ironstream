// FILE: stepcaf_control_data_map_of_shape_sdr.rs
// occt: STEPCAFControl_DataMapOfShapeSDR

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<TopoDS_Shape,
//!    opencascade::handle<StepShape_ShapeDefinitionRepresentation>, TopTools_ShapeMapHasher>
//!    STEPCAFControl_DataMapOfShapeSDR;`
//!
//! Shape keys use `TopTools_ShapeMapHasher` (IsSame: TShape + Location,
//! orientation ignored), reproduced by the local shape stub below.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `TopoDS_Shape` (IsSame identity: tshape + location).
#[derive(Clone, Debug)]
pub struct TopoShapeStubSsdr {
    pub tshape_id: u64,
    pub location_id: u32,
    /// true = FORWARD, false = REVERSED; excluded from IsSame.
    pub forward: bool,
}

impl TopoShapeStubSsdr {
    pub fn new(tshape_id: u64, location_id: u32) -> Self {
        TopoShapeStubSsdr {
            tshape_id,
            location_id,
            forward: true,
        }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }
}

#[derive(Clone, Debug)]
struct ShapeHasherKeySsdr(TopoShapeStubSsdr);

impl PartialEq for ShapeHasherKeySsdr {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}
impl Eq for ShapeHasherKeySsdr {}
impl std::hash::Hash for ShapeHasherKeySsdr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.tshape_id.hash(state);
        self.0.location_id.hash(state);
    }
}

/// Local stand-in for `StepShape_ShapeDefinitionRepresentation`.
#[derive(Debug)]
pub struct StepSdrEntitySsdr {
    pub definition_name: String,
}

pub type HandleSdrSsdr = Rc<StepSdrEntitySsdr>;

/// `STEPCAFControl_DataMapOfShapeSDR` with NCollection_DataMap semantics.
#[derive(Default)]
pub struct StepcafControlDataMapOfShapeSdr {
    inner: HashMap<ShapeHasherKeySsdr, HandleSdrSsdr>,
}

impl StepcafControlDataMapOfShapeSdr {
    pub fn new() -> Self {
        StepcafControlDataMapOfShapeSdr {
            inner: HashMap::new(),
        }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: TopoShapeStubSsdr, item: HandleSdrSsdr) -> bool {
        self.inner.insert(ShapeHasherKeySsdr(key), item).is_none()
    }

    pub fn is_bound(&self, key: &TopoShapeStubSsdr) -> bool {
        self.inner.contains_key(&ShapeHasherKeySsdr(key.clone()))
    }

    pub fn find(&self, key: &TopoShapeStubSsdr) -> Option<&HandleSdrSsdr> {
        self.inner.get(&ShapeHasherKeySsdr(key.clone()))
    }

    pub fn un_bind(&mut self, key: &TopoShapeStubSsdr) -> bool {
        self.inner.remove(&ShapeHasherKeySsdr(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `STEPCAFControl_DataMapIteratorOfDataMapOfShapeSDR`.
    pub fn iter(&self) -> impl Iterator<Item = (&TopoShapeStubSsdr, &HandleSdrSsdr)> {
        self.inner.iter().map(|(k, v)| (&k.0, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_excluded_from_key() {
        let mut map = StepcafControlDataMapOfShapeSdr::new();
        let mut sh = TopoShapeStubSsdr::new(9, 3);
        map.bind(
            sh.clone(),
            Rc::new(StepSdrEntitySsdr {
                definition_name: "sdr_main".into(),
            }),
        );
        sh.forward = false;
        assert!(map.is_bound(&sh));
        assert_eq!(map.find(&sh).unwrap().definition_name, "sdr_main");
    }

    #[test]
    fn bind_unbind_extent() {
        let mut map = StepcafControlDataMapOfShapeSdr::new();
        let a = TopoShapeStubSsdr::new(1, 0);
        let b = TopoShapeStubSsdr::new(2, 0);
        assert!(map.bind(a.clone(), Rc::new(StepSdrEntitySsdr { definition_name: "A".into() })));
        assert!(map.bind(b.clone(), Rc::new(StepSdrEntitySsdr { definition_name: "B".into() })));
        assert_eq!(map.extent(), 2);
        assert!(map.un_bind(&a));
        assert!(!map.is_bound(&a));
        assert!(map.is_bound(&b));
        map.clear();
        assert_eq!(map.extent(), 0);
    }
}
