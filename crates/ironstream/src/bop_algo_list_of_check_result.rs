// FILE: bop_algo_list_of_check_result.rs
// occt: BOPAlgo_ListOfCheckResult

//! NCollection alias: List of BOPAlgo_CheckResult
//! Deprecated type for backward compatibility.

use std::collections::LinkedList;

/// Deprecated: BOPAlgo_ListOfCheckResult
/// Use `std::collections::LinkedList<BOPAlgo_CheckResult>` directly instead.
pub type BOPAlgoListOfCheckResult = LinkedList<u32>; // Placeholder: actual BOPAlgo_CheckResult would be a real type

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let list: BOPAlgoListOfCheckResult = LinkedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_push_pop() {
        let mut list: BOPAlgoListOfCheckResult = LinkedList::new();
        list.push_back(42);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(42));
        assert!(list.is_empty());
    }
}
