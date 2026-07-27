// FILE: select_mgr_and_or_filter.rs
// occt: SelectMgr_AndOrFilter
// occt-ref: SelectMgr_FilterType

use std::collections::HashSet;

/// Enumeration defining the filter type: AND or OR logic
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectMgrFilterType {
    /// An object should be suitable for all filters (AND logic)
    And,
    /// An object should be suitable for at least one filter (OR logic)
    Or,
}

/// Trait for filter operations needed by this module
pub trait FilterOps {
    fn is_ok(&self) -> bool;
}

/// A framework to define an OR or AND selection filter.
/// Uses either AND or OR logic based on the filter type.
pub struct SelectMgrAndOrFilter {
    filter_type: SelectMgrFilterType,
    filters: Vec<Box<dyn FilterOps>>,
    disabled_objects: HashSet<usize>, // Simple representation of disabled objects
}

impl SelectMgrAndOrFilter {
    /// Constructs an empty selection filter with the specified type.
    /// By default, SelectMgrFilterType::Or is used.
    pub fn new(filter_type: SelectMgrFilterType) -> Self {
        SelectMgrAndOrFilter {
            filter_type,
            filters: Vec::new(),
            disabled_objects: HashSet::new(),
        }
    }

    /// Indicates that the selected object passes the filter.
    /// Returns true if the object is not disabled and passes the filter logic.
    pub fn is_ok(&self, obj_id: usize) -> bool {
        // If object is disabled, it never passes
        if self.disabled_objects.contains(&obj_id) {
            return false;
        }

        match self.filter_type {
            SelectMgrFilterType::Or => {
                // For OR: at least one filter must pass (empty list returns false)
                !self.filters.is_empty() && self.filters.iter().any(|f| f.is_ok())
            }
            SelectMgrFilterType::And => {
                // For AND: all filters must pass (empty list returns true)
                self.filters.iter().all(|f| f.is_ok())
            }
        }
    }

    /// Disable selection of specified objects.
    /// Objects in the disabled set will never pass the filter.
    pub fn set_disabled_objects(&mut self, disabled: HashSet<usize>) {
        self.disabled_objects = disabled;
    }

    /// Returns the current filter type (AND or OR).
    pub fn filter_type(&self) -> SelectMgrFilterType {
        self.filter_type
    }

    /// Sets the filter type.
    /// SelectMgrFilterType::Or is used by default.
    pub fn set_filter_type(&mut self, filter_type: SelectMgrFilterType) {
        self.filter_type = filter_type;
    }

    /// Adds a filter to this composition
    pub fn add_filter(&mut self, filter: Box<dyn FilterOps>) {
        self.filters.push(filter);
    }

    /// Clears all filters
    pub fn clear(&mut self) {
        self.filters.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestFilter {
        passes: bool,
    }

    impl FilterOps for TestFilter {
        fn is_ok(&self) -> bool {
            self.passes
        }
    }

    #[test]
    fn test_or_filter_no_filters_returns_false() {
        let filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::Or);
        assert!(!filter.is_ok(1));
    }

    #[test]
    fn test_or_filter_one_passes() {
        let mut filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::Or);
        filter.add_filter(Box::new(TestFilter { passes: true }));
        filter.add_filter(Box::new(TestFilter { passes: false }));
        assert!(filter.is_ok(1));
    }

    #[test]
    fn test_or_filter_all_fail() {
        let mut filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::Or);
        filter.add_filter(Box::new(TestFilter { passes: false }));
        filter.add_filter(Box::new(TestFilter { passes: false }));
        assert!(!filter.is_ok(1));
    }

    #[test]
    fn test_and_filter_no_filters_returns_true() {
        let filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::And);
        assert!(filter.is_ok(1));
    }

    #[test]
    fn test_and_filter_all_pass() {
        let mut filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::And);
        filter.add_filter(Box::new(TestFilter { passes: true }));
        filter.add_filter(Box::new(TestFilter { passes: true }));
        assert!(filter.is_ok(1));
    }

    #[test]
    fn test_and_filter_one_fails() {
        let mut filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::And);
        filter.add_filter(Box::new(TestFilter { passes: true }));
        filter.add_filter(Box::new(TestFilter { passes: false }));
        assert!(!filter.is_ok(1));
    }

    #[test]
    fn test_disabled_objects() {
        let mut filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::Or);
        filter.add_filter(Box::new(TestFilter { passes: true }));

        // Object 1 passes normally
        assert!(filter.is_ok(1));

        // Disable object 1
        let mut disabled = HashSet::new();
        disabled.insert(1);
        filter.set_disabled_objects(disabled);

        // Object 1 now fails due to being disabled
        assert!(!filter.is_ok(1));

        // Object 2 still passes
        assert!(filter.is_ok(2));
    }

    #[test]
    fn test_set_filter_type() {
        let mut filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::Or);
        assert_eq!(filter.filter_type(), SelectMgrFilterType::Or);

        filter.set_filter_type(SelectMgrFilterType::And);
        assert_eq!(filter.filter_type(), SelectMgrFilterType::And);
    }

    #[test]
    fn test_clear_filters() {
        let mut filter = SelectMgrAndOrFilter::new(SelectMgrFilterType::And);
        filter.add_filter(Box::new(TestFilter { passes: true }));

        filter.clear();
        assert!(filter.is_ok(1));
    }
}
