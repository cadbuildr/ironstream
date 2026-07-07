// FILE: t_function_array1_of_data_map_of_guid_driver.rs
// occt: TFunction_Array1OfDataMapOfGUIDDriver

//! Deprecated typedef for TFunction_Array1OfDataMapOfGUIDDriver.
//!
//! In OCCT, this was a 1-indexed array of TFunction_DataMapOfGUIDDriver items.
//! We implement a minimal array structure using Vec with 1-based indexing semantics.

use std::collections::HashMap;
use std::fmt;

/// Placeholder for TFunction_DataMapOfGUIDDriver (GUID -> Driver map).
#[derive(Clone, PartialEq)]
pub struct TFunctionDataMapOfGuidDriver {
    data: HashMap<String, i32>,  // guid -> driver_id placeholder
}

impl TFunctionDataMapOfGuidDriver {
    pub fn new() -> Self {
        TFunctionDataMapOfGuidDriver {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: String, val: i32) {
        self.data.insert(key, val);
    }

    pub fn find(&self, key: &str) -> Option<i32> {
        self.data.get(key).copied()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for TFunctionDataMapOfGuidDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TFunctionDataMapOfGuidDriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TFunctionDataMapOfGuidDriver")
            .field("size", &self.data.len())
            .finish()
    }
}

/// TFunction_Array1OfDataMapOfGUIDDriver: A 1-indexed array of DataMapOfGUIDDriver items (deprecated typedef).
/// Wraps a Vec and provides 1-based indexing semantics.
#[derive(Clone)]
pub struct TFunctionArray1OfDataMapOfGuidDriver {
    items: Vec<TFunctionDataMapOfGuidDriver>,
    lower: usize,
}

impl TFunctionArray1OfDataMapOfGuidDriver {
    /// Create a new array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        TFunctionArray1OfDataMapOfGuidDriver {
            items: (0..size).map(|_| TFunctionDataMapOfGuidDriver::new()).collect(),
            lower,
        }
    }

    /// Create a standard 1-indexed array of size n.
    pub fn new_1indexed(n: usize) -> Self {
        TFunctionArray1OfDataMapOfGuidDriver {
            items: (0..n).map(|_| TFunctionDataMapOfGuidDriver::new()).collect(),
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
    pub fn value(&self, index: usize) -> Option<TFunctionDataMapOfGuidDriver> {
        if index >= self.lower && index <= self.upper_bound() {
            Some(self.items[index - self.lower].clone())
        } else {
            None
        }
    }

    /// Set value at 1-based index.
    pub fn set_value(&mut self, index: usize, val: TFunctionDataMapOfGuidDriver) -> bool {
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

impl Default for TFunctionArray1OfDataMapOfGuidDriver {
    fn default() -> Self {
        Self::new_1indexed(0)
    }
}

impl fmt::Debug for TFunctionArray1OfDataMapOfGuidDriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TFunctionArray1OfDataMapOfGuidDriver")
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
        let arr = TFunctionArray1OfDataMapOfGuidDriver::new_1indexed(3);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 3);
        assert_eq!(arr.size(), 3);
    }

    #[test]
    fn test_new_with_bounds() {
        let arr = TFunctionArray1OfDataMapOfGuidDriver::new(2, 5);
        assert_eq!(arr.lower_bound(), 2);
        assert_eq!(arr.upper_bound(), 5);
        assert_eq!(arr.size(), 4);
    }

    #[test]
    fn test_get_set_values() {
        let mut arr = TFunctionArray1OfDataMapOfGuidDriver::new_1indexed(2);
        let mut map1 = TFunctionDataMapOfGuidDriver::new();
        map1.bind("g1".to_string(), 10);

        let mut map2 = TFunctionDataMapOfGuidDriver::new();
        map2.bind("g2".to_string(), 20);

        assert!(arr.set_value(1, map1.clone()));
        assert!(arr.set_value(2, map2.clone()));

        assert_eq!(arr.value(1).unwrap().find("g1"), Some(10));
        assert_eq!(arr.value(2).unwrap().find("g2"), Some(20));
    }

    #[test]
    fn test_1based_indexing() {
        let mut arr = TFunctionArray1OfDataMapOfGuidDriver::new(1, 2);
        let map = TFunctionDataMapOfGuidDriver::new();
        arr.set_value(1, map.clone());
        arr.set_value(2, map);

        assert_eq!(arr.value(1).unwrap().size(), 0);
        assert_eq!(arr.value(2).unwrap().size(), 0);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut arr = TFunctionArray1OfDataMapOfGuidDriver::new_1indexed(2);
        let map = TFunctionDataMapOfGuidDriver::new();
        assert!(!arr.set_value(0, map.clone()));
        assert!(!arr.set_value(3, map.clone()));
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(3), None);
    }

    #[test]
    fn test_empty_array() {
        let arr = TFunctionArray1OfDataMapOfGuidDriver::new(5, 4);
        assert!(arr.is_empty());
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_clear() {
        let mut arr = TFunctionArray1OfDataMapOfGuidDriver::new_1indexed(2);
        assert_eq!(arr.size(), 2);

        arr.clear();
        assert_eq!(arr.size(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_data_map_operations() {
        let mut map = TFunctionDataMapOfGuidDriver::new();
        map.bind("guid1".to_string(), 100);
        map.bind("guid2".to_string(), 200);

        assert_eq!(map.find("guid1"), Some(100));
        assert_eq!(map.find("guid2"), Some(200));
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_debug() {
        let arr = TFunctionArray1OfDataMapOfGuidDriver::new_1indexed(5);
        let debug_str = format!("{:?}", arr);
        assert!(debug_str.contains("lower"));
        assert!(debug_str.contains("upper"));
    }
}
