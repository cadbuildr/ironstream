// FILE: select_mgr_picking_strategy.rs
// occt: SelectMgr_PickingStrategy

/// Enumeration defines picking strategy - which entities detected by picking line
/// will be accepted, considering selection filters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectMgrPickingStrategy {
    /// The first detected entity passing selection filter is accepted (e.g. any)
    FirstAcceptable,
    /// Only topmost detected entity passing selection filter is accepted
    OnlyTopmost,
}

impl Default for SelectMgrPickingStrategy {
    fn default() -> Self {
        SelectMgrPickingStrategy::FirstAcceptable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picking_strategy_default() {
        assert_eq!(
            SelectMgrPickingStrategy::default(),
            SelectMgrPickingStrategy::FirstAcceptable
        );
    }

    #[test]
    fn test_first_acceptable() {
        let strategy = SelectMgrPickingStrategy::FirstAcceptable;
        assert_eq!(strategy, SelectMgrPickingStrategy::FirstAcceptable);
    }

    #[test]
    fn test_only_topmost() {
        let strategy = SelectMgrPickingStrategy::OnlyTopmost;
        assert_eq!(strategy, SelectMgrPickingStrategy::OnlyTopmost);
    }

    #[test]
    fn test_picking_strategy_inequality() {
        assert_ne!(
            SelectMgrPickingStrategy::FirstAcceptable,
            SelectMgrPickingStrategy::OnlyTopmost
        );
    }

    #[test]
    fn test_picking_strategy_clone() {
        let strategy1 = SelectMgrPickingStrategy::OnlyTopmost;
        let strategy2 = strategy1;
        assert_eq!(strategy1, strategy2);
    }

    #[test]
    fn test_picking_strategy_in_hash_set() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SelectMgrPickingStrategy::FirstAcceptable);
        set.insert(SelectMgrPickingStrategy::OnlyTopmost);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&SelectMgrPickingStrategy::FirstAcceptable));
        assert!(set.contains(&SelectMgrPickingStrategy::OnlyTopmost));
    }
}
