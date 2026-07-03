// FILE: n_collection_h_sequence.rs
// occt: NCollection_HSequence

/// Handle to sequence.
pub struct NCollectionHSequence<T> {
    data: Vec<T>,
}

impl<T> NCollectionHSequence<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn append(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.data.len() {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T> Default for NCollectionHSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let seq = NCollectionHSequence::<i32>::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_append() {
        let mut seq = NCollectionHSequence::new();
        seq.append(1);
        seq.append(2);
        seq.append(3);
        assert_eq!(seq.size(), 3);
    }

    #[test]
    fn test_get() {
        let mut seq = NCollectionHSequence::new();
        seq.append(10);
        seq.append(20);
        assert_eq!(*seq.get(1).unwrap(), 20);
    }
}
