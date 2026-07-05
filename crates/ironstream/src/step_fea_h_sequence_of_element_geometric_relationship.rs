// FILE: step_fea_h_sequence_of_element_geometric_relationship.rs
// occt: StepFEA_HSequenceOfElementGeometricRelationship

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HSequence<opencascade::handle<StepFEA_ElementGeometricRelationship>>
//!    StepFEA_HSequenceOfElementGeometricRelationship;`
//!
//! 1-based handle-sequence with NCollection_Sequence semantics.

use std::rc::Rc;

/// Local stand-in for `StepFEA_ElementGeometricRelationship`
/// (association between an element representation and its analysed item).
#[derive(Debug)]
pub struct FeaElemGeomRelStubGr {
    /// Name of the element representation side.
    pub element_ref: String,
    /// Name of the analysed geometric item.
    pub item_ref: String,
    /// Aspect of the element (e.g. whole element, volume, surface...).
    pub aspect: String,
}

pub type HandleFeaElemGeomRelGr = Rc<FeaElemGeomRelStubGr>;

/// `StepFEA_HSequenceOfElementGeometricRelationship`.
#[derive(Default)]
pub struct StepFeaHSequenceOfElementGeometricRelationship {
    items: Vec<HandleFeaElemGeomRelGr>,
}

impl StepFeaHSequenceOfElementGeometricRelationship {
    pub fn new() -> Self {
        StepFeaHSequenceOfElementGeometricRelationship { items: Vec::new() }
    }

    pub fn length(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn append(&mut self, item: HandleFeaElemGeomRelGr) {
        self.items.push(item);
    }

    pub fn prepend(&mut self, item: HandleFeaElemGeomRelGr) {
        self.items.insert(0, item);
    }

    fn offset(&self, index: i32) -> usize {
        assert!(
            index >= 1 && index <= self.items.len() as i32,
            "Sequence: index {} out of range [1, {}]",
            index,
            self.items.len()
        );
        (index - 1) as usize
    }

    pub fn value(&self, index: i32) -> &HandleFeaElemGeomRelGr {
        &self.items[self.offset(index)]
    }

    pub fn set_value(&mut self, index: i32, item: HandleFeaElemGeomRelGr) {
        let off = self.offset(index);
        self.items[off] = item;
    }

    pub fn remove(&mut self, index: i32) {
        let off = self.offset(index);
        self.items.remove(off);
    }

    /// Exchange two items (NCollection_Sequence::Exchange).
    pub fn exchange(&mut self, i: i32, j: i32) {
        let a = self.offset(i);
        let b = self.offset(j);
        self.items.swap(a, b);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(elem: &str, item: &str) -> HandleFeaElemGeomRelGr {
        Rc::new(FeaElemGeomRelStubGr {
            element_ref: elem.into(),
            item_ref: item.into(),
            aspect: "whole_element".into(),
        })
    }

    #[test]
    fn sequence_grows_one_based() {
        let mut seq = StepFeaHSequenceOfElementGeometricRelationship::new();
        assert!(seq.is_empty());
        seq.append(rel("e1", "face_1"));
        seq.append(rel("e2", "face_2"));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1).element_ref, "e1");
        assert_eq!(seq.value(2).item_ref, "face_2");
    }

    #[test]
    fn exchange_swaps_items() {
        let mut seq = StepFeaHSequenceOfElementGeometricRelationship::new();
        seq.append(rel("a", "ia"));
        seq.append(rel("b", "ib"));
        seq.exchange(1, 2);
        assert_eq!(seq.value(1).element_ref, "b");
        assert_eq!(seq.value(2).element_ref, "a");
    }

    #[test]
    fn remove_shifts_left() {
        let mut seq = StepFeaHSequenceOfElementGeometricRelationship::new();
        seq.append(rel("a", "1"));
        seq.append(rel("b", "2"));
        seq.append(rel("c", "3"));
        seq.remove(2);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(2).element_ref, "c");
    }
}
