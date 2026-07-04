// FILE: t_doc_std_compound_delta.rs
// occt: TDocStd_CompoundDelta

use std::collections::VecDeque;

/// A compound delta that groups multiple deltas together.
/// Used for managing complex undo/redo operations composed of multiple changes.
#[derive(Clone, Debug)]
pub struct TDocStd_CompoundDelta {
    deltas: VecDeque<String>,
}

impl TDocStd_CompoundDelta {
    /// Create a new compound delta.
    pub fn new() -> Self {
        Self {
            deltas: VecDeque::new(),
        }
    }

    /// Add a delta to the compound.
    pub fn add_delta(&mut self, delta: String) {
        self.deltas.push_back(delta);
    }

    /// Get the number of deltas.
    pub fn delta_count(&self) -> usize {
        self.deltas.len()
    }

    /// Check if the compound is empty.
    pub fn is_empty(&self) -> bool {
        self.deltas.is_empty()
    }

    /// Get a delta by index.
    pub fn get_delta(&self, index: usize) -> Option<&str> {
        self.deltas.get(index).map(|s| s.as_str())
    }

    /// Clear all deltas.
    pub fn clear(&mut self) {
        self.deltas.clear();
    }
}

impl Default for TDocStd_CompoundDelta {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_compound() {
        let compound = TDocStd_CompoundDelta::new();
        assert!(compound.is_empty());
    }

    #[test]
    fn test_add_delta() {
        let mut compound = TDocStd_CompoundDelta::new();
        compound.add_delta("delta1".to_string());
        compound.add_delta("delta2".to_string());
        assert_eq!(compound.delta_count(), 2);
    }

    #[test]
    fn test_get_delta() {
        let mut compound = TDocStd_CompoundDelta::new();
        compound.add_delta("delta1".to_string());
        assert_eq!(compound.get_delta(0), Some("delta1"));
    }

    #[test]
    fn test_clear() {
        let mut compound = TDocStd_CompoundDelta::new();
        compound.add_delta("delta1".to_string());
        compound.clear();
        assert!(compound.is_empty());
    }

    #[test]
    fn test_default() {
        let compound = TDocStd_CompoundDelta::default();
        assert!(compound.is_empty());
    }
}
