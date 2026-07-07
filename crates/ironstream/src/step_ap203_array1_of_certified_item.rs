// FILE: step_ap203_array1_of_certified_item.rs
// occt: StepAP203_Array1OfCertifiedItem

/// Deprecated typedef for backward compatibility.
/// A 1-based fixed-size array of certified items.
/// Corresponds to NCollection_Array1<StepAP203_CertifiedItem>
pub struct StepAp203Array1OfCertifiedItem {
    data: Vec<Option<String>>,
    lower: usize,
    upper: usize,
}

impl StepAp203Array1OfCertifiedItem {
    /// Create a new array with specified bounds.
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(lower > 0 && upper >= lower);
        let mut data = vec![None; upper + 1];
        StepAp203Array1OfCertifiedItem {
            data,
            lower,
            upper,
        }
    }

    /// Get value at 1-based index.
    pub fn value(&self, index: usize) -> Option<&String> {
        if index >= self.lower && index <= self.upper {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    /// Set value at 1-based index.
    pub fn set_value(&mut self, index: usize, value: String) -> bool {
        if index >= self.lower && index <= self.upper {
            self.data[index] = Some(value);
            true
        } else {
            false
        }
    }

    /// Get lower bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get upper bound.
    pub fn upper(&self) -> usize {
        self.upper
    }

    /// Get length.
    pub fn len(&self) -> usize {
        if self.upper >= self.lower {
            self.upper - self.lower + 1
        } else {
            0
        }
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.upper < self.lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_bounds() {
        let arr = StepAp203Array1OfCertifiedItem::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_set_get() {
        let mut arr = StepAp203Array1OfCertifiedItem::new(1, 3);
        assert!(arr.set_value(1, "cert1".to_string()));
        assert_eq!(arr.value(1), Some(&"cert1".to_string()));
    }

    #[test]
    fn test_bounds_check() {
        let mut arr = StepAp203Array1OfCertifiedItem::new(2, 5);
        assert!(!arr.set_value(1, "bad".to_string()));
        assert!(!arr.set_value(6, "bad".to_string()));
    }
}
