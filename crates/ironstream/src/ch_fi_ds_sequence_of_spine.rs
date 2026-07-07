// FILE: ch_fi_ds_sequence_of_spine.rs
// occt: ChFiDS_SequenceOfSpine

//! Deprecated type alias for backward compatibility.
//! Use Vec<Arc<ChFiDsSpine>> directly instead.

use std::sync::Arc;

/// Spine handle type (opaque marker).
pub struct ChFiDsSpineHandle;

/// Deprecated sequence of spine handles.
/// Maps to NCollection_Sequence<opencascade::handle<ChFiDS_Spine>>.
/// A sequence is an ordered collection with indexed access.
pub type ChFiDsSequenceOfSpine = Vec<Arc<ChFiDsSpineHandle>>;

/// Helper methods for sequence operations.
pub struct ChFiDsSequenceOfSpineOps;

impl ChFiDsSequenceOfSpineOps {
    /// Appends an element to the sequence.
    pub fn append(seq: &mut ChFiDsSequenceOfSpine, element: Arc<ChFiDsSpineHandle>) {
        seq.push(element);
    }

    /// Returns the number of elements in the sequence.
    pub fn length(seq: &ChFiDsSequenceOfSpine) -> usize {
        seq.len()
    }

    /// Returns a reference to element at position (1-based).
    pub fn value(seq: &ChFiDsSequenceOfSpine, index: usize) -> Option<&Arc<ChFiDsSpineHandle>> {
        if index == 0 {
            return None;
        }
        seq.get(index - 1)
    }

    /// Removes element at position (1-based).
    pub fn remove(seq: &mut ChFiDsSequenceOfSpine, index: usize) -> Option<Arc<ChFiDsSpineHandle>> {
        if index == 0 || index > seq.len() {
            return None;
        }
        Some(seq.remove(index - 1))
    }

    /// Clears the sequence.
    pub fn clear(seq: &mut ChFiDsSequenceOfSpine) {
        seq.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_of_spine_creation() {
        let seq: ChFiDsSequenceOfSpine = Vec::new();
        assert_eq!(ChFiDsSequenceOfSpineOps::length(&seq), 0);
    }

    #[test]
    fn test_sequence_of_spine_append() {
        let mut seq: ChFiDsSequenceOfSpine = Vec::new();
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));

        assert_eq!(ChFiDsSequenceOfSpineOps::length(&seq), 2);
    }

    #[test]
    fn test_sequence_of_spine_value() {
        let mut seq: ChFiDsSequenceOfSpine = Vec::new();
        let spine1 = Arc::new(ChFiDsSpineHandle);
        let spine2 = Arc::new(ChFiDsSpineHandle);

        ChFiDsSequenceOfSpineOps::append(&mut seq, spine1.clone());
        ChFiDsSequenceOfSpineOps::append(&mut seq, spine2.clone());

        assert!(ChFiDsSequenceOfSpineOps::value(&seq, 1).is_some());
        assert!(ChFiDsSequenceOfSpineOps::value(&seq, 2).is_some());
        assert!(ChFiDsSequenceOfSpineOps::value(&seq, 3).is_none());
    }

    #[test]
    fn test_sequence_of_spine_remove() {
        let mut seq: ChFiDsSequenceOfSpine = Vec::new();
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));

        assert_eq!(ChFiDsSequenceOfSpineOps::length(&seq), 3);

        let removed = ChFiDsSequenceOfSpineOps::remove(&mut seq, 2);
        assert!(removed.is_some());
        assert_eq!(ChFiDsSequenceOfSpineOps::length(&seq), 2);
    }

    #[test]
    fn test_sequence_of_spine_clear() {
        let mut seq: ChFiDsSequenceOfSpine = Vec::new();
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));

        assert_eq!(ChFiDsSequenceOfSpineOps::length(&seq), 2);

        ChFiDsSequenceOfSpineOps::clear(&mut seq);
        assert_eq!(ChFiDsSequenceOfSpineOps::length(&seq), 0);
    }

    #[test]
    fn test_sequence_of_spine_bounds() {
        let mut seq: ChFiDsSequenceOfSpine = Vec::new();
        ChFiDsSequenceOfSpineOps::append(&mut seq, Arc::new(ChFiDsSpineHandle));

        // Test 1-based indexing (value at index 0 returns None)
        assert!(ChFiDsSequenceOfSpineOps::value(&seq, 0).is_none());
        assert!(ChFiDsSequenceOfSpineOps::value(&seq, 1).is_some());
    }
}
