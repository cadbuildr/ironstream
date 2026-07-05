// FILE: int_ana_list_of_curve.rs
// occt: IntAna_ListOfCurve

use std::vec::Vec;

/// Deprecated alias for a list of curves from analytical intersection.
#[derive(Clone, Debug)]
pub struct IntAna_ListOfCurve {
    curves: Vec<i32>,
}

impl IntAna_ListOfCurve {
    /// Create a new list of curves.
    pub fn new() -> Self {
        IntAna_ListOfCurve {
            curves: Vec::new(),
        }
    }

    /// Add a curve to the list.
    pub fn append(&mut self, curve_id: i32) {
        self.curves.push(curve_id);
    }

    /// Get the number of curves.
    pub fn length(&self) -> usize {
        self.curves.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.curves.is_empty()
    }

    /// Get a curve by index.
    pub fn curve(&self, index: usize) -> Option<i32> {
        self.curves.get(index).copied()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.curves.clear();
    }
}

impl Default for IntAna_ListOfCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = IntAna_ListOfCurve::new();
        assert!(list.is_empty());
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = IntAna_ListOfCurve::new();
        list.append(1);
        list.append(2);
        assert_eq!(list.length(), 2);
        assert_eq!(list.curve(0), Some(1));
        assert_eq!(list.curve(1), Some(2));
    }

    #[test]
    fn test_clear() {
        let mut list = IntAna_ListOfCurve::new();
        list.append(1);
        list.append(2);
        list.clear();
        assert!(list.is_empty());
    }
}
