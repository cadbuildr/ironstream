// FILE: step_fea_h_array1_of_element_representation.rs
// occt: StepFEA_HArray1OfElementRepresentation

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HArray1<opencascade::handle<StepFEA_ElementRepresentation>>
//!    StepFEA_HArray1OfElementRepresentation;`
//!
//! Handle-array with OCCT Array1 semantics: fixed bounds set at creation,
//! arbitrary Lower/Upper (typically 1-based), Value/SetValue by index.

use std::rc::Rc;

/// Local stand-in for `StepFEA_ElementRepresentation`
/// (a representation whose items reference FEA nodes).
#[derive(Debug)]
pub struct FeaElementReprStubEr {
    pub name: String,
    /// Node ids referenced by this element representation.
    pub node_ids: Vec<i32>,
}

pub type HandleFeaElementReprEr = Rc<FeaElementReprStubEr>;

/// `StepFEA_HArray1OfElementRepresentation`.
pub struct StepFeaHArray1OfElementRepresentation {
    lower: i32,
    upper: i32,
    data: Vec<Option<HandleFeaElementReprEr>>,
}

impl StepFeaHArray1OfElementRepresentation {
    /// Creates an array with bounds [lower, upper]; items are null handles.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(upper >= lower, "HArray1: upper must be >= lower");
        StepFeaHArray1OfElementRepresentation {
            lower,
            upper,
            data: vec![None; (upper - lower + 1) as usize],
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

    /// SetValue(index, item).
    pub fn set_value(&mut self, index: i32, item: HandleFeaElementReprEr) {
        let off = self.offset(index);
        self.data[off] = Some(item);
    }

    /// Value(index) — None models a null handle.
    pub fn value(&self, index: i32) -> Option<&HandleFeaElementReprEr> {
        self.data[self.offset(index)].as_ref()
    }

    /// Init(item) — fill the whole array with one handle.
    pub fn init(&mut self, item: HandleFeaElementReprEr) {
        for slot in self.data.iter_mut() {
            *slot = Some(item.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_based_bounds() {
        let arr = StepFeaHArray1OfElementRepresentation::new(1, 4);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 4);
        assert_eq!(arr.length(), 4);
        assert!(arr.value(1).is_none(), "fresh slots are null handles");
    }

    #[test]
    fn set_and_get_by_occt_index() {
        let mut arr = StepFeaHArray1OfElementRepresentation::new(1, 3);
        arr.set_value(
            2,
            Rc::new(FeaElementReprStubEr { name: "elem2".into(), node_ids: vec![10, 11, 12] }),
        );
        assert_eq!(arr.value(2).unwrap().name, "elem2");
        assert_eq!(arr.value(2).unwrap().node_ids.len(), 3);
        assert!(arr.value(1).is_none());
        assert!(arr.value(3).is_none());
    }

    #[test]
    fn custom_lower_bound() {
        let mut arr = StepFeaHArray1OfElementRepresentation::new(5, 7);
        arr.set_value(5, Rc::new(FeaElementReprStubEr { name: "first".into(), node_ids: vec![] }));
        arr.set_value(7, Rc::new(FeaElementReprStubEr { name: "last".into(), node_ids: vec![] }));
        assert_eq!(arr.length(), 3);
        assert_eq!(arr.value(5).unwrap().name, "first");
        assert_eq!(arr.value(7).unwrap().name, "last");
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn out_of_range_panics() {
        let arr = StepFeaHArray1OfElementRepresentation::new(1, 2);
        let _ = arr.value(3);
    }

    #[test]
    fn init_fills_shared_handle() {
        let mut arr = StepFeaHArray1OfElementRepresentation::new(1, 3);
        let e = Rc::new(FeaElementReprStubEr { name: "shared".into(), node_ids: vec![1] });
        arr.init(e.clone());
        for i in 1..=3 {
            assert!(Rc::ptr_eq(arr.value(i).unwrap(), &e));
        }
    }
}
