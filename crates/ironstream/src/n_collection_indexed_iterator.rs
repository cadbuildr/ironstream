// FILE: n_collection_indexed_iterator.rs
// occt: NCollection_IndexedIterator

/// Iterator for indexed collections.
pub struct IndexedIterator<T> {
    data: Vec<T>,
    index: usize,
}

impl<T: Clone> IndexedIterator<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, index: 0 }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn current_index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> Iterator for IndexedIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
