// FILE: step_construct_data_map_of_point_transient.rs
// occt: STEPConstruct_DataMapOfPointTransient

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<gp_Pnt, opencascade::handle<Standard_Transient>>
//!    STEPConstruct_DataMapOfPointTransient;`
//!
//! `gp_Pnt` keys hash by exact coordinate bits (STEPConstruct_PointHasher
//! compares with strict coordinate equality, not tolerance).

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `gp_Pnt`.
#[derive(Clone, Copy, Debug)]
pub struct GpPntKeyPt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpPntKeyPt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpPntKeyPt { x, y, z }
    }

    /// STEPConstruct_PointHasher::IsEqual — strict coordinate equality.
    pub fn is_equal(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

/// Bit-exact key wrapper (hash consistent with strict equality).
#[derive(Clone, Copy, Debug)]
struct PntBitsKeyPt(GpPntKeyPt);

impl PartialEq for PntBitsKeyPt {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_equal(&other.0)
    }
}
impl Eq for PntBitsKeyPt {}
impl std::hash::Hash for PntBitsKeyPt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
        self.0.z.to_bits().hash(state);
    }
}

/// Local stand-in for a `Standard_Transient` value.
#[derive(Debug)]
pub struct TransientRecPt {
    pub dynamic_type: String,
}

pub type HandleTransientPt = Rc<TransientRecPt>;

/// `STEPConstruct_DataMapOfPointTransient` with NCollection semantics.
#[derive(Default)]
pub struct StepConstructDataMapOfPointTransient {
    inner: HashMap<PntBitsKeyPt, HandleTransientPt>,
}

impl StepConstructDataMapOfPointTransient {
    pub fn new() -> Self {
        StepConstructDataMapOfPointTransient {
            inner: HashMap::new(),
        }
    }

    /// Bind — true when the key is new.
    pub fn bind(&mut self, key: GpPntKeyPt, item: HandleTransientPt) -> bool {
        self.inner.insert(PntBitsKeyPt(key), item).is_none()
    }

    pub fn is_bound(&self, key: &GpPntKeyPt) -> bool {
        self.inner.contains_key(&PntBitsKeyPt(*key))
    }

    pub fn find(&self, key: &GpPntKeyPt) -> Option<&HandleTransientPt> {
        self.inner.get(&PntBitsKeyPt(*key))
    }

    pub fn un_bind(&mut self, key: &GpPntKeyPt) -> bool {
        self.inner.remove(&PntBitsKeyPt(*key)).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `STEPConstruct_DataMapIteratorOfDataMapOfPointTransient`.
    pub fn iter(&self) -> impl Iterator<Item = (&GpPntKeyPt, &HandleTransientPt)> {
        self.inner.iter().map(|(k, v)| (&k.0, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_coordinate_equality() {
        let mut map = StepConstructDataMapOfPointTransient::new();
        let p = GpPntKeyPt::new(1.0, 2.0, 3.0);
        map.bind(p, Rc::new(TransientRecPt { dynamic_type: "CartesianPoint".into() }));
        // Bit-identical point matches.
        assert!(map.is_bound(&GpPntKeyPt::new(1.0, 2.0, 3.0)));
        // A point differing by 1 ulp-ish amount does NOT match (no tolerance).
        assert!(!map.is_bound(&GpPntKeyPt::new(1.0 + 1e-12, 2.0, 3.0)));
    }

    #[test]
    fn bind_returns_novelty() {
        let mut map = StepConstructDataMapOfPointTransient::new();
        let p = GpPntKeyPt::new(0.0, 0.0, 0.0);
        assert!(map.bind(p, Rc::new(TransientRecPt { dynamic_type: "A".into() })));
        assert!(!map.bind(p, Rc::new(TransientRecPt { dynamic_type: "B".into() })));
        assert_eq!(map.find(&p).unwrap().dynamic_type, "B");
        assert_eq!(map.extent(), 1);
        assert!(map.un_bind(&p));
        assert!(map.find(&p).is_none());
    }

    #[test]
    fn multiple_points() {
        let mut map = StepConstructDataMapOfPointTransient::new();
        for i in 0..5 {
            map.bind(
                GpPntKeyPt::new(i as f64, 0.0, 0.0),
                Rc::new(TransientRecPt { dynamic_type: format!("P{i}") }),
            );
        }
        assert_eq!(map.extent(), 5);
        assert_eq!(map.find(&GpPntKeyPt::new(3.0, 0.0, 0.0)).unwrap().dynamic_type, "P3");
    }
}
