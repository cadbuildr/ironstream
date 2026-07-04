// FILE: shape_persistent_h_array1.rs
// occt: ShapePersistent_HArray1

/// Dynamic array of persistent objects (1D)
pub struct HArray1 {
    data: Vec<Option<String>>,
}

impl HArray1 {
    /// Create a new dynamic array
    pub fn new(size: usize) -> Self {
        HArray1 {
            data: vec![None; size],
        }
    }

    /// Get array size
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get element at index
    pub fn get(&self, index: usize) -> Option<&Option<String>> {
        self.data.get(index)
    }

    /// Set element at index
    pub fn set(&mut self, index: usize, value: Option<String>) {
        if let Some(elem) = self.data.get_mut(index) {
            *elem = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let arr = HArray1::new(10);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_get_set() {
        let mut arr = HArray1::new(5);
        arr.set(2, Some("value".to_string()));
        assert_eq!(arr.get(2), Some(&Some("value".to_string())));
    }
}
