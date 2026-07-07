// FILE: step_geom_array1_of_boundary_curve.rs
// occt: StepGeom_Array1OfBoundaryCurve

use std::vec::Vec;

/// Deprecated typedef alias for NCollection_Array1<StepGeom_BoundaryCurve>.
/// Provides 1-based indexing semantics with fixed bounds (Lower/Upper).
pub struct StepGeomArray1OfBoundaryCurve {
    data: Vec<Option<String>>, // Using String as placeholder for StepGeom_BoundaryCurve handle
    lower: usize,
}

impl StepGeomArray1OfBoundaryCurve {
    /// Create an array with specified lower and upper bounds (1-based).
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            return Self {
                data: Vec::new(),
                lower,
            };
        }
        let size = upper - lower + 1;
        Self {
            data: vec![None; size],
            lower,
        }
    }

    /// Get the lower bound index.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get the upper bound index.
    pub fn upper(&self) -> usize {
        if self.data.is_empty() {
            self.lower - 1
        } else {
            self.lower + self.data.len() - 1
        }
    }

    /// Get the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get an element by 1-based index. Returns None if index is out of bounds.
    pub fn value(&self, index: usize) -> Option<&Option<String>> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let zero_based_index = index - self.lower;
        self.data.get(zero_based_index)
    }

    /// Set an element by 1-based index. Returns false if index is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Option<String>) -> bool {
        if index < self.lower || index > self.upper() {
            return false;
        }
        let zero_based_index = index - self.lower;
        if let Some(elem) = self.data.get_mut(zero_based_index) {
            *elem = value;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array_with_bounds() {
        let arr = StepGeomArray1OfBoundaryCurve::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_empty_array_invalid_bounds() {
        let arr = StepGeomArray1OfBoundaryCurve::new(5, 1);
        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_one_based_indexing() {
        let mut arr = StepGeomArray1OfBoundaryCurve::new(1, 3);

        arr.set_value(1, Some("first".to_string()));
        arr.set_value(2, Some("second".to_string()));
        arr.set_value(3, Some("third".to_string()));

        assert_eq!(arr.value(1), Some(&Some("first".to_string())));
        assert_eq!(arr.value(2), Some(&Some("second".to_string())));
        assert_eq!(arr.value(3), Some(&Some("third".to_string())));
    }

    #[test]
    fn test_custom_lower_bound() {
        let mut arr = StepGeomArray1OfBoundaryCurve::new(10, 12);
        assert_eq!(arr.lower(), 10);
        assert_eq!(arr.upper(), 12);
        assert_eq!(arr.len(), 3);

        arr.set_value(10, Some("a".to_string()));
        arr.set_value(11, Some("b".to_string()));
        arr.set_value(12, Some("c".to_string()));

        assert_eq!(arr.value(10), Some(&Some("a".to_string())));
        assert_eq!(arr.value(11), Some(&Some("b".to_string())));
        assert_eq!(arr.value(12), Some(&Some("c".to_string())));

        // Out of bounds below lower
        assert_eq!(arr.value(9), None);
        // Out of bounds above upper
        assert_eq!(arr.value(13), None);
    }

    #[test]
    fn test_set_value_out_of_bounds() {
        let mut arr = StepGeomArray1OfBoundaryCurve::new(1, 3);

        let success = arr.set_value(1, Some("valid".to_string()));
        assert!(success);

        let out_of_bounds_low = arr.set_value(0, Some("invalid".to_string()));
        assert!(!out_of_bounds_low);

        let out_of_bounds_high = arr.set_value(4, Some("invalid".to_string()));
        assert!(!out_of_bounds_high);
    }

    #[test]
    fn test_initial_values_are_none() {
        let arr = StepGeomArray1OfBoundaryCurve::new(1, 3);

        assert_eq!(arr.value(1), Some(&None));
        assert_eq!(arr.value(2), Some(&None));
        assert_eq!(arr.value(3), Some(&None));
    }

    #[test]
    fn test_large_array() {
        let mut arr = StepGeomArray1OfBoundaryCurve::new(1, 100);
        assert_eq!(arr.len(), 100);

        arr.set_value(1, Some("start".to_string()));
        arr.set_value(50, Some("middle".to_string()));
        arr.set_value(100, Some("end".to_string()));

        assert_eq!(arr.value(1), Some(&Some("start".to_string())));
        assert_eq!(arr.value(50), Some(&Some("middle".to_string())));
        assert_eq!(arr.value(100), Some(&Some("end".to_string())));
    }
}
