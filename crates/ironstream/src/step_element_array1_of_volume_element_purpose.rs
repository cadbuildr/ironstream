// FILE: step_element_array1_of_volume_element_purpose.rs
// occt: StepElement_Array1OfVolumeElementPurpose

/// Rust port of OCCT's deprecated StepElement_Array1OfVolumeElementPurpose.
/// A 1-based fixed-length array wrapping a Vec, mimicking NCollection_Array1 semantics.
#[derive(Clone, Debug)]
pub struct StepElementArray1OfVolumeElementPurpose {
    data: Vec<i32>,
    lower: i32,
}

impl StepElementArray1OfVolumeElementPurpose {
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
        let arr = StepElementArray1OfVolumeElementPurpose::new(1, 5);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 5);
    }

    #[test]
    fn test_array1_set_get() {
        let mut arr = StepElementArray1OfVolumeElementPurpose::new(1, 3);
        arr.set_value(1, 11);
        arr.set_value(2, 22);
        arr.set_value(3, 33);

        assert_eq!(arr.value(1), Some(&11));
        assert_eq!(arr.value(2), Some(&22));
        assert_eq!(arr.value(3), Some(&33));
    }

    #[test]
    fn test_array1_from_vec() {
        let values = vec![100, 200, 300];
        let arr = StepElementArray1OfVolumeElementPurpose::from_vec(1, values);

        assert_eq!(arr.length(), 3);
        assert_eq!(arr.value(2), Some(&200));
    }
}
