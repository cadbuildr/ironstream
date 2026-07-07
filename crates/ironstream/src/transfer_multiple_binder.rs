// FILE: transfer_multiple_binder.rs
// occt: Transfer_MultipleBinder

/// A binder that can associate a source entity with multiple result entities.
/// Extends basic binder functionality to handle one-to-many mappings.
#[derive(Clone, Debug)]
pub struct TransferMultipleBinder {
    /// Collection of result IDs
    results: Vec<u32>,
}

impl TransferMultipleBinder {
    /// Creates a new multiple binder.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Adds a result to the binder.
    pub fn add_result(&mut self, result_id: u32) {
        if !self.results.contains(&result_id) {
            self.results.push(result_id);
        }
    }

    /// Returns the number of bound results.
    pub fn nb_results(&self) -> usize {
        self.results.len()
    }

    /// Returns whether this binder has multiple results.
    pub fn is_multiple(&self) -> bool {
        self.results.len() > 1
    }

    /// Returns a result at the given index.
    pub fn result(&self, index: usize) -> Option<u32> {
        if index < self.results.len() {
            Some(self.results[index])
        } else {
            None
        }
    }

    /// Returns all results.
    pub fn results(&self) -> &[u32] {
        &self.results
    }

    /// Clears all results.
    pub fn clear(&mut self) {
        self.results.clear();
    }
}

impl Default for TransferMultipleBinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let binder = TransferMultipleBinder::new();
        assert_eq!(binder.nb_results(), 0);
        assert!(!binder.is_multiple());
    }

    #[test]
    fn test_add_result() {
        let mut binder = TransferMultipleBinder::new();
        binder.add_result(10);
        assert_eq!(binder.nb_results(), 1);
        assert!(!binder.is_multiple());

        binder.add_result(20);
        assert_eq!(binder.nb_results(), 2);
        assert!(binder.is_multiple());

        // Adding duplicate should not increase count
        binder.add_result(10);
        assert_eq!(binder.nb_results(), 2);
    }

    #[test]
    fn test_result() {
        let mut binder = TransferMultipleBinder::new();
        binder.add_result(100);
        binder.add_result(200);
        binder.add_result(300);

        assert_eq!(binder.result(0), Some(100));
        assert_eq!(binder.result(1), Some(200));
        assert_eq!(binder.result(2), Some(300));
        assert_eq!(binder.result(3), None);
    }

    #[test]
    fn test_results() {
        let mut binder = TransferMultipleBinder::new();
        binder.add_result(10);
        binder.add_result(20);

        let results = binder.results();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], 10);
        assert_eq!(results[1], 20);
    }

    #[test]
    fn test_clear() {
        let mut binder = TransferMultipleBinder::new();
        binder.add_result(10);
        binder.add_result(20);
        assert_eq!(binder.nb_results(), 2);

        binder.clear();
        assert_eq!(binder.nb_results(), 0);
    }
}
