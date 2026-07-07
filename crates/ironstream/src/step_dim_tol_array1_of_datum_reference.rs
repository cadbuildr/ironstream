// FILE: step_dim_tol_array1_of_datum_reference.rs
// occt: StepDimTol_Array1OfDatumReference

/// Rust port of OCCT's deprecated StepDimTol_Array1OfDatumReference.
/// A 1-based fixed-length array wrapping a Vec, mimicking NCollection_Array1 semantics.
///
/// OCCT deprecated this in favor of NCollection_Array1<opencascade::handle<StepDimTol_DatumReference>>,
/// but we provide it as a convenience newtype with 1-based indexing.
#[derive(Clone, Debug)]
pub struct StepDimTolArray1OfDatumReference {
    data: Vec<String>,
    lower: i32,
}

impl StepDimTolArray1OfDatumReference {
    /// Create a new Array1 with the given lower bound and size.
    pub fn new(lower: i32, size: usize) -> Self {
        Self {
            data: vec![String::new(); size],
            lower,
        }
    }

    /// Create from a lower bound and a vec of values.
    pub fn from_vec(lower: i32, values: Vec<String>) -> Self {
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
    pub fn value(&self, idx: i32) -> Option<&String> {
        if idx < self.lower || idx > self.upper_bound() {
            return None;
        }
        self.data.get((idx - self.lower) as usize)
    }

    /// Get a mutable reference to the value at the given 1-based index.
    pub fn value_mut(&mut self, idx: i32) -> Option<&mut String> {
        if idx < self.lower || idx > self.upper_bound() {
            return None;
        }
        self.data.get_mut((idx - self.lower) as usize)
    }

    /// Set the value at the given 1-based index.
    pub fn set_value(&mut self, idx: i32, val: String) -> bool {
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
    fn test_array1_creation_and_bounds() {
        let arr = StepDimTolArray1OfDatumReference::new(1, 5);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_array1_access_1based() {
        let mut arr = StepDimTolArray1OfDatumReference::new(1, 3);
        arr.set_value(1, "a".to_string());
        arr.set_value(2, "b".to_string());
        arr.set_value(3, "c".to_string());

        assert_eq!(arr.value(1), Some(&"a".to_string()));
        assert_eq!(arr.value(2), Some(&"b".to_string()));
        assert_eq!(arr.value(3), Some(&"c".to_string()));
    }

    #[test]
    fn test_array1_out_of_bounds() {
        let arr = StepDimTolArray1OfDatumReference::new(1, 3);
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }

    #[test]
    fn test_array1_custom_lower_bound() {
        let mut arr = StepDimTolArray1OfDatumReference::new(5, 3);
        assert_eq!(arr.lower_bound(), 5);
        assert_eq!(arr.upper_bound(), 7);

        arr.set_value(5, "first".to_string());
        arr.set_value(6, "second".to_string());
        arr.set_value(7, "third".to_string());

        assert_eq!(arr.value(5), Some(&"first".to_string()));
        assert_eq!(arr.value(6), Some(&"second".to_string()));
        assert_eq!(arr.value(7), Some(&"third".to_string()));
        assert_eq!(arr.value(4), None);
        assert_eq!(arr.value(8), None);
    }

    #[test]
    fn test_array1_from_vec() {
        let values = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let arr = StepDimTolArray1OfDatumReference::from_vec(1, values);

        assert_eq!(arr.length(), 3);
        assert_eq!(arr.value(1), Some(&"x".to_string()));
        assert_eq!(arr.value(2), Some(&"y".to_string()));
        assert_eq!(arr.value(3), Some(&"z".to_string()));
    }
}
