// FILE: step_dim_tol_h_array1_of_geometric_tolerance_modifier.rs
// occt: StepDimTol_HArray1OfGeometricToleranceModifier

/// Rust port of OCCT's deprecated StepDimTol_HArray1OfGeometricToleranceModifier.
/// HArray1 is a heap-allocated wrapper. A 1-based fixed-length array wrapping a Vec.
#[derive(Clone, Debug)]
pub struct StepDimTolHArray1OfGeometricToleranceModifier {
    data: Vec<i32>,
    lower: i32,
}

impl StepDimTolHArray1OfGeometricToleranceModifier {
    /// Create a new HArray1 with the given lower bound and size.
    pub fn new(lower: i32, size: usize) -> Self {
        Self {
            data: vec![0; size],
            lower,
        }
    }

    /// Create from a lower bound and a vec of values.
    pub fn from_vec(lower: i32, values: Vec<i32>) -> Self {
        Self {
            data: values,
            lower,
        }
    }

    /// Get the lower bound (1-based indexing).
    pub fn lower_bound(&self) -> i32 {
        self.lower
    }

    /// Get the upper bound (inclusive).
    pub fn upper_bound(&self) -> i32 {
        self.lower + self.data.len() as i32 - 1
    }

    /// Get the length.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Get a reference to the value at the given 1-based index.
    pub fn value(&self, idx: i32) -> Option<&i32> {
        if idx < self.lower || idx > self.upper_bound() {
            return None;
        }
        self.data.get((idx - self.lower) as usize)
    }

    /// Get a mutable reference to the value at the given 1-based index.
    pub fn value_mut(&mut self, idx: i32) -> Option<&mut i32> {
        if idx < self.lower || idx > self.upper_bound() {
            return None;
        }
        self.data.get_mut((idx - self.lower) as usize)
    }

    /// Set the value at the given 1-based index.
    pub fn set_value(&mut self, idx: i32, val: i32) -> bool {
        if let Some(r) = self.value_mut(idx) {
            *r = val;
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
    fn test_harray1_creation() {
        let arr = StepDimTolHArray1OfGeometricToleranceModifier::new(1, 6);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 6);
    }

    #[test]
    fn test_harray1_set_get() {
        let mut arr = StepDimTolHArray1OfGeometricToleranceModifier::new(1, 2);
        arr.set_value(1, 50);
        arr.set_value(2, 100);

        assert_eq!(arr.value(1), Some(&50));
        assert_eq!(arr.value(2), Some(&100));
    }

    #[test]
    fn test_harray1_from_vec() {
        let values = vec![1, 2, 3, 4];
        let arr = StepDimTolHArray1OfGeometricToleranceModifier::from_vec(1, values);

        assert_eq!(arr.length(), 4);
        assert_eq!(arr.value(1), Some(&1));
        assert_eq!(arr.value(4), Some(&4));
    }
}
