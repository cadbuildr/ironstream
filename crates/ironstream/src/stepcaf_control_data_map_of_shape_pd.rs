// FILE: stepcaf_control_data_map_of_shape_pd.rs
// occt: STEPCAFControl_DataMapOfShapePD

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<TopoDS_Shape,
//!    opencascade::handle<StepBasic_ProductDefinition>, TopTools_ShapeMapHasher>
//!    STEPCAFControl_DataMapOfShapePD;`
//!
//! `TopTools_ShapeMapHasher` compares shapes with `TopoDS_Shape::IsSame`
//! (same TShape and same Location; orientation ignored). The local shape
//! stub carries a tshape id + location id + orientation to reproduce that.

use std::collections::HashMap;
use std::rc::Rc;

/// Orientation flag of a TopoDS_Shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeOrientationSpd {
    Forward,
    Reversed,
}

/// Local stand-in for `TopoDS_Shape`.
#[derive(Clone, Debug)]
pub struct TopoShapeStubSpd {
    /// Identity of the underlying TShape.
    pub tshape_id: u64,
    /// Identity of the location.
    pub location_id: u32,
    /// Orientation — NOT part of IsSame comparison.
    pub orientation: ShapeOrientationSpd,
}

impl TopoShapeStubSpd {
    pub fn new(tshape_id: u64, location_id: u32) -> Self {
        TopoShapeStubSpd {
            tshape_id,
            location_id,
            orientation: ShapeOrientationSpd::Forward,
        }
    }

    /// TopoDS_Shape::IsSame — same TShape and Location, orientation ignored.
    pub fn is_same(&self, other: &Self) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }

    pub fn reversed(&self) -> Self {
        TopoShapeStubSpd {
            orientation: match self.orientation {
                ShapeOrientationSpd::Forward => ShapeOrientationSpd::Reversed,
                ShapeOrientationSpd::Reversed => ShapeOrientationSpd::Forward,
            },
            ..self.clone()
        }
    }
}

/// Key wrapper implementing TopTools_ShapeMapHasher semantics.
#[derive(Clone, Debug)]
struct ShapeMapKeySpd(TopoShapeStubSpd);

impl PartialEq for ShapeMapKeySpd {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}
impl Eq for ShapeMapKeySpd {}
impl std::hash::Hash for ShapeMapKeySpd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.tshape_id.hash(state);
        self.0.location_id.hash(state);
        // orientation intentionally excluded (IsSame semantics)
    }
}

/// Local stand-in for `StepBasic_ProductDefinition`.
#[derive(Debug)]
pub struct StepProductDefinitionSpd {
    pub id: String,
}

pub type HandlePdSpd = Rc<StepProductDefinitionSpd>;

/// `STEPCAFControl_DataMapOfShapePD` with NCollection_DataMap semantics.
#[derive(Default)]
pub struct StepcafControlDataMapOfShapePd {
    inner: HashMap<ShapeMapKeySpd, HandlePdSpd>,
}

impl StepcafControlDataMapOfShapePd {
    pub fn new() -> Self {
        StepcafControlDataMapOfShapePd {
            inner: HashMap::new(),
        }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: TopoShapeStubSpd, item: HandlePdSpd) -> bool {
        self.inner.insert(ShapeMapKeySpd(key), item).is_none()
    }

    pub fn is_bound(&self, key: &TopoShapeStubSpd) -> bool {
        self.inner.contains_key(&ShapeMapKeySpd(key.clone()))
    }

    pub fn find(&self, key: &TopoShapeStubSpd) -> Option<&HandlePdSpd> {
        self.inner.get(&ShapeMapKeySpd(key.clone()))
    }

    pub fn un_bind(&mut self, key: &TopoShapeStubSpd) -> bool {
        self.inner.remove(&ShapeMapKeySpd(key.clone())).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `STEPCAFControl_DataMapIteratorOfDataMapOfShapePD`.
    pub fn iter(&self) -> impl Iterator<Item = (&TopoShapeStubSpd, &HandlePdSpd)> {
        self.inner.iter().map(|(k, v)| (&k.0, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_same_ignores_orientation() {
        let mut map = StepcafControlDataMapOfShapePd::new();
        let sh = TopoShapeStubSpd::new(42, 0);
        map.bind(sh.clone(), Rc::new(StepProductDefinitionSpd { id: "PD1".into() }));
        // Reversed shape IsSame -> found under the same key.
        let rev = sh.reversed();
        assert!(map.is_bound(&rev));
        assert_eq!(map.find(&rev).unwrap().id, "PD1");
    }

    #[test]
    fn different_location_is_different_key() {
        let mut map = StepcafControlDataMapOfShapePd::new();
        let sh_a = TopoShapeStubSpd::new(42, 0);
        let sh_b = TopoShapeStubSpd::new(42, 7); // same TShape, moved
        map.bind(sh_a.clone(), Rc::new(StepProductDefinitionSpd { id: "PD1".into() }));
        assert!(!map.is_bound(&sh_b));
        map.bind(sh_b, Rc::new(StepProductDefinitionSpd { id: "PD2".into() }));
        assert_eq!(map.extent(), 2);
    }

    #[test]
    fn rebind_replaces_value() {
        let mut map = StepcafControlDataMapOfShapePd::new();
        let sh = TopoShapeStubSpd::new(1, 1);
        assert!(map.bind(sh.clone(), Rc::new(StepProductDefinitionSpd { id: "old".into() })));
        assert!(!map.bind(sh.clone(), Rc::new(StepProductDefinitionSpd { id: "new".into() })));
        assert_eq!(map.find(&sh).unwrap().id, "new");
        assert!(map.un_bind(&sh));
        assert_eq!(map.extent(), 0);
    }
}
