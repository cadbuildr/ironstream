// FILE: contap_sequence_of_iw_line_of_the_i_walking.rs
// occt: Contap_SequenceOfIWLineOfTheIWalking

//! Deprecated type alias for backward compatibility.
//! Use Vec<Arc<ContapTheIWLineOfTheIWalking>> directly instead.

use std::sync::Arc;

/// IWLine (intersection walking line) handle type (opaque marker).
pub struct ContapTheIWLineOfTheIWalkingHandle;

/// Deprecated sequence of IWLine handles.
/// Maps to NCollection_Sequence<opencascade::handle<Contap_TheIWLineOfTheIWalking>>.
/// A sequence is an ordered collection with indexed access.
pub type ContapSequenceOfIWLineOfTheIWalking = Vec<Arc<ContapTheIWLineOfTheIWalkingHandle>>;

/// Helper methods for sequence operations.
pub struct ContapSequenceOfIWLineOfTheIWalkingOps;

impl ContapSequenceOfIWLineOfTheIWalkingOps {
    /// Appends an element to the sequence.
    pub fn append(seq: &mut ContapSequenceOfIWLineOfTheIWalking, element: Arc<ContapTheIWLineOfTheIWalkingHandle>) {
        seq.push(element);
    }

    /// Returns the number of elements in the sequence.
    pub fn length(seq: &ContapSequenceOfIWLineOfTheIWalking) -> usize {
        seq.len()
    }

    /// Returns a reference to element at position (1-based).
    pub fn value(seq: &ContapSequenceOfIWLineOfTheIWalking, index: usize) -> Option<&Arc<ContapTheIWLineOfTheIWalkingHandle>> {
        if index == 0 {
            return None;
        }
        seq.get(index - 1)
    }

    /// Removes element at position (1-based).
    pub fn remove(seq: &mut ContapSequenceOfIWLineOfTheIWalking, index: usize) -> Option<Arc<ContapTheIWLineOfTheIWalkingHandle>> {
        if index == 0 || index > seq.len() {
            return None;
        }
        Some(seq.remove(index - 1))
    }

    /// Clears the sequence.
    pub fn clear(seq: &mut ContapSequenceOfIWLineOfTheIWalking) {
        seq.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_of_iw_line_creation() {
        let seq: ContapSequenceOfIWLineOfTheIWalking = Vec::new();
        assert_eq!(ContapSequenceOfIWLineOfTheIWalkingOps::length(&seq), 0);
    }

    #[test]
    fn test_sequence_of_iw_line_append() {
        let mut seq: ContapSequenceOfIWLineOfTheIWalking = Vec::new();
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));

        assert_eq!(ContapSequenceOfIWLineOfTheIWalkingOps::length(&seq), 2);
    }

    #[test]
    fn test_sequence_of_iw_line_value() {
        let mut seq: ContapSequenceOfIWLineOfTheIWalking = Vec::new();
        let line1 = Arc::new(ContapTheIWLineOfTheIWalkingHandle);
        let line2 = Arc::new(ContapTheIWLineOfTheIWalkingHandle);

        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, line1.clone());
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, line2.clone());

        assert!(ContapSequenceOfIWLineOfTheIWalkingOps::value(&seq, 1).is_some());
        assert!(ContapSequenceOfIWLineOfTheIWalkingOps::value(&seq, 2).is_some());
        assert!(ContapSequenceOfIWLineOfTheIWalkingOps::value(&seq, 3).is_none());
    }

    #[test]
    fn test_sequence_of_iw_line_remove() {
        let mut seq: ContapSequenceOfIWLineOfTheIWalking = Vec::new();
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));

        assert_eq!(ContapSequenceOfIWLineOfTheIWalkingOps::length(&seq), 3);

        let removed = ContapSequenceOfIWLineOfTheIWalkingOps::remove(&mut seq, 2);
        assert!(removed.is_some());
        assert_eq!(ContapSequenceOfIWLineOfTheIWalkingOps::length(&seq), 2);
    }

    #[test]
    fn test_sequence_of_iw_line_clear() {
        let mut seq: ContapSequenceOfIWLineOfTheIWalking = Vec::new();
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));

        assert_eq!(ContapSequenceOfIWLineOfTheIWalkingOps::length(&seq), 2);

        ContapSequenceOfIWLineOfTheIWalkingOps::clear(&mut seq);
        assert_eq!(ContapSequenceOfIWLineOfTheIWalkingOps::length(&seq), 0);
    }

    #[test]
    fn test_sequence_of_iw_line_bounds() {
        let mut seq: ContapSequenceOfIWLineOfTheIWalking = Vec::new();
        ContapSequenceOfIWLineOfTheIWalkingOps::append(&mut seq, Arc::new(ContapTheIWLineOfTheIWalkingHandle));

        // Test 1-based indexing (value at index 0 returns None)
        assert!(ContapSequenceOfIWLineOfTheIWalkingOps::value(&seq, 0).is_none());
        assert!(ContapSequenceOfIWLineOfTheIWalkingOps::value(&seq, 1).is_some());
    }
}
