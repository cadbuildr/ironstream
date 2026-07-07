// FILE: step_element_array1_of_curve_element_end_release_packet.rs
// occt: StepElement_Array1OfCurveElementEndReleasePacket

/// Rust port of OCCT's deprecated StepElement_Array1OfCurveElementEndReleasePacket.
/// A 1-based fixed-length array wrapping a Vec, mimicking NCollection_Array1 semantics.
#[derive(Clone, Debug)]
pub struct StepElementArray1OfCurveElementEndReleasePacket {
    data: Vec<String>,
    lower: i32,
}

impl StepElementArray1OfCurveElementEndReleasePacket {
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
        let arr = StepElementArray1OfCurveElementEndReleasePacket::new(1, 5);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 5);
    }

    #[test]
    fn test_array1_access() {
        let mut arr = StepElementArray1OfCurveElementEndReleasePacket::new(1, 3);
        arr.set_value(1, "packet1".to_string());
        arr.set_value(2, "packet2".to_string());
        arr.set_value(3, "packet3".to_string());

        assert_eq!(arr.value(1), Some(&"packet1".to_string()));
        assert_eq!(arr.value(2), Some(&"packet2".to_string()));
        assert_eq!(arr.value(3), Some(&"packet3".to_string()));
    }

    #[test]
    fn test_array1_bounds() {
        let arr = StepElementArray1OfCurveElementEndReleasePacket::new(1, 2);
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(3), None);
    }

    #[test]
    fn test_array1_from_vec() {
        let values = vec!["p1".to_string(), "p2".to_string()];
        let arr = StepElementArray1OfCurveElementEndReleasePacket::from_vec(1, values);

        assert_eq!(arr.length(), 2);
        assert_eq!(arr.value(1), Some(&"p1".to_string()));
        assert_eq!(arr.value(2), Some(&"p2".to_string()));
    }
}
