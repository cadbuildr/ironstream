// FILE: t_data_std_boolean_array.rs
// occt: TDataStd_BooleanArray

/// Attribute storing an array of boolean values.
pub struct TDataStdBooleanArray {
    values: Vec<bool>,
}

impl TDataStdBooleanArray {
    /// Creates a new boolean array.
    pub fn new() -> Self {
        TDataStdBooleanArray {
            values: Vec::new(),
        }
    }

    /// Creates a boolean array with a given size.
    pub fn with_size(size: usize) -> Self {
        TDataStdBooleanArray {
            values: vec![false; size],
        }
    }

    /// Sets a value at the given index.
    pub fn set(&mut self, index: usize, value: bool) {
        if index < self.values.len() {
            self.values[index] = value;
        }
    }

    /// Gets a value at the given index.
    pub fn get(&self, index: usize) -> Option<bool> {
        self.values.get(index).copied()
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for TDataStdBooleanArray {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_array_new() {
        let arr = TDataStdBooleanArray::new();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_boolean_array_with_size() {
        let arr = TDataStdBooleanArray::with_size(5);
        assert_eq!(arr.len(), 5);
        assert_eq!(arr.get(0), Some(false));
    }

    #[test]
    fn test_boolean_array_set_get() {
        let mut arr = TDataStdBooleanArray::with_size(3);
        arr.set(1, true);
        assert_eq!(arr.get(1), Some(true));
        assert_eq!(arr.get(0), Some(false));
    }

    #[test]
    fn test_boolean_array_default() {
        let arr = TDataStdBooleanArray::default();
        assert!(arr.is_empty());
    }
}
