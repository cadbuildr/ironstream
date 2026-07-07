// FILE: extrema_array1_of_p_on_curv.rs
// occt: Extrema_Array1OfPOnCurv

use std::rc::Rc;
use std::cell::RefCell;

/// Point on curve element (placeholder).
#[derive(Clone, Debug)]
pub struct POnCurv {
    // Placeholder for point on curve data
}

/// Deprecated: 1D array of POnCurv.
/// Use Vec<POnCurv> or NCollection_Array1<POnCurv> directly instead.
#[derive(Clone, Debug)]
pub struct Array1OfPOnCurv {
    items: Vec<POnCurv>,
    lower: usize,
}

impl Array1OfPOnCurv {
    /// Create an array from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Array1OfPOnCurv {
            items: vec![POnCurv {}; size],
            lower,
        }
    }

    /// Get the element at index i (1-indexed in OCCT).
    pub fn at(&self, i: usize) -> Option<&POnCurv> {
        if i >= self.lower && i < self.lower + self.items.len() {
            Some(&self.items[i - self.lower])
        } else {
            None
        }
    }

    /// Set the element at index i (1-indexed in OCCT).
    pub fn set(&mut self, i: usize, val: POnCurv) -> bool {
        if i >= self.lower && i < self.lower + self.items.len() {
            self.items[i - self.lower] = val;
            true
        } else {
            false
        }
    }

    /// Get the lower bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get the upper bound.
    pub fn upper(&self) -> usize {
        self.lower + self.items.len() - 1
    }

    /// Get the length.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = Array1OfPOnCurv::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_array_access() {
        let arr = Array1OfPOnCurv::new(1, 3);
        assert!(arr.at(1).is_some());
        assert!(arr.at(3).is_some());
        assert!(arr.at(4).is_none());
    }

    #[test]
    fn test_array_set() {
        let mut arr = Array1OfPOnCurv::new(1, 3);
        let val = POnCurv {};
        assert!(arr.set(1, val));
        assert!(!arr.set(5, POnCurv {}));
    }

    #[test]
    fn test_array_empty() {
        let arr = Array1OfPOnCurv::new(1, 0);
        assert!(arr.is_empty());
    }
}
