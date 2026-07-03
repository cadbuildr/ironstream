// FILE: n_collection_aliased_array.rs
// occt: NCollection_AliasedArray

/// Aliased array that references external memory.
pub struct NCollectionAliasedArray<'a> {
    data: &'a mut [u8],
    size: usize,
}

impl<'a> NCollectionAliasedArray<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        let size = data.len();
        Self { data, size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.size {
            Some(self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, value: u8) -> bool {
        if index < self.size {
            self.data[index] = value;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut buffer = [0u8; 10];
        let arr = NCollectionAliasedArray::new(&mut buffer);
        assert_eq!(arr.size(), 10);
    }

    #[test]
    fn test_get_set() {
        let mut buffer = [0u8; 5];
        let mut arr = NCollectionAliasedArray::new(&mut buffer);
        arr.set(2, 42);
        assert_eq!(arr.get(2), Some(42));
    }
}
