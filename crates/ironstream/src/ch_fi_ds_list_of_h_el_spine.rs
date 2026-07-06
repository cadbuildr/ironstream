// FILE: ch_fi_ds_list_of_h_el_spine.rs
// occt: ChFiDS_ListOfHElSpine, ChFiDS_ListIteratorOfListOfHElSpine

//! Deprecated type aliases for backward compatibility.
//! Use VecDeque<Arc<ChFiDsElSpine>> directly instead.

use std::sync::Arc;
use std::collections::VecDeque;

/// ElSpine handle type (opaque marker).
pub struct ChFiDsElSpineHandle;

/// Deprecated list of ElSpine handles.
/// Maps to NCollection_List<opencascade::handle<ChFiDS_ElSpine>>.
pub type ChFiDsListOfHElSpine = VecDeque<Arc<ChFiDsElSpineHandle>>;

/// Deprecated iterator over a list of ElSpine handles.
/// Maps to NCollection_List<...>::Iterator.
pub struct ChFiDsListIteratorOfListOfHElSpine<'a> {
    items: &'a VecDeque<Arc<ChFiDsElSpineHandle>>,
    index: usize,
}

impl<'a> ChFiDsListIteratorOfListOfHElSpine<'a> {
    /// Creates a new iterator over the list.
    pub fn new(list: &'a ChFiDsListOfHElSpine) -> Self {
        ChFiDsListIteratorOfListOfHElSpine {
            items: list,
            index: 0,
        }
    }

    /// Returns true if there are more elements to iterate.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Returns a reference to the current element.
    pub fn value(&self) -> Option<&Arc<ChFiDsElSpineHandle>> {
        self.items.get(self.index)
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }
}

impl<'a> Iterator for ChFiDsListIteratorOfListOfHElSpine<'a> {
    type Item = &'a Arc<ChFiDsElSpineHandle>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.items.get(self.index);
        if current.is_some() {
            self.index += 1;
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_of_h_el_spine_creation() {
        let list: ChFiDsListOfHElSpine = VecDeque::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_of_h_el_spine_push() {
        let mut list: ChFiDsListOfHElSpine = VecDeque::new();
        let spine = Arc::new(ChFiDsElSpineHandle);
        list.push_back(spine.clone());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_iterator_of_h_el_spine() {
        let mut list: ChFiDsListOfHElSpine = VecDeque::new();
        list.push_back(Arc::new(ChFiDsElSpineHandle));
        list.push_back(Arc::new(ChFiDsElSpineHandle));
        list.push_back(Arc::new(ChFiDsElSpineHandle));

        let mut iter = ChFiDsListIteratorOfListOfHElSpine::new(&list);
        assert!(iter.more());
        assert!(iter.value().is_some());

        iter.next();
        assert!(iter.more());

        iter.next();
        assert!(iter.more());

        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_list_iterator_of_h_el_spine_as_rust_iterator() {
        let mut list: ChFiDsListOfHElSpine = VecDeque::new();
        list.push_back(Arc::new(ChFiDsElSpineHandle));
        list.push_back(Arc::new(ChFiDsElSpineHandle));

        let iter = ChFiDsListIteratorOfListOfHElSpine::new(&list);
        let count = iter.count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_list_operations() {
        let mut list: ChFiDsListOfHElSpine = VecDeque::new();
        let spine1 = Arc::new(ChFiDsElSpineHandle);
        let spine2 = Arc::new(ChFiDsElSpineHandle);

        list.push_back(spine1);
        list.push_back(spine2);

        assert_eq!(list.len(), 2);
        assert!(list.front().is_some());
        assert!(list.back().is_some());

        list.clear();
        assert!(list.is_empty());
    }
}
