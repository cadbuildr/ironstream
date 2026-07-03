// FILE: n_collection_h_array1.rs
// occt: NCollection_HArray1

/// Handle to 1D array.
pub struct NCollectionHArray1<T> {
    data: Vec<T>,
}

impl<T: Clone> NCollectionHArray1<T> {
    pub fn new(size: usize, init_value: T) -> Self {
        Self {
            data: vec![init_value; size],
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.data.len() {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> bool {
        if index < self.data.len() {
            self.data[index] = value;
            true
        } else {
            false
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr = NCollectionHArray1::new(5, 0i32);
        assert_eq!(arr.size(), 5);
    }

    #[test]
    fn test_get_set() {
        let mut arr = NCollectionHArray1::new(5, 0i32);
        arr.set(2, 42);
        assert_eq!(*arr.get(2).unwrap(), 42);
    }
}
