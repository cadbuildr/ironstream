// FILE: step_dim_tol_h_array1_of_datum_reference_compartment.rs
// occt: StepDimTol_HArray1OfDatumReferenceCompartment

/// Rust port of OCCT's deprecated StepDimTol_HArray1OfDatumReferenceCompartment.
/// HArray1 is a heap-allocated wrapper. A 1-based fixed-length array wrapping a Vec.
#[derive(Clone, Debug)]
pub struct StepDimTolHArray1OfDatumReferenceCompartment {
    data: Vec<String>,
    lower: i32,
}

impl StepDimTolHArray1OfDatumReferenceCompartment {
    /// Create a new HArray1 with the given lower bound and size.
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
    fn test_harray1_creation() {
        let arr = StepDimTolHArray1OfDatumReferenceCompartment::new(1, 4);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 4);
    }

    #[test]
    fn test_harray1_access() {
        let mut arr = StepDimTolHArray1OfDatumReferenceCompartment::new(1, 2);
        arr.set_value(1, "c1".to_string());
        arr.set_value(2, "c2".to_string());

        assert_eq!(arr.value(1), Some(&"c1".to_string()));
        assert_eq!(arr.value(2), Some(&"c2".to_string()));
    }

    #[test]
    fn test_harray1_from_vec() {
        let values = vec!["x".to_string(), "y".to_string()];
        let arr = StepDimTolHArray1OfDatumReferenceCompartment::from_vec(1, values);

        assert_eq!(arr.length(), 2);
        assert_eq!(arr.value(1), Some(&"x".to_string()));
        assert_eq!(arr.value(2), Some(&"y".to_string()));
    }
}
