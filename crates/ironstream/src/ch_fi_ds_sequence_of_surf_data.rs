// FILE: ch_fi_ds_sequence_of_surf_data.rs
// occt: ChFiDS_SequenceOfSurfData

//! Deprecated type alias for backward compatibility.
//! Use Vec<Arc<ChFiDsSurfData>> directly instead.

use std::sync::Arc;

/// Surface data handle type (opaque marker).
pub struct ChFiDsSurfDataHandle;

/// Deprecated sequence of surface data handles.
/// Maps to NCollection_Sequence<opencascade::handle<ChFiDS_SurfData>>.
/// A sequence is an ordered collection with indexed access.
pub type ChFiDsSequenceOfSurfData = Vec<Arc<ChFiDsSurfDataHandle>>;

/// Helper methods for sequence operations.
pub struct ChFiDsSequenceOfSurfDataOps;

impl ChFiDsSequenceOfSurfDataOps {
    /// Appends an element to the sequence.
    pub fn append(seq: &mut ChFiDsSequenceOfSurfData, element: Arc<ChFiDsSurfDataHandle>) {
        seq.push(element);
    }

    /// Returns the number of elements in the sequence.
    pub fn length(seq: &ChFiDsSequenceOfSurfData) -> usize {
        seq.len()
    }

    /// Returns a reference to element at position (1-based).
    pub fn value(seq: &ChFiDsSequenceOfSurfData, index: usize) -> Option<&Arc<ChFiDsSurfDataHandle>> {
        if index == 0 {
            return None;
        }
        seq.get(index - 1)
    }

    /// Removes element at position (1-based).
    pub fn remove(seq: &mut ChFiDsSequenceOfSurfData, index: usize) -> Option<Arc<ChFiDsSurfDataHandle>> {
        if index == 0 || index > seq.len() {
            return None;
        }
        Some(seq.remove(index - 1))
    }

    /// Clears the sequence.
    pub fn clear(seq: &mut ChFiDsSequenceOfSurfData) {
        seq.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_of_surf_data_creation() {
        let seq: ChFiDsSequenceOfSurfData = Vec::new();
        assert_eq!(ChFiDsSequenceOfSurfDataOps::length(&seq), 0);
    }

    #[test]
    fn test_sequence_of_surf_data_append() {
        let mut seq: ChFiDsSequenceOfSurfData = Vec::new();
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));

        assert_eq!(ChFiDsSequenceOfSurfDataOps::length(&seq), 2);
    }

    #[test]
    fn test_sequence_of_surf_data_value() {
        let mut seq: ChFiDsSequenceOfSurfData = Vec::new();
        let data1 = Arc::new(ChFiDsSurfDataHandle);
        let data2 = Arc::new(ChFiDsSurfDataHandle);

        ChFiDsSequenceOfSurfDataOps::append(&mut seq, data1.clone());
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, data2.clone());

        assert!(ChFiDsSequenceOfSurfDataOps::value(&seq, 1).is_some());
        assert!(ChFiDsSequenceOfSurfDataOps::value(&seq, 2).is_some());
        assert!(ChFiDsSequenceOfSurfDataOps::value(&seq, 3).is_none());
    }

    #[test]
    fn test_sequence_of_surf_data_remove() {
        let mut seq: ChFiDsSequenceOfSurfData = Vec::new();
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));

        assert_eq!(ChFiDsSequenceOfSurfDataOps::length(&seq), 3);

        let removed = ChFiDsSequenceOfSurfDataOps::remove(&mut seq, 2);
        assert!(removed.is_some());
        assert_eq!(ChFiDsSequenceOfSurfDataOps::length(&seq), 2);
    }

    #[test]
    fn test_sequence_of_surf_data_clear() {
        let mut seq: ChFiDsSequenceOfSurfData = Vec::new();
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));

        assert_eq!(ChFiDsSequenceOfSurfDataOps::length(&seq), 2);

        ChFiDsSequenceOfSurfDataOps::clear(&mut seq);
        assert_eq!(ChFiDsSequenceOfSurfDataOps::length(&seq), 0);
    }

    #[test]
    fn test_sequence_of_surf_data_bounds() {
        let mut seq: ChFiDsSequenceOfSurfData = Vec::new();
        ChFiDsSequenceOfSurfDataOps::append(&mut seq, Arc::new(ChFiDsSurfDataHandle));

        // Test 1-based indexing (value at index 0 returns None)
        assert!(ChFiDsSequenceOfSurfDataOps::value(&seq, 0).is_none());
        assert!(ChFiDsSequenceOfSurfDataOps::value(&seq, 1).is_some());
    }
}
