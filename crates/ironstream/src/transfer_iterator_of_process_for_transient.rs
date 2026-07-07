// FILE: transfer_iterator_of_process_for_transient.rs
// occt: Transfer_IteratorOfProcessForTransient

/// An iterator over results in a transient-based transfer process.
/// Allows sequential access to transient transfer results.
#[derive(Clone, Debug)]
pub struct TransferIteratorOfProcessForTransient {
    /// Current position in iteration
    position: usize,
    /// Items in the collection
    items: Vec<u32>,
}

impl TransferIteratorOfProcessForTransient {
    /// Creates a new iterator.
    pub fn new() -> Self {
        Self {
            position: 0,
            items: Vec::new(),
        }
    }

    /// Creates an iterator with items.
    pub fn with_items(items: Vec<u32>) -> Self {
        Self { position: 0, items }
    }

    /// Returns the current item if available.
    pub fn current(&self) -> Option<u32> {
        if self.position < self.items.len() {
            Some(self.items[self.position])
        } else {
            None
        }
    }

    /// Moves to the next item.
    pub fn next(&mut self) {
        if self.position < self.items.len() {
            self.position += 1;
        }
    }

    /// Returns whether there are more items.
    pub fn has_more(&self) -> bool {
        self.position < self.items.len()
    }

    /// Resets the iterator to the beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Returns the number of items.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Adds an item to the iterator.
    pub fn add_item(&mut self, item: u32) {
        self.items.push(item);
    }
}

impl Default for TransferIteratorOfProcessForTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let iter = TransferIteratorOfProcessForTransient::new();
        assert_eq!(iter.size(), 0);
        assert!(!iter.has_more());
    }

    #[test]
    fn test_with_items() {
        let iter = TransferIteratorOfProcessForTransient::with_items(vec![1, 2, 3]);
        assert_eq!(iter.size(), 3);
        assert!(iter.has_more());
        assert_eq!(iter.current(), Some(1));
    }

    #[test]
    fn test_iteration() {
        let mut iter = TransferIteratorOfProcessForTransient::with_items(vec![100, 200, 300]);

        assert_eq!(iter.current(), Some(100));
        iter.next();
        assert_eq!(iter.current(), Some(200));
        iter.next();
        assert_eq!(iter.current(), Some(300));
        iter.next();
        assert!(!iter.has_more());
    }

    #[test]
    fn test_reset() {
        let mut iter = TransferIteratorOfProcessForTransient::with_items(vec![1, 2, 3, 4]);
        iter.next();
        iter.next();
        assert_eq!(iter.current(), Some(3));

        iter.reset();
        assert_eq!(iter.current(), Some(1));
    }
}
