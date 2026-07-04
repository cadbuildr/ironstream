// FILE: t_data_std_delta_on_modification_of_int_packed_map.rs
// occt: TDataStd_DeltaOnModificationOfIntPackedMap

/// Delta tracking for modifications to an IntPackedMap.
/// Stores additions and deletions as packed maps for efficient undo/redo.
pub struct TDataStd_DeltaOnModificationOfIntPackedMap {
    addition: Option<Vec<i32>>,
    deletion: Option<Vec<i32>>,
}

impl TDataStd_DeltaOnModificationOfIntPackedMap {
    /// Create a new delta tracking object.
    pub fn new() -> Self {
        Self {
            addition: None,
            deletion: None,
        }
    }

    /// Apply this delta to restore a previous state.
    pub fn apply(&mut self) {
        // Framework operation: modifies an IntPackedMap attribute
        // Implementation depends on framework context not available in std-only
    }

    /// Set the addition map.
    pub fn set_addition(&mut self, map: Vec<i32>) {
        self.addition = Some(map);
    }

    /// Set the deletion map.
    pub fn set_deletion(&mut self, map: Vec<i32>) {
        self.deletion = Some(map);
    }

    /// Get the addition map.
    pub fn addition(&self) -> Option<&[i32]> {
        self.addition.as_deref()
    }

    /// Get the deletion map.
    pub fn deletion(&self) -> Option<&[i32]> {
        self.deletion.as_deref()
    }
}

impl Default for TDataStd_DeltaOnModificationOfIntPackedMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_delta() {
        let delta = TDataStd_DeltaOnModificationOfIntPackedMap::new();
        assert!(delta.addition().is_none());
        assert!(delta.deletion().is_none());
    }

    #[test]
    fn test_set_maps() {
        let mut delta = TDataStd_DeltaOnModificationOfIntPackedMap::new();
        let add_map = vec![1, 2, 3];
        let del_map = vec![4, 5];

        delta.set_addition(add_map.clone());
        delta.set_deletion(del_map.clone());

        assert_eq!(delta.addition(), Some(&add_map[..]));
        assert_eq!(delta.deletion(), Some(&del_map[..]));
    }

    #[test]
    fn test_default() {
        let delta = TDataStd_DeltaOnModificationOfIntPackedMap::default();
        assert!(delta.addition().is_none());
        assert!(delta.deletion().is_none());
    }
}
