// FILE: t_data_xtd_array1_of_trsf.rs
// occt: TDataXtd_Array1OfTrsf

//! Deprecated typedef for TDataXtd_Array1OfTrsf.
//!
//! In OCCT, this was a 1-indexed array of gp_Trsf (transformation) items.
//! We implement a minimal array structure using Vec with 1-based indexing semantics.

use std::fmt;

/// Placeholder for gp_Trsf (transformation matrix).
#[derive(Clone, Debug)]
pub struct GpTrsf {
    // Simplified transformation placeholder
    // In full port, this would be a proper 4x4 transformation matrix
    data: [f64; 12],  // 3x4 matrix for affine transform
}

impl GpTrsf {
    /// Create an identity transformation.
    pub fn new() -> Self {
        GpTrsf {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
            ],
        }
    }

    /// Create a transformation from an array.
    pub fn from_array(data: [f64; 12]) -> Self {
        GpTrsf { data }
    }
}

impl Default for GpTrsf {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for GpTrsf {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter().zip(other.data.iter()).all(|(a, b)| (a - b).abs() < 1e-10)
    }
}

/// TDataXtd_Array1OfTrsf: A 1-indexed array of gp_Trsf items (deprecated typedef).
/// Wraps a Vec and provides 1-based indexing semantics matching OCCT Array1<gp_Trsf> behavior.
#[derive(Clone)]
pub struct TDataXtdArray1OfTrsf {
    items: Vec<GpTrsf>,
    lower: usize,
}

impl TDataXtdArray1OfTrsf {
    /// Create a new array with bounds [lower, upper].
    /// In OCCT, arrays are 1-indexed by default unless otherwise specified.
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        TDataXtdArray1OfTrsf {
            items: (0..size).map(|_| GpTrsf::new()).collect(),
            lower,
        }
    }

    /// Create a standard 1-indexed array of size n.
    pub fn new_1indexed(n: usize) -> Self {
        TDataXtdArray1OfTrsf {
            items: (0..n).map(|_| GpTrsf::new()).collect(),
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
    pub fn value(&self, index: usize) -> Option<GpTrsf> {
        if index >= self.lower && index <= self.upper_bound() {
            Some(self.items[index - self.lower].clone())
        } else {
            None
        }
    }

    /// Set value at 1-based index.
    pub fn set_value(&mut self, index: usize, val: GpTrsf) -> bool {
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

impl Default for TDataXtdArray1OfTrsf {
    fn default() -> Self {
        Self::new_1indexed(0)
    }
}

impl fmt::Debug for TDataXtdArray1OfTrsf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TDataXtdArray1OfTrsf")
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
        let arr = TDataXtdArray1OfTrsf::new_1indexed(3);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 3);
        assert_eq!(arr.size(), 3);
    }

    #[test]
    fn test_new_with_bounds() {
        let arr = TDataXtdArray1OfTrsf::new(2, 5);
        assert_eq!(arr.lower_bound(), 2);
        assert_eq!(arr.upper_bound(), 5);
        assert_eq!(arr.size(), 4);
    }

    #[test]
    fn test_get_set_values() {
        let mut arr = TDataXtdArray1OfTrsf::new_1indexed(3);
        let trsf1 = GpTrsf::new();
        let trsf2 = GpTrsf::from_array([2.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 2.0, 0.0, 0.0, 2.0, 3.0]);

        assert!(arr.set_value(1, trsf1.clone()));
        assert!(arr.set_value(2, trsf2.clone()));

        assert_eq!(arr.value(1), Some(trsf1));
        assert_eq!(arr.value(2), Some(trsf2));
    }

    #[test]
    fn test_1based_indexing() {
        let mut arr = TDataXtdArray1OfTrsf::new(1, 3);
        let identity = GpTrsf::new();
        arr.set_value(1, identity.clone());
        arr.set_value(3, identity.clone());

        assert_eq!(arr.value(1), Some(identity.clone()));
        assert_eq!(arr.value(3), Some(identity));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut arr = TDataXtdArray1OfTrsf::new_1indexed(3);
        let trsf = GpTrsf::new();
        assert!(!arr.set_value(0, trsf.clone()));
        assert!(!arr.set_value(4, trsf.clone()));
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }

    #[test]
    fn test_empty_array() {
        let arr = TDataXtdArray1OfTrsf::new(5, 4);  // upper < lower
        assert!(arr.is_empty());
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_clear() {
        let mut arr = TDataXtdArray1OfTrsf::new_1indexed(2);
        assert_eq!(arr.size(), 2);

        arr.clear();
        assert_eq!(arr.size(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_identity_trsf() {
        let identity = GpTrsf::new();
        let expected = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        assert_eq!(identity, GpTrsf::from_array(expected));
    }

    #[test]
    fn test_debug() {
        let arr = TDataXtdArray1OfTrsf::new_1indexed(5);
        let debug_str = format!("{:?}", arr);
        assert!(debug_str.contains("lower"));
        assert!(debug_str.contains("upper"));
    }
}
