// FILE: step_fea_h_sequence_of_element_representation.rs
// occt: StepFEA_HSequenceOfElementRepresentation

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HSequence<opencascade::handle<StepFEA_ElementRepresentation>>
//!    StepFEA_HSequenceOfElementRepresentation;`
//!
//! 1-based handle-sequence with NCollection_Sequence semantics, including
//! the Append(sequence) splice used when concatenating harvested elements.

use std::rc::Rc;

/// Local stand-in for `StepFEA_ElementRepresentation`.
#[derive(Debug)]
pub struct FeaElementReprStubSeqEr {
    pub name: String,
}

pub type HandleFeaElementReprSeqEr = Rc<FeaElementReprStubSeqEr>;

/// `StepFEA_HSequenceOfElementRepresentation`.
#[derive(Default)]
pub struct StepFeaHSequenceOfElementRepresentation {
    items: Vec<HandleFeaElementReprSeqEr>,
}

impl StepFeaHSequenceOfElementRepresentation {
    pub fn new() -> Self {
        StepFeaHSequenceOfElementRepresentation { items: Vec::new() }
    }

    pub fn length(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn append(&mut self, item: HandleFeaElementReprSeqEr) {
        self.items.push(item);
    }

    /// Append(otherSequence) — NCollection splice: other is emptied.
    pub fn append_sequence(&mut self, other: &mut StepFeaHSequenceOfElementRepresentation) {
        self.items.append(&mut other.items);
    }

    pub fn prepend(&mut self, item: HandleFeaElementReprSeqEr) {
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

    pub fn value(&self, index: i32) -> &HandleFeaElementReprSeqEr {
        &self.items[self.offset(index)]
    }

    pub fn remove(&mut self, index: i32) {
        let off = self.offset(index);
        self.items.remove(off);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn er(name: &str) -> HandleFeaElementReprSeqEr {
        Rc::new(FeaElementReprStubSeqEr { name: name.into() })
    }

    #[test]
    fn basic_sequence_semantics() {
        let mut seq = StepFeaHSequenceOfElementRepresentation::new();
        seq.append(er("beam1"));
        seq.append(er("beam2"));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1).name, "beam1");
        seq.remove(1);
        assert_eq!(seq.value(1).name, "beam2");
    }

    #[test]
    fn append_sequence_splices_and_empties_source() {
        let mut a = StepFeaHSequenceOfElementRepresentation::new();
        a.append(er("a1"));
        let mut b = StepFeaHSequenceOfElementRepresentation::new();
        b.append(er("b1"));
        b.append(er("b2"));
        a.append_sequence(&mut b);
        assert_eq!(a.length(), 3);
        assert!(b.is_empty(), "OCCT Append(Sequence&) moves nodes out of the source");
        assert_eq!(a.value(3).name, "b2");
    }
}
