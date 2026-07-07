// FILE: transfer_transfer_iterator.rs
// occt: Transfer_TransferIterator

/// Iterator over transfer results.
/// Allows sequential access through the results of a transfer operation.
#[derive(Clone, Debug)]
pub struct TransferTransferIterator {
    /// Current position
    position: usize,
    /// Results being iterated
    results: Vec<u32>,
}

impl TransferTransferIterator {
    /// Creates a new transfer iterator.
    pub fn new() -> Self {
        Self {
            position: 0,
            results: Vec::new(),
        }
    }

    /// Creates an iterator with results.
    pub fn with_results(results: Vec<u32>) -> Self {
        Self { position: 0, results }
    }

    /// Returns the current result.
    pub fn current(&self) -> Option<u32> {
        if self.position < self.results.len() {
            Some(self.results[self.position])
        } else {
            None
        }
    }

    /// Advances to the next result.
    pub fn next(&mut self) {
        if self.position < self.results.len() {
            self.position += 1;
        }
    }

    /// Returns whether there are more results.
    pub fn has_more(&self) -> bool {
        self.position < self.results.len()
    }

    /// Resets to the beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Returns the total number of results.
    pub fn nb_results(&self) -> usize {
        self.results.len()
    }

    /// Adds a result.
    pub fn add_result(&mut self, result: u32) {
        self.results.push(result);
    }
}

impl Default for TransferTransferIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let iter = TransferTransferIterator::new();
        assert_eq!(iter.nb_results(), 0);
        assert!(!iter.has_more());
    }

    #[test]
    fn test_with_results() {
        let iter = TransferTransferIterator::with_results(vec![10, 20, 30]);
        assert_eq!(iter.nb_results(), 3);
        assert!(iter.has_more());
    }

    #[test]
    fn test_iteration() {
        let mut iter = TransferTransferIterator::with_results(vec![100, 200, 300]);

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
        let mut iter = TransferTransferIterator::with_results(vec![1, 2, 3]);
        iter.next();
        iter.next();

        iter.reset();
        assert_eq!(iter.current(), Some(1));
    }

    #[test]
    fn test_add_result() {
        let mut iter = TransferTransferIterator::new();
        iter.add_result(50);
        iter.add_result(60);

        assert_eq!(iter.nb_results(), 2);
        assert_eq!(iter.current(), Some(50));
    }
}
