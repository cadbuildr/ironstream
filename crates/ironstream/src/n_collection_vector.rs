// FILE: n_collection_vector.rs
// occt: NCollection_Vector

/// Generic dynamic array (vector) container.
#[derive(Clone)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    /// Create empty vector.
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    /// Create vector with initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Add element to end.
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    /// Remove and return last element.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Get number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get reference at index.
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    /// Get mutable reference at index.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.data.get_mut(idx)
    }

    /// Clear all elements.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Access as slice.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Access as mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let v: Vector<i32> = Vector::new();
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut v = Vector::new();
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut v = Vector::new();
        v.push("a");
        v.push("b");
        assert_eq!(v.get(0), Some(&"a"));
        assert_eq!(v.get(1), Some(&"b"));
        assert_eq!(v.get(2), None);
    }

    #[test]
    fn test_clear() {
        let mut v = Vector::new();
        v.push(1);
        v.push(2);
        v.clear();
        assert!(v.is_empty());
    }
}
