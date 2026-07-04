// FILE: transfer_transient_list_binder.rs
// occt: Transfer_TransientListBinder

/// A binder that associates a source entity with a list of transient results.
/// Maps entities to multiple transient objects.
#[derive(Clone, Debug)]
pub struct TransferTransientListBinder {
    /// List of result entity IDs
    results: Vec<u32>,
}

impl TransferTransientListBinder {
    /// Creates a new transient list binder.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Creates a binder with initial results.
    pub fn with_results(results: Vec<u32>) -> Self {
        Self { results }
    }

    /// Adds a result to the list.
    pub fn add_result(&mut self, result_id: u32) {
        self.results.push(result_id);
    }

    /// Returns the number of results.
    pub fn nb_results(&self) -> usize {
        self.results.len()
    }

    /// Returns whether this binder has multiple results.
    pub fn is_multiple(&self) -> bool {
        self.results.len() > 1
    }

    /// Returns a result at the given index (1-based).
    pub fn result(&self, index: usize) -> Option<u32> {
        if index > 0 && index <= self.results.len() {
            Some(self.results[index - 1])
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

impl Default for TransferTransientListBinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let binder = TransferTransientListBinder::new();
        assert_eq!(binder.nb_results(), 0);
        assert!(!binder.is_multiple());
    }

    #[test]
    fn test_add_result() {
        let mut binder = TransferTransientListBinder::new();
        binder.add_result(10);
        assert_eq!(binder.nb_results(), 1);

        binder.add_result(20);
        assert_eq!(binder.nb_results(), 2);
        assert!(binder.is_multiple());
    }

    #[test]
    fn test_result() {
        let mut binder = TransferTransientListBinder::new();
        binder.add_result(100);
        binder.add_result(200);
        binder.add_result(300);

        assert_eq!(binder.result(1), Some(100));
        assert_eq!(binder.result(2), Some(200));
        assert_eq!(binder.result(3), Some(300));
        assert_eq!(binder.result(4), None);
    }

    #[test]
    fn test_with_results() {
        let binder = TransferTransientListBinder::with_results(vec![50, 60, 70]);
        assert_eq!(binder.nb_results(), 3);
        assert!(binder.is_multiple());
    }

    #[test]
    fn test_clear() {
        let mut binder = TransferTransientListBinder::new();
        binder.add_result(10);
        binder.add_result(20);

        binder.clear();
        assert_eq!(binder.nb_results(), 0);
    }
}
