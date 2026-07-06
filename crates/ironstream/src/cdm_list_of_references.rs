// FILE: cdm_list_of_references.rs
// occt: CDM_ListOfReferences, CDM_ListIteratorOfListOfReferences

//! Deprecated type aliases for backward compatibility.
//! Use VecDeque<Arc<CdmReference>> directly instead.

use std::sync::Arc;
use std::collections::VecDeque;

/// Reference handle type (opaque marker).
pub struct CdmReferenceHandle;

/// Deprecated list of reference handles.
/// Maps to NCollection_List<opencascade::handle<CDM_Reference>>.
pub type CdmListOfReferences = VecDeque<Arc<CdmReferenceHandle>>;

/// Deprecated iterator over a list of references.
/// Maps to NCollection_List<...>::Iterator.
pub struct CdmListIteratorOfListOfReferences<'a> {
    items: &'a VecDeque<Arc<CdmReferenceHandle>>,
    index: usize,
}

impl<'a> CdmListIteratorOfListOfReferences<'a> {
    /// Creates a new iterator over the list.
    pub fn new(list: &'a CdmListOfReferences) -> Self {
        CdmListIteratorOfListOfReferences {
            items: list,
            index: 0,
        }
    }

    /// Returns true if there are more elements to iterate.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Returns a reference to the current element.
    pub fn value(&self) -> Option<&Arc<CdmReferenceHandle>> {
        self.items.get(self.index)
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }
}

impl<'a> Iterator for CdmListIteratorOfListOfReferences<'a> {
    type Item = &'a Arc<CdmReferenceHandle>;

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
    fn test_list_of_references_creation() {
        let list: CdmListOfReferences = VecDeque::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_of_references_push() {
        let mut list: CdmListOfReferences = VecDeque::new();
        let ref_handle = Arc::new(CdmReferenceHandle);
        list.push_back(ref_handle.clone());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_iterator_of_references() {
        let mut list: CdmListOfReferences = VecDeque::new();
        list.push_back(Arc::new(CdmReferenceHandle));
        list.push_back(Arc::new(CdmReferenceHandle));
        list.push_back(Arc::new(CdmReferenceHandle));

        let mut iter = CdmListIteratorOfListOfReferences::new(&list);
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
    fn test_list_iterator_of_references_as_rust_iterator() {
        let mut list: CdmListOfReferences = VecDeque::new();
        list.push_back(Arc::new(CdmReferenceHandle));
        list.push_back(Arc::new(CdmReferenceHandle));

        let iter = CdmListIteratorOfListOfReferences::new(&list);
        let count = iter.count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_list_operations() {
        let mut list: CdmListOfReferences = VecDeque::new();
        let ref1 = Arc::new(CdmReferenceHandle);
        let ref2 = Arc::new(CdmReferenceHandle);

        list.push_back(ref1);
        list.push_back(ref2);

        assert_eq!(list.len(), 2);
        assert!(list.front().is_some());
        assert!(list.back().is_some());

        list.clear();
        assert!(list.is_empty());
    }
}
