// FILE: select_mgr_filter_type.rs
// occt: SelectMgr_FilterType

/// Enumeration defines the filter type for selection filtering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectMgrFilterType {
    /// An object should be suitable for all filters (AND logic)
    And,
    /// An object should be suitable for at least one filter (OR logic)
    Or,
}

impl Default for SelectMgrFilterType {
    fn default() -> Self {
        SelectMgrFilterType::Or
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_type_default() {
        assert_eq!(SelectMgrFilterType::default(), SelectMgrFilterType::Or);
    }

    #[test]
    fn test_filter_type_and() {
        let ft = SelectMgrFilterType::And;
        assert_eq!(ft, SelectMgrFilterType::And);
    }

    #[test]
    fn test_filter_type_or() {
        let ft = SelectMgrFilterType::Or;
        assert_eq!(ft, SelectMgrFilterType::Or);
    }

    #[test]
    fn test_filter_type_inequality() {
        assert_ne!(SelectMgrFilterType::And, SelectMgrFilterType::Or);
    }

    #[test]
    fn test_filter_type_clone() {
        let ft1 = SelectMgrFilterType::And;
        let ft2 = ft1;
        assert_eq!(ft1, ft2);
    }

    #[test]
    fn test_filter_type_in_hash_set() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SelectMgrFilterType::And);
        set.insert(SelectMgrFilterType::Or);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&SelectMgrFilterType::And));
        assert!(set.contains(&SelectMgrFilterType::Or));
    }
}
