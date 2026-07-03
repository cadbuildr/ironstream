// FILE: n_collection_dynamic_array.rs
// occt: NCollection_DynamicArray

/// Dynamic array that grows as needed.
pub struct DynamicArray<T> {
    data: Vec<T>,
}

impl<T: Clone + Default> DynamicArray<T> {
    pub fn new() -> Self {
        DynamicArray {
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DynamicArray {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn append(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.data.get_mut(idx)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T: Clone + Default> Default for DynamicArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_array() {
        let mut arr = DynamicArray::<i32>::new();
        arr.append(1);
        arr.append(2);
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get(0), Some(&1));
    }
}
