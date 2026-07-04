// FILE: n_collection_local_array.rs
// occt: NCollection_LocalArray

/// Local (stack-allocated) array.
pub struct LocalArray<T: Clone, const N: usize> {
    data: [Option<T>; N],
    size: usize,
}

impl<T: Clone + Default, const N: usize> LocalArray<T, N> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            size: 0,
        }
    }

    pub fn with_size(size: usize) -> Self {
        assert!(size <= N);
        let mut arr = Self::new();
        arr.size = size;
        arr
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: Clone + Default, const N: usize> Default for LocalArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr: LocalArray<i32, 10> = LocalArray::new();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_with_size() {
        let arr: LocalArray<i32, 10> = LocalArray::with_size(5);
        assert_eq!(arr.len(), 5);
    }
}
