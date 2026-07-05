// FILE: step_fea_array1_of_element_representation.rs
// occt: StepFEA_Array1OfElementRepresentation

/// Rust port of OCCT's deprecated StepFEA_Array1OfElementRepresentation.
/// A 1-based fixed-length array wrapping a Vec, mimicking NCollection_Array1 semantics.
#[derive(Clone, Debug)]
pub struct StepFEAArray1OfElementRepresentation {
    data: Vec<String>,
    lower: i32,
}

impl StepFEAArray1OfElementRepresentation {
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
    fn test_array1_creation() {
        let arr = StepFEAArray1OfElementRepresentation::new(1, 5);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 5);
    }

    #[test]
    fn test_array1_access() {
        let mut arr = StepFEAArray1OfElementRepresentation::new(1, 2);
        arr.set_value(1, "elem_repr1".to_string());
        arr.set_value(2, "elem_repr2".to_string());

        assert_eq!(arr.value(1), Some(&"elem_repr1".to_string()));
        assert_eq!(arr.value(2), Some(&"elem_repr2".to_string()));
    }

    #[test]
    fn test_array1_from_vec() {
        let values = vec!["er1".to_string(), "er2".to_string()];
        let arr = StepFEAArray1OfElementRepresentation::from_vec(1, values);

        assert_eq!(arr.length(), 2);
        assert_eq!(arr.value(1), Some(&"er1".to_string()));
    }
}
