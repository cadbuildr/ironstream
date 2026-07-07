// FILE: bopds_list_of_pave.rs
// occt: BOPDS_ListOfPave

//! NCollection alias: List<BOPDS_Pave>
//! Deprecated type for backward compatibility.

use std::collections::LinkedList;

/// Deprecated: BOPDS_ListOfPave
pub type BOPDSListOfPave = LinkedList<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let list: BOPDSListOfPave = LinkedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_operations() {
        let mut list: BOPDSListOfPave = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 2);
    }
}
