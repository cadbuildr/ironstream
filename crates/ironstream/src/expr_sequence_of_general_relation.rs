// FILE: expr_sequence_of_general_relation.rs
// occt: Expr_SequenceOfGeneralRelation

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a handle to a GeneralRelation (simulated via Rc).
pub type GeneralRelationHandle = Rc<RefCell<GeneralRelation>>;

/// A general relation (e.g., constraint like "a = b").
#[derive(Clone, Debug)]
pub struct GeneralRelation {
    // Placeholder for relation content
}

/// Deprecated: Sequence of handles to GeneralRelation.
/// Use SequenceOfGeneralRelation instead of directly using Vec<GeneralRelationHandle>.
/// This is a newtype alias over a Vec for type safety, matching OCCT's deprecated typedef.
#[derive(Clone, Debug)]
pub struct SequenceOfGeneralRelation {
    items: Vec<GeneralRelationHandle>,
}

impl SequenceOfGeneralRelation {
    /// Create an empty sequence.
    pub fn new() -> Self {
        SequenceOfGeneralRelation {
            items: Vec::new(),
        }
    }

    /// Append a relation handle to the sequence.
    pub fn append(&mut self, rel: GeneralRelationHandle) {
        self.items.push(rel);
    }

    /// Return the number of items in the sequence.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the i-th item (1-indexed, like OCCT).
    pub fn at(&self, i: usize) -> Option<GeneralRelationHandle> {
        if i > 0 && i <= self.items.len() {
            Some(self.items[i - 1].clone())
        } else {
            None
        }
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Iterate over the relations.
    pub fn iter(&self) -> std::slice::Iter<GeneralRelationHandle> {
        self.items.iter()
    }
}

impl Default for SequenceOfGeneralRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_sequence() {
        let seq = SequenceOfGeneralRelation::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_single() {
        let mut seq = SequenceOfGeneralRelation::new();
        let rel = Rc::new(RefCell::new(GeneralRelation {}));
        seq.append(rel.clone());

        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
        assert!(seq.at(1).is_some());
        assert!(seq.at(2).is_none());
    }

    #[test]
    fn test_append_multiple() {
        let mut seq = SequenceOfGeneralRelation::new();
        let r1 = Rc::new(RefCell::new(GeneralRelation {}));
        let r2 = Rc::new(RefCell::new(GeneralRelation {}));
        let r3 = Rc::new(RefCell::new(GeneralRelation {}));

        seq.append(r1);
        seq.append(r2);
        seq.append(r3);

        assert_eq!(seq.len(), 3);
        assert!(seq.at(1).is_some());
        assert!(seq.at(2).is_some());
        assert!(seq.at(3).is_some());
        assert!(seq.at(4).is_none());
    }

    #[test]
    fn test_clear() {
        let mut seq = SequenceOfGeneralRelation::new();
        let r1 = Rc::new(RefCell::new(GeneralRelation {}));
        seq.append(r1);

        assert_eq!(seq.len(), 1);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = SequenceOfGeneralRelation::new();
        let r1 = Rc::new(RefCell::new(GeneralRelation {}));
        let r2 = Rc::new(RefCell::new(GeneralRelation {}));

        seq.append(r1);
        seq.append(r2);

        let count = seq.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_clone() {
        let mut seq = SequenceOfGeneralRelation::new();
        let r1 = Rc::new(RefCell::new(GeneralRelation {}));
        seq.append(r1);

        let seq2 = seq.clone();
        assert_eq!(seq.len(), seq2.len());
    }
}
