// FILE: transfer_iterator_of_process_for_finder.rs
// occt: Transfer_IteratorOfProcessForFinder

/// An iterator over results in a finder-based transfer process.
/// Allows sequential access to transfer results.
#[derive(Clone, Debug)]
pub struct TransferIteratorOfProcessForFinder {
    /// Current position in iteration
    position: usize,
    /// Items in the collection
    items: Vec<u32>,
}

impl TransferIteratorOfProcessForFinder {
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

impl Default for TransferIteratorOfProcessForFinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let iter = TransferIteratorOfProcessForFinder::new();
        assert_eq!(iter.size(), 0);
        assert!(!iter.has_more());
    }

    #[test]
    fn test_with_items() {
        let iter = TransferIteratorOfProcessForFinder::with_items(vec![1, 2, 3]);
        assert_eq!(iter.size(), 3);
        assert!(iter.has_more());
        assert_eq!(iter.current(), Some(1));
    }

    #[test]
    fn test_iteration() {
        let mut iter = TransferIteratorOfProcessForFinder::with_items(vec![10, 20, 30]);

        assert_eq!(iter.current(), Some(10));
        iter.next();
        assert_eq!(iter.current(), Some(20));
        iter.next();
        assert_eq!(iter.current(), Some(30));
        iter.next();
        assert!(!iter.has_more());
        assert_eq!(iter.current(), None);
    }

    #[test]
    fn test_reset() {
        let mut iter = TransferIteratorOfProcessForFinder::with_items(vec![1, 2, 3]);
        iter.next();
        iter.next();
        assert_eq!(iter.current(), Some(3));

        iter.reset();
        assert_eq!(iter.current(), Some(1));
    }

    #[test]
    fn test_add_item() {
        let mut iter = TransferIteratorOfProcessForFinder::new();
        iter.add_item(5);
        iter.add_item(10);
        assert_eq!(iter.size(), 2);
        assert_eq!(iter.current(), Some(5));
    }
}
