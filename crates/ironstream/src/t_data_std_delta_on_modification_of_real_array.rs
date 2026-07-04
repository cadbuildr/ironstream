// FILE: t_data_std_delta_on_modification_of_real_array.rs
// occt: TDataStd_DeltaOnModificationOfRealArray

/// Delta tracking for modifications to a RealArray.
/// Stores indices and values that changed for efficient undo/redo.
pub struct TDataStd_DeltaOnModificationOfRealArray {
    indices: Vec<i32>,
    values: Vec<f64>,
    up1: i32,
    up2: i32,
}

impl TDataStd_DeltaOnModificationOfRealArray {
    /// Create a new delta tracking object.
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            values: Vec::new(),
            up1: 0,
            up2: 0,
        }
    }

    /// Apply this delta to restore a previous state.
    pub fn apply(&mut self) {
        // Framework operation: modifies a RealArray attribute
        // Implementation depends on framework context not available in std-only
    }

    /// Get the indices of modified elements.
    pub fn indices(&self) -> &[i32] {
        &self.indices
    }

    /// Get the values of modified elements.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Set the indices and values.
    pub fn set_changes(&mut self, indices: Vec<i32>, values: Vec<f64>, up1: i32, up2: i32) {
        self.indices = indices;
        self.values = values;
        self.up1 = up1;
        self.up2 = up2;
    }

    /// Get the upper bound 1.
    pub fn up1(&self) -> i32 {
        self.up1
    }

    /// Get the upper bound 2.
    pub fn up2(&self) -> i32 {
        self.up2
    }
}

impl Default for TDataStd_DeltaOnModificationOfRealArray {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_delta() {
        let delta = TDataStd_DeltaOnModificationOfRealArray::new();
        assert_eq!(delta.indices().len(), 0);
        assert_eq!(delta.values().len(), 0);
    }

    #[test]
    fn test_set_changes() {
        let mut delta = TDataStd_DeltaOnModificationOfRealArray::new();
        let indices = vec![1, 3, 5];
        let values = vec![1.5, 2.5, 3.5];

        delta.set_changes(indices.clone(), values.clone(), 10, 20);

        assert_eq!(delta.indices(), &indices[..]);
        assert_eq!(delta.values(), &values[..]);
        assert_eq!(delta.up1(), 10);
        assert_eq!(delta.up2(), 20);
    }

    #[test]
    fn test_default() {
        let delta = TDataStd_DeltaOnModificationOfRealArray::default();
        assert_eq!(delta.indices().len(), 0);
        assert_eq!(delta.values().len(), 0);
    }
}
