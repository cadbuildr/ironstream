// FILE: step_dim_tol_array1_of_datum_reference_element.rs
// occt: StepDimTol_Array1OfDatumReferenceElement

/// Rust port of OCCT's deprecated StepDimTol_Array1OfDatumReferenceElement.
/// A 1-based fixed-length array wrapping a Vec, mimicking NCollection_Array1 semantics.
#[derive(Clone, Debug)]
pub struct StepDimTolArray1OfDatumReferenceElement {
    data: Vec<String>,
    lower: i32,
}

impl StepDimTolArray1OfDatumReferenceElement {
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
    fn test_array1_bounds() {
        let arr = StepDimTolArray1OfDatumReferenceElement::new(1, 3);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 3);
    }

    #[test]
    fn test_array1_indexing() {
        let mut arr = StepDimTolArray1OfDatumReferenceElement::new(1, 2);
        arr.set_value(1, "elem1".to_string());
        arr.set_value(2, "elem2".to_string());

        assert_eq!(arr.value(1), Some(&"elem1".to_string()));
        assert_eq!(arr.value(2), Some(&"elem2".to_string()));
        assert!(arr.value(3).is_none());
    }

    #[test]
    fn test_array1_from_vec() {
        let values = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let arr = StepDimTolArray1OfDatumReferenceElement::from_vec(2, values);

        assert_eq!(arr.lower_bound(), 2);
        assert_eq!(arr.upper_bound(), 4);
        assert_eq!(arr.length(), 3);
        assert_eq!(arr.value(2), Some(&"x".to_string()));
        assert_eq!(arr.value(3), Some(&"y".to_string()));
        assert_eq!(arr.value(4), Some(&"z".to_string()));
    }
}
