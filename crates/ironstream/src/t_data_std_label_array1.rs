// FILE: t_data_std_label_array1.rs
// occt: TDataStd_LabelArray1

//! Deprecated typedef for TDataStd_LabelArray1.
//!
//! In OCCT, this was a 1-indexed array of TDF_Label items.
//! We implement a minimal array structure using Vec with 1-based indexing semantics.

use std::fmt;

/// TDataStd_LabelArray1: A 1-indexed array of TDF_Label items (deprecated typedef).
/// Wraps a Vec and provides 1-based indexing semantics matching OCCT Array1<TDF_Label> behavior.
#[derive(Clone)]
pub struct TDataStdLabelArray1 {
    items: Vec<String>,  // Placeholder: would be Vec<TdfLabel> in full port
    lower: usize,
}

impl TDataStdLabelArray1 {
    /// Create a new array with bounds [lower, upper].
    /// In OCCT, arrays are 1-indexed by default unless otherwise specified.
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        TDataStdLabelArray1 {
            items: vec!["".to_string(); size],
            lower,
        }
    }

    /// Create a standard 1-indexed array of size n.
    pub fn new_1indexed(n: usize) -> Self {
        TDataStdLabelArray1 {
            items: vec!["".to_string(); n],
            lower: 1,
        }
    }

    /// Get the lower bound.
    pub fn lower_bound(&self) -> usize {
        self.lower
    }

    /// Get the upper bound.
    pub fn upper_bound(&self) -> usize {
        if self.items.is_empty() {
            self.lower
        } else {
            self.lower + self.items.len() - 1
        }
    }

    /// Get the size of the array.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get value at 1-based index.
    pub fn value(&self, index: usize) -> Option<String> {
        if index >= self.lower && index <= self.upper_bound() {
            Some(self.items[index - self.lower].clone())
        } else {
            None
        }
    }

    /// Set value at 1-based index.
    pub fn set_value(&mut self, index: usize, val: String) -> bool {
        if index >= self.lower && index <= self.upper_bound() {
            self.items[index - self.lower] = val;
            true
        } else {
            false
        }
    }

    /// Clear the array.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for TDataStdLabelArray1 {
    fn default() -> Self {
        Self::new_1indexed(0)
    }
}

impl fmt::Debug for TDataStdLabelArray1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDataStdLabelArray1")
            .field("lower", &self.lower)
            .field("upper", &self.upper_bound())
            .field("size", &self.items.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_1indexed() {
        let arr = TDataStdLabelArray1::new_1indexed(4);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 4);
        assert_eq!(arr.size(), 4);
    }

    #[test]
    fn test_new_with_bounds() {
        let arr = TDataStdLabelArray1::new(2, 6);
        assert_eq!(arr.lower_bound(), 2);
        assert_eq!(arr.upper_bound(), 6);
        assert_eq!(arr.size(), 5);
    }

    #[test]
    fn test_get_set_values() {
        let mut arr = TDataStdLabelArray1::new_1indexed(3);
        assert!(arr.set_value(1, "l1".to_string()));
        assert!(arr.set_value(2, "l2".to_string()));
        assert!(arr.set_value(3, "l3".to_string()));

        assert_eq!(arr.value(1), Some("l1".to_string()));
        assert_eq!(arr.value(2), Some("l2".to_string()));
        assert_eq!(arr.value(3), Some("l3".to_string()));
    }

    #[test]
    fn test_1based_indexing() {
        let mut arr = TDataStdLabelArray1::new(1, 3);
        arr.set_value(1, "a".to_string());
        arr.set_value(3, "c".to_string());

        assert_eq!(arr.value(1), Some("a".to_string()));
        assert_eq!(arr.value(3), Some("c".to_string()));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut arr = TDataStdLabelArray1::new_1indexed(3);
        assert!(!arr.set_value(0, "x".to_string()));
        assert!(!arr.set_value(4, "x".to_string()));
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }

    #[test]
    fn test_empty_array() {
        let arr = TDataStdLabelArray1::new(10, 9);  // upper < lower
        assert!(arr.is_empty());
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_clear() {
        let mut arr = TDataStdLabelArray1::new_1indexed(2);
        arr.set_value(1, "label".to_string());
        assert_eq!(arr.size(), 2);

        arr.clear();
        assert_eq!(arr.size(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_debug() {
        let arr = TDataStdLabelArray1::new_1indexed(5);
        let debug_str = format!("{:?}", arr);
        assert!(debug_str.contains("lower"));
        assert!(debug_str.contains("upper"));
    }
}
