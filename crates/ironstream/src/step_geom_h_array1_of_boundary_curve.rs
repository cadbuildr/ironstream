// FILE: step_geom_h_array1_of_boundary_curve.rs
// occt: StepGeom_HArray1OfBoundaryCurve

use std::vec::Vec;

/// Deprecated typedef alias for NCollection_Array1<StepGeom_BoundaryCurve> wrapped in a handle.
/// HArray1 provides reference-counted array semantics.
pub struct StepGeomHArray1OfBoundaryCurve {
    data: Vec<Option<String>>,
    lower: usize,
}

impl StepGeomHArray1OfBoundaryCurve {
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

    /// Get an element by 1-based index.
    pub fn value(&self, index: usize) -> Option<&Option<String>> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let zero_based_index = index - self.lower;
        self.data.get(zero_based_index)
    }

    /// Set an element by 1-based index.
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
    fn test_harray1_new() {
        let arr = StepGeomHArray1OfBoundaryCurve::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_harray1_indexing() {
        let mut arr = StepGeomHArray1OfBoundaryCurve::new(1, 3);
        arr.set_value(1, Some("bc1".to_string()));
        arr.set_value(2, Some("bc2".to_string()));

        assert_eq!(arr.value(1), Some(&Some("bc1".to_string())));
        assert_eq!(arr.value(2), Some(&Some("bc2".to_string())));
        assert_eq!(arr.value(3), Some(&None));
    }

    #[test]
    fn test_harray1_bounds() {
        let arr = StepGeomHArray1OfBoundaryCurve::new(1, 3);
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }
}
