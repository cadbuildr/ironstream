// FILE: select_mgr_and_filter.rs
// occt: SelectMgr_AndFilter

// A simple composition filter that returns true only if all filters pass.
// This represents the AND logic for multiple selection filters.
pub struct SelectMgrAndFilter {
    filters: Vec<Box<dyn SelectMgrFilterOps>>,
}

// Trait representing filter operations for this module
pub trait SelectMgrFilterOps {
    fn is_ok(&self) -> bool;
}

impl SelectMgrAndFilter {
    /// Constructs an empty AND filter
    pub fn new() -> Self {
        SelectMgrAndFilter {
            filters: Vec::new(),
        }
    }

    /// Adds a filter to the composition
    pub fn add(&mut self, filter: Box<dyn SelectMgrFilterOps>) {
        self.filters.push(filter);
    }

    /// Returns true if all filters pass, false otherwise
    pub fn is_ok(&self) -> bool {
        // Empty filter list returns true (vacuous truth)
        if self.filters.is_empty() {
            return true;
        }
        // All filters must pass for AND logic
        self.filters.iter().all(|f| f.is_ok())
    }

    /// Returns true if the filter list is empty
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// Clears all filters
    pub fn clear(&mut self) {
        self.filters.clear();
    }
}

impl Default for SelectMgrAndFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestFilter {
        passes: bool,
    }

    impl SelectMgrFilterOps for TestFilter {
        fn is_ok(&self) -> bool {
            self.passes
        }
    }

    #[test]
    fn test_empty_filter_returns_true() {
        let filter = SelectMgrAndFilter::new();
        assert!(filter.is_ok());
    }

    #[test]
    fn test_single_passing_filter() {
        let mut filter = SelectMgrAndFilter::new();
        filter.add(Box::new(TestFilter { passes: true }));
        assert!(filter.is_ok());
    }

    #[test]
    fn test_single_failing_filter() {
        let mut filter = SelectMgrAndFilter::new();
        filter.add(Box::new(TestFilter { passes: false }));
        assert!(!filter.is_ok());
    }

    #[test]
    fn test_multiple_filters_all_pass() {
        let mut filter = SelectMgrAndFilter::new();
        filter.add(Box::new(TestFilter { passes: true }));
        filter.add(Box::new(TestFilter { passes: true }));
        filter.add(Box::new(TestFilter { passes: true }));
        assert!(filter.is_ok());
    }

    #[test]
    fn test_multiple_filters_one_fails() {
        let mut filter = SelectMgrAndFilter::new();
        filter.add(Box::new(TestFilter { passes: true }));
        filter.add(Box::new(TestFilter { passes: false }));
        filter.add(Box::new(TestFilter { passes: true }));
        assert!(!filter.is_ok());
    }

    #[test]
    fn test_clear_filters() {
        let mut filter = SelectMgrAndFilter::new();
        filter.add(Box::new(TestFilter { passes: true }));
        assert!(!filter.is_empty());
        filter.clear();
        assert!(filter.is_empty());
        assert!(filter.is_ok());
    }
}
