// FILE: step_fea_h_sequence_of_node_representation.rs
// occt: StepFEA_HSequenceOfNodeRepresentation

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HSequence<opencascade::handle<StepFEA_NodeRepresentation>>
//!    StepFEA_HSequenceOfNodeRepresentation;`
//!
//! 1-based handle-sequence with NCollection_Sequence semantics.

use std::rc::Rc;

/// Local stand-in for `StepFEA_NodeRepresentation`.
#[derive(Debug)]
pub struct FeaNodeReprStubSeqNr {
    pub name: String,
    pub model_ref: String,
}

pub type HandleFeaNodeReprSeqNr = Rc<FeaNodeReprStubSeqNr>;

/// `StepFEA_HSequenceOfNodeRepresentation`.
#[derive(Default)]
pub struct StepFeaHSequenceOfNodeRepresentation {
    items: Vec<HandleFeaNodeReprSeqNr>,
}

impl StepFeaHSequenceOfNodeRepresentation {
    pub fn new() -> Self {
        StepFeaHSequenceOfNodeRepresentation { items: Vec::new() }
    }

    pub fn length(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn append(&mut self, item: HandleFeaNodeReprSeqNr) {
        self.items.push(item);
    }

    pub fn prepend(&mut self, item: HandleFeaNodeReprSeqNr) {
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

    pub fn value(&self, index: i32) -> &HandleFeaNodeReprSeqNr {
        &self.items[self.offset(index)]
    }

    pub fn set_value(&mut self, index: i32, item: HandleFeaNodeReprSeqNr) {
        let off = self.offset(index);
        self.items[off] = item;
    }

    pub fn remove(&mut self, index: i32) {
        let off = self.offset(index);
        self.items.remove(off);
    }

    pub fn first(&self) -> &HandleFeaNodeReprSeqNr {
        assert!(!self.items.is_empty(), "Sequence: First on empty sequence");
        &self.items[0]
    }

    pub fn last(&self) -> &HandleFeaNodeReprSeqNr {
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

    fn node(name: &str) -> HandleFeaNodeReprSeqNr {
        Rc::new(FeaNodeReprStubSeqNr { name: name.into(), model_ref: "fea_model_1".into() })
    }

    #[test]
    fn append_and_one_based_access() {
        let mut seq = StepFeaHSequenceOfNodeRepresentation::new();
        for i in 1..=4 {
            seq.append(node(&format!("n{i}")));
        }
        assert_eq!(seq.length(), 4);
        assert_eq!(seq.value(1).name, "n1");
        assert_eq!(seq.value(4).name, "n4");
        assert_eq!(seq.first().name, "n1");
        assert_eq!(seq.last().name, "n4");
    }

    #[test]
    fn set_value_replaces() {
        let mut seq = StepFeaHSequenceOfNodeRepresentation::new();
        seq.append(node("old"));
        seq.set_value(1, node("new"));
        assert_eq!(seq.value(1).name, "new");
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn remove_then_empty() {
        let mut seq = StepFeaHSequenceOfNodeRepresentation::new();
        seq.append(node("only"));
        seq.remove(1);
        assert!(seq.is_empty());
    }
}
