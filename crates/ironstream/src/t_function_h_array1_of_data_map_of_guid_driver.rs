// FILE: t_function_h_array1_of_data_map_of_guid_driver.rs
// occt: TFunction_HArray1OfDataMapOfGUIDDriver

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HArray1<TFunction_DataMapOfGUIDDriver>
//!    TFunction_HArray1OfDataMapOfGUIDDriver;`
//!
//! An HArray1 whose items are GUID->driver data maps (one map per
//! document/table slot). The GUID and the function driver are modeled
//! locally: GUID as its canonical 16-byte value, driver as a named record.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `Standard_GUID` (16 raw bytes).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GuidKeyGd {
    pub bytes: [u8; 16],
}

impl GuidKeyGd {
    /// Builds a GUID from a short seed (test convenience, deterministic).
    pub fn from_seed(seed: u8) -> Self {
        let mut bytes = [0u8; 16];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = seed.wrapping_add(i as u8);
        }
        GuidKeyGd { bytes }
    }
}

/// Local stand-in for `TFunction_Driver`.
#[derive(Debug)]
pub struct FunctionDriverStubGd {
    pub name: String,
}

pub type HandleFunctionDriverGd = Rc<FunctionDriverStubGd>;

/// `TFunction_DataMapOfGUIDDriver` (the array item type).
#[derive(Default, Clone)]
pub struct TFunctionDataMapOfGuidDriverGd {
    inner: HashMap<GuidKeyGd, HandleFunctionDriverGd>,
}

impl TFunctionDataMapOfGuidDriverGd {
    pub fn new() -> Self {
        TFunctionDataMapOfGuidDriverGd { inner: HashMap::new() }
    }

    /// Bind — true when the GUID was not bound yet.
    pub fn bind(&mut self, guid: GuidKeyGd, driver: HandleFunctionDriverGd) -> bool {
        self.inner.insert(guid, driver).is_none()
    }

    pub fn is_bound(&self, guid: &GuidKeyGd) -> bool {
        self.inner.contains_key(guid)
    }

    pub fn find(&self, guid: &GuidKeyGd) -> Option<&HandleFunctionDriverGd> {
        self.inner.get(guid)
    }

    pub fn un_bind(&mut self, guid: &GuidKeyGd) -> bool {
        self.inner.remove(guid).is_some()
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }
}

/// `TFunction_HArray1OfDataMapOfGUIDDriver`: HArray1 with fixed bounds,
/// each slot default-constructed to an empty map.
pub struct TFunctionHArray1OfDataMapOfGuidDriver {
    lower: i32,
    upper: i32,
    data: Vec<TFunctionDataMapOfGuidDriverGd>,
}

impl TFunctionHArray1OfDataMapOfGuidDriver {
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(upper >= lower, "HArray1: upper must be >= lower");
        TFunctionHArray1OfDataMapOfGuidDriver {
            lower,
            upper,
            data: vec![TFunctionDataMapOfGuidDriverGd::new(); (upper - lower + 1) as usize],
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }

    fn offset(&self, index: i32) -> usize {
        assert!(
            index >= self.lower && index <= self.upper,
            "HArray1: index {} out of range [{}, {}]",
            index,
            self.lower,
            self.upper
        );
        (index - self.lower) as usize
    }

    /// Value(index) — the map stored in the slot.
    pub fn value(&self, index: i32) -> &TFunctionDataMapOfGuidDriverGd {
        &self.data[self.offset(index)]
    }

    /// ChangeValue(index) — mutable access for in-place binding.
    pub fn change_value(&mut self, index: i32) -> &mut TFunctionDataMapOfGuidDriverGd {
        let off = self.offset(index);
        &mut self.data[off]
    }

    /// SetValue(index, map).
    pub fn set_value(&mut self, index: i32, map: TFunctionDataMapOfGuidDriverGd) {
        let off = self.offset(index);
        self.data[off] = map;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_of_empty_maps() {
        let arr = TFunctionHArray1OfDataMapOfGuidDriver::new(1, 3);
        assert_eq!(arr.length(), 3);
        for i in 1..=3 {
            assert_eq!(arr.value(i).extent(), 0);
        }
    }

    #[test]
    fn per_slot_guid_driver_binding() {
        let mut arr = TFunctionHArray1OfDataMapOfGuidDriver::new(1, 2);
        let guid_box = GuidKeyGd::from_seed(1);
        let guid_cyl = GuidKeyGd::from_seed(2);
        arr.change_value(1)
            .bind(guid_box, Rc::new(FunctionDriverStubGd { name: "BoxDriver".into() }));
        arr.change_value(2)
            .bind(guid_cyl, Rc::new(FunctionDriverStubGd { name: "CylDriver".into() }));
        assert!(arr.value(1).is_bound(&guid_box));
        assert!(!arr.value(1).is_bound(&guid_cyl), "slot maps are independent");
        assert_eq!(arr.value(2).find(&guid_cyl).unwrap().name, "CylDriver");
    }

    #[test]
    fn guid_equality_is_by_value() {
        let mut map = TFunctionDataMapOfGuidDriverGd::new();
        let g1 = GuidKeyGd::from_seed(7);
        let g2 = GuidKeyGd::from_seed(7); // same bytes
        assert!(map.bind(g1, Rc::new(FunctionDriverStubGd { name: "D".into() })));
        assert!(!map.bind(g2, Rc::new(FunctionDriverStubGd { name: "E".into() })));
        assert_eq!(map.extent(), 1);
        assert!(map.un_bind(&g2));
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn out_of_bounds_slot_panics() {
        let arr = TFunctionHArray1OfDataMapOfGuidDriver::new(1, 2);
        let _ = arr.value(3);
    }
}
