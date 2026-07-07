// FILE: pcdm_sequence_of_reference.rs
// occt: PCDM_SequenceOfReference

//! Deprecated: PCDM_SequenceOfReference is a type alias for NCollection_Sequence<Handle(PCDM_Reference)>.

use std::collections::VecDeque;

/// Reference handle placeholder
#[derive(Debug, Clone)]
pub struct Reference {
    id: u32,
}

impl Reference {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Sequence of references
#[derive(Debug, Clone)]
pub struct Sequence {
    references: VecDeque<Reference>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            references: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.references.is_empty()
    }

    pub fn len(&self) -> usize {
        self.references.len()
    }

    pub fn append(&mut self, ref_: Reference) {
        self.references.push_back(ref_);
    }

    pub fn first(&self) -> Option<&Reference> {
        self.references.front()
    }

    pub fn last(&self) -> Option<&Reference> {
        self.references.back()
    }

    pub fn value(&self, index: usize) -> Option<&Reference> {
        self.references.get(index)
    }

    pub fn clear(&mut self) {
        self.references.clear();
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

pub type PcdmSequenceOfReference = Sequence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let seq: PcdmSequenceOfReference = Sequence::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = Sequence::new();
        seq.append(Reference::new(1));
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_value() {
        let mut seq = Sequence::new();
        seq.append(Reference::new(42));
        assert_eq!(seq.value(0).unwrap().id(), 42);
    }

    #[test]
    fn test_default() {
        let seq = Sequence::default();
        assert!(seq.is_empty());
    }
}
