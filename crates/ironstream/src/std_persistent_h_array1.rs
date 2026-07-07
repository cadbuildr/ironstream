// FILE: std_persistent_h_array1.rs
// occt: StdPersistent_HArray1

/// Persistent 1D array
pub struct PersistentHArray1 {
    data: Vec<Option<i32>>,
}

impl PersistentHArray1 {
    /// Create a new persistent array
    pub fn new(size: usize) -> Self {
        PersistentHArray1 {
            data: vec![None; size],
        }
    }

    /// Get array size
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get element
    pub fn get(&self, index: usize) -> Option<&Option<i32>> {
        self.data.get(index)
    }

    /// Set element
    pub fn set(&mut self, index: usize, value: Option<i32>) {
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
        let arr = PersistentHArray1::new(10);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_get_set() {
        let mut arr = PersistentHArray1::new(5);
        arr.set(2, Some(42));
        assert_eq!(arr.get(2), Some(&Some(42)));
    }
}
