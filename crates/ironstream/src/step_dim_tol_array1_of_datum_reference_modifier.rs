// FILE: step_dim_tol_array1_of_datum_reference_modifier.rs
// occt: StepDimTol_Array1OfDatumReferenceModifier

/// Rust port of OCCT's deprecated StepDimTol_Array1OfDatumReferenceModifier.
/// A 1-based fixed-length array wrapping a Vec, mimicking NCollection_Array1 semantics.
#[derive(Clone, Debug)]
pub struct StepDimTolArray1OfDatumReferenceModifier {
    data: Vec<i32>,
    lower: i32,
}

impl StepDimTolArray1OfDatumReferenceModifier {
    /// Create a new Array1 with the given lower bound and size.
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
    fn test_array1_creation() {
        let arr = StepDimTolArray1OfDatumReferenceModifier::new(1, 4);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 4);
        assert_eq!(arr.length(), 4);
    }

    #[test]
    fn test_array1_set_and_get() {
        let mut arr = StepDimTolArray1OfDatumReferenceModifier::new(1, 3);
        arr.set_value(1, 10);
        arr.set_value(2, 20);
        arr.set_value(3, 30);

        assert_eq!(arr.value(1), Some(&10));
        assert_eq!(arr.value(2), Some(&20));
        assert_eq!(arr.value(3), Some(&30));
    }

    #[test]
    fn test_array1_out_of_bounds() {
        let arr = StepDimTolArray1OfDatumReferenceModifier::new(1, 2);
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(3), None);
    }

    #[test]
    fn test_array1_custom_lower() {
        let mut arr = StepDimTolArray1OfDatumReferenceModifier::new(10, 2);
        arr.set_value(10, 100);
        arr.set_value(11, 200);

        assert_eq!(arr.value(10), Some(&100));
        assert_eq!(arr.value(11), Some(&200));
        assert_eq!(arr.value(9), None);
        assert_eq!(arr.value(12), None);
    }
}
