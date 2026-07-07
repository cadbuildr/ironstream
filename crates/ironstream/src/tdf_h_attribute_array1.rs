// FILE: tdf_h_attribute_array1.rs
// occt: TDF_HAttributeArray1

//! Deprecated typedef for TDF_HAttributeArray1.
//!
//! In OCCT, this was a handle-based 1-indexed array of TDF_Attribute handles.
//! We implement a minimal array structure using Vec with 1-based indexing semantics.

use std::fmt;

/// TDF_HAttributeArray1: A 1-indexed array of TDF_Attribute handles (deprecated typedef).
/// Wraps a Vec and provides 1-based indexing semantics matching OCCT Array1<Handle<T>> behavior.
#[derive(Clone)]
pub struct TdfHAttributeArray1 {
    items: Vec<i32>,  // Placeholder: would be Vec<Handle<TdfAttribute>> in full port
    lower: usize,
}

impl TdfHAttributeArray1 {
    /// Create a new array with bounds [lower, upper].
    /// In OCCT, arrays are 1-indexed by default unless otherwise specified.
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        TdfHAttributeArray1 {
            items: vec![0; size],
            lower,
        }
    }

    /// Create a standard 1-indexed array of size n.
    pub fn new_1indexed(n: usize) -> Self {
        TdfHAttributeArray1 {
            items: vec![0; n],
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
    pub fn value(&self, index: usize) -> Option<i32> {
        if index >= self.lower && index <= self.upper_bound() {
            Some(self.items[index - self.lower])
        } else {
            None
        }
    }

    /// Set value at 1-based index.
    pub fn set_value(&mut self, index: usize, val: i32) -> bool {
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

impl Default for TdfHAttributeArray1 {
    fn default() -> Self {
        Self::new_1indexed(0)
    }
}

impl fmt::Debug for TdfHAttributeArray1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TdfHAttributeArray1")
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
        let arr = TdfHAttributeArray1::new_1indexed(5);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 5);
        assert_eq!(arr.size(), 5);
    }

    #[test]
    fn test_new_with_bounds() {
        let arr = TdfHAttributeArray1::new(2, 6);
        assert_eq!(arr.lower_bound(), 2);
        assert_eq!(arr.upper_bound(), 6);
        assert_eq!(arr.size(), 5);
    }

    #[test]
    fn test_get_set_values() {
        let mut arr = TdfHAttributeArray1::new_1indexed(3);
        assert!(arr.set_value(1, 10));
        assert!(arr.set_value(2, 20));
        assert!(arr.set_value(3, 30));

        assert_eq!(arr.value(1), Some(10));
        assert_eq!(arr.value(2), Some(20));
        assert_eq!(arr.value(3), Some(30));
    }

    #[test]
    fn test_1based_indexing() {
        let mut arr = TdfHAttributeArray1::new(1, 4);
        arr.set_value(1, 100);
        arr.set_value(4, 400);

        assert_eq!(arr.value(1), Some(100));
        assert_eq!(arr.value(4), Some(400));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut arr = TdfHAttributeArray1::new_1indexed(3);
        assert!(!arr.set_value(0, 10));
        assert!(!arr.set_value(4, 10));
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }

    #[test]
    fn test_empty_array() {
        let arr = TdfHAttributeArray1::new(5, 4);  // upper < lower
        assert!(arr.is_empty());
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_clear() {
        let mut arr = TdfHAttributeArray1::new_1indexed(3);
        arr.set_value(1, 10);
        assert_eq!(arr.size(), 3);

        arr.clear();
        assert_eq!(arr.size(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_debug() {
        let arr = TdfHAttributeArray1::new_1indexed(5);
        let debug_str = format!("{:?}", arr);
        assert!(debug_str.contains("lower"));
        assert!(debug_str.contains("upper"));
    }
}
