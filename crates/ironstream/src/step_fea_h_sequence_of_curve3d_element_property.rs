// FILE: step_fea_h_sequence_of_curve3d_element_property.rs
// occt: StepFEA_HSequenceOfCurve3dElementProperty

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HSequence<opencascade::handle<StepFEA_Curve3dElementProperty>>
//!    StepFEA_HSequenceOfCurve3dElementProperty;`
//!
//! Handle-sequence with OCCT Sequence semantics: 1-based indices,
//! Append/Prepend/Value/Remove/Length/First/Last.

use std::rc::Rc;

/// Local stand-in for `StepFEA_Curve3dElementProperty`
/// (interval definitions of a 3D curve element).
#[derive(Debug)]
pub struct FeaCurve3dPropStubCp {
    pub property_id: String,
    pub description: String,
}

pub type HandleFeaCurve3dPropCp = Rc<FeaCurve3dPropStubCp>;

/// `StepFEA_HSequenceOfCurve3dElementProperty`.
#[derive(Default)]
pub struct StepFeaHSequenceOfCurve3dElementProperty {
    items: Vec<HandleFeaCurve3dPropCp>,
}

impl StepFeaHSequenceOfCurve3dElementProperty {
    pub fn new() -> Self {
        StepFeaHSequenceOfCurve3dElementProperty { items: Vec::new() }
    }

    pub fn length(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Append — item becomes the last (index Length()).
    pub fn append(&mut self, item: HandleFeaCurve3dPropCp) {
        self.items.push(item);
    }

    /// Prepend — item becomes the first (index 1).
    pub fn prepend(&mut self, item: HandleFeaCurve3dPropCp) {
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

    /// Value(index) — 1-based.
    pub fn value(&self, index: i32) -> &HandleFeaCurve3dPropCp {
        &self.items[self.offset(index)]
    }

    /// SetValue(index, item) — 1-based replacement.
    pub fn set_value(&mut self, index: i32, item: HandleFeaCurve3dPropCp) {
        let off = self.offset(index);
        self.items[off] = item;
    }

    /// InsertAfter(index, item).
    pub fn insert_after(&mut self, index: i32, item: HandleFeaCurve3dPropCp) {
        let off = self.offset(index);
        self.items.insert(off + 1, item);
    }

    /// Remove(index) — 1-based.
    pub fn remove(&mut self, index: i32) {
        let off = self.offset(index);
        self.items.remove(off);
    }

    pub fn first(&self) -> &HandleFeaCurve3dPropCp {
        assert!(!self.items.is_empty(), "Sequence: First on empty sequence");
        &self.items[0]
    }

    pub fn last(&self) -> &HandleFeaCurve3dPropCp {
        assert!(!self.items.is_empty(), "Sequence: Last on empty sequence");
        self.items.last().unwrap()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prop(id: &str) -> HandleFeaCurve3dPropCp {
        Rc::new(FeaCurve3dPropStubCp { property_id: id.into(), description: String::new() })
    }

    #[test]
    fn append_prepend_order() {
        let mut seq = StepFeaHSequenceOfCurve3dElementProperty::new();
        seq.append(prop("mid"));
        seq.append(prop("end"));
        seq.prepend(prop("start"));
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(1).property_id, "start");
        assert_eq!(seq.value(2).property_id, "mid");
        assert_eq!(seq.value(3).property_id, "end");
        assert_eq!(seq.first().property_id, "start");
        assert_eq!(seq.last().property_id, "end");
    }

    #[test]
    fn insert_after_and_remove() {
        let mut seq = StepFeaHSequenceOfCurve3dElementProperty::new();
        seq.append(prop("a"));
        seq.append(prop("c"));
        seq.insert_after(1, prop("b"));
        assert_eq!(seq.value(2).property_id, "b");
        seq.remove(1);
        assert_eq!(seq.value(1).property_id, "b");
        assert_eq!(seq.length(), 2);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn zero_index_is_invalid() {
        let mut seq = StepFeaHSequenceOfCurve3dElementProperty::new();
        seq.append(prop("x"));
        let _ = seq.value(0);
    }
}
