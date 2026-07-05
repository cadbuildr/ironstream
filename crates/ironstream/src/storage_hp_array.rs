// FILE: storage_hp_array.rs
// occt: Storage_HPArray

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct StoragePArray {
    lower: i32,
    upper: i32,
    data: Vec<u64>,
}

impl StoragePArray {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        Self {
            lower,
            upper,
            data: vec![0; size],
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn len(&self) -> i32 {
        self.upper - self.lower + 1
    }

    pub fn at(&self, idx: i32) -> u64 {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize]
    }

    pub fn set(&mut self, idx: i32, value: u64) {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize] = value;
    }
}

pub type Storage_HPArray = Arc<StoragePArray>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hparray_bounds() {
        let arr = Arc::new(StoragePArray::new(1, 5));
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_hparray_shared() {
        let arr1 = Arc::new(StoragePArray::new(1, 3));
        let arr2 = Arc::clone(&arr1);
        assert_eq!(Arc::strong_count(&arr1), 2);
    }
}
