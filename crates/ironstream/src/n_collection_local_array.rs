// FILE: n_collection_local_array.rs
// occt: NCollection_LocalArray

/// Local (stack-preferred) array, port of NCollection_LocalArray<Item, MAX_ARRAY_SIZE>.
/// Uses an inline buffer of capacity N; allocations larger than N fall back
/// to the heap, mirroring the OCCT template behavior.
pub struct LocalArray<T: Clone + Default, const N: usize> {
    buffer: [T; N],
    heap: Option<Vec<T>>,
    size: usize,
}

impl<T: Clone + Default, const N: usize> LocalArray<T, N> {
    /// Empty array (Allocate(0)).
    pub fn new() -> Self {
        Self {
            buffer: std::array::from_fn(|_| T::default()),
            heap: None,
            size: 0,
        }
    }

    /// Array pre-allocated with `size` default-initialized items.
    pub fn with_size(size: usize) -> Self {
        let mut arr = Self::new();
        arr.allocate(size);
        arr
    }

    /// (Re)allocate the array. Uses the local buffer if `size <= N`,
    /// otherwise allocates on the heap — like NCollection_LocalArray::Allocate.
    pub fn allocate(&mut self, size: usize) {
        if size > N {
            self.heap = Some(vec![T::default(); size]);
        } else {
            self.heap = None;
            for item in self.buffer[..size].iter_mut() {
                *item = T::default();
            }
        }
        self.size = size;
    }

    /// Number of allocated items (Size()).
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// True when the inline (stack) buffer is in use.
    pub fn is_using_local_buffer(&self) -> bool {
        self.heap.is_none()
    }

    /// Slice view of the allocated items (operator theItem*).
    pub fn as_slice(&self) -> &[T] {
        match &self.heap {
            Some(v) => &v[..self.size],
            None => &self.buffer[..self.size],
        }
    }

    /// Mutable slice view of the allocated items.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        match &mut self.heap {
            Some(v) => &mut v[..self.size],
            None => &mut self.buffer[..self.size],
        }
    }
}

impl<T: Clone + Default, const N: usize> std::ops::Index<usize> for LocalArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.as_slice()[index]
    }
}

impl<T: Clone + Default, const N: usize> std::ops::IndexMut<usize> for LocalArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.as_mut_slice()[index]
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
        assert!(arr.is_using_local_buffer());
    }

    #[test]
    fn test_with_size_local() {
        let arr: LocalArray<i32, 10> = LocalArray::with_size(5);
        assert_eq!(arr.len(), 5);
        assert!(arr.is_using_local_buffer());
        assert_eq!(arr.as_slice(), &[0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_with_size_heap() {
        let arr: LocalArray<i32, 10> = LocalArray::with_size(25);
        assert_eq!(arr.len(), 25);
        assert!(!arr.is_using_local_buffer());
        assert!(arr.as_slice().iter().all(|&v| v == 0));
    }

    #[test]
    fn test_indexing() {
        let mut arr: LocalArray<i32, 10> = LocalArray::with_size(4);
        arr[0] = 7;
        arr[3] = -2;
        assert_eq!(arr[0], 7);
        assert_eq!(arr[1], 0);
        assert_eq!(arr[3], -2);
    }

    #[test]
    fn test_reallocate_switches_storage() {
        let mut arr: LocalArray<i32, 4> = LocalArray::with_size(3);
        assert!(arr.is_using_local_buffer());
        arr.allocate(8);
        assert_eq!(arr.len(), 8);
        assert!(!arr.is_using_local_buffer());
        arr.allocate(2);
        assert_eq!(arr.len(), 2);
        assert!(arr.is_using_local_buffer());
    }

    #[test]
    fn test_allocate_resets_values() {
        let mut arr: LocalArray<i32, 4> = LocalArray::with_size(3);
        arr[0] = 42;
        arr.allocate(3);
        assert_eq!(arr[0], 0);
    }
}
