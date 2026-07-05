// FILE: step_fea_h_array1_of_node_representation.rs
// occt: StepFEA_HArray1OfNodeRepresentation

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HArray1<opencascade::handle<StepFEA_NodeRepresentation>>
//!    StepFEA_HArray1OfNodeRepresentation;`
//!
//! Handle-array with OCCT Array1 semantics (fixed Lower/Upper bounds).

use std::rc::Rc;

/// Local stand-in for `StepFEA_NodeRepresentation`
/// (a node of a FEA model with its coordinates).
#[derive(Debug)]
pub struct FeaNodeReprStubNr {
    pub name: String,
    pub coords: [f64; 3],
}

pub type HandleFeaNodeReprNr = Rc<FeaNodeReprStubNr>;

/// `StepFEA_HArray1OfNodeRepresentation`.
pub struct StepFeaHArray1OfNodeRepresentation {
    lower: i32,
    upper: i32,
    data: Vec<Option<HandleFeaNodeReprNr>>,
}

impl StepFeaHArray1OfNodeRepresentation {
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(upper >= lower, "HArray1: upper must be >= lower");
        StepFeaHArray1OfNodeRepresentation {
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

    pub fn set_value(&mut self, index: i32, item: HandleFeaNodeReprNr) {
        let off = self.offset(index);
        self.data[off] = Some(item);
    }

    pub fn value(&self, index: i32) -> Option<&HandleFeaNodeReprNr> {
        self.data[self.offset(index)].as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_and_default_null_handles() {
        let arr = StepFeaHArray1OfNodeRepresentation::new(1, 8);
        assert_eq!((arr.lower(), arr.upper(), arr.length()), (1, 8, 8));
        for i in 1..=8 {
            assert!(arr.value(i).is_none());
        }
    }

    #[test]
    fn node_storage_roundtrip() {
        let mut arr = StepFeaHArray1OfNodeRepresentation::new(1, 2);
        arr.set_value(
            1,
            Rc::new(FeaNodeReprStubNr { name: "n1".into(), coords: [0.0, 0.0, 0.0] }),
        );
        arr.set_value(
            2,
            Rc::new(FeaNodeReprStubNr { name: "n2".into(), coords: [1.0, 2.0, 3.0] }),
        );
        assert_eq!(arr.value(2).unwrap().coords, [1.0, 2.0, 3.0]);
        assert_eq!(arr.value(1).unwrap().name, "n1");
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn below_lower_panics() {
        let arr = StepFeaHArray1OfNodeRepresentation::new(3, 5);
        let _ = arr.value(2);
    }
}
