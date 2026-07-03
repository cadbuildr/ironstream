// FILE: int_walk_vector_of_integer.rs
// occt: IntWalk_VectorOfInteger

/// Vector of integers for walking algorithm
#[derive(Clone, Debug)]
pub struct VectorOfInteger {
    data: Vec<i32>,
}

impl VectorOfInteger {
    /// Create new vector
    pub fn new() -> Self {
        VectorOfInteger { data: Vec::new() }
    }

    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        VectorOfInteger {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Get value at index
    pub fn value(&self, index: usize) -> Option<i32> {
        self.data.get(index).copied()
    }

    /// Append value
    pub fn append(&mut self, value: i32) {
        self.data.push(value);
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for VectorOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = VectorOfInteger::new();
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_append() {
        let mut v = VectorOfInteger::new();
        v.append(5);
        v.append(10);
        assert_eq!(v.len(), 2);
        assert_eq!(v.value(0), Some(5));
        assert_eq!(v.value(1), Some(10));
    }
}
