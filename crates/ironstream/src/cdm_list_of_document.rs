// FILE: cdm_list_of_document.rs
// occt: CDM_ListOfDocument, CDM_ListIteratorOfListOfDocument

//! Deprecated type aliases for backward compatibility.
//! Use VecDeque<Arc<CdmDocument>> directly instead.

use std::sync::Arc;
use std::collections::VecDeque;

/// Document reference type (opaque marker).
pub struct CdmDocumentHandle;

/// Deprecated list of document handles.
/// Maps to NCollection_List<opencascade::handle<CDM_Document>>.
pub type CdmListOfDocument = VecDeque<Arc<CdmDocumentHandle>>;

/// Deprecated iterator over a list of documents.
/// Maps to NCollection_List<...>::Iterator.
pub struct CdmListIteratorOfListOfDocument<'a> {
    items: &'a VecDeque<Arc<CdmDocumentHandle>>,
    index: usize,
}

impl<'a> CdmListIteratorOfListOfDocument<'a> {
    /// Creates a new iterator over the list.
    pub fn new(list: &'a CdmListOfDocument) -> Self {
        CdmListIteratorOfListOfDocument {
            items: list,
            index: 0,
        }
    }

    /// Returns true if there are more elements to iterate.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Returns a reference to the current element.
    pub fn value(&self) -> Option<&Arc<CdmDocumentHandle>> {
        self.items.get(self.index)
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }
}

impl<'a> Iterator for CdmListIteratorOfListOfDocument<'a> {
    type Item = &'a Arc<CdmDocumentHandle>;

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
    fn test_list_of_document_creation() {
        let list: CdmListOfDocument = VecDeque::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_of_document_push() {
        let mut list: CdmListOfDocument = VecDeque::new();
        let doc = Arc::new(CdmDocumentHandle);
        list.push_back(doc.clone());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_iterator() {
        let mut list: CdmListOfDocument = VecDeque::new();
        list.push_back(Arc::new(CdmDocumentHandle));
        list.push_back(Arc::new(CdmDocumentHandle));
        list.push_back(Arc::new(CdmDocumentHandle));

        let mut iter = CdmListIteratorOfListOfDocument::new(&list);
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
    fn test_list_iterator_as_rust_iterator() {
        let mut list: CdmListOfDocument = VecDeque::new();
        list.push_back(Arc::new(CdmDocumentHandle));
        list.push_back(Arc::new(CdmDocumentHandle));

        let iter = CdmListIteratorOfListOfDocument::new(&list);
        let count = iter.count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_list_operations() {
        let mut list: CdmListOfDocument = VecDeque::new();
        let doc1 = Arc::new(CdmDocumentHandle);
        let doc2 = Arc::new(CdmDocumentHandle);

        list.push_back(doc1);
        list.push_back(doc2);

        assert_eq!(list.len(), 2);
        assert!(list.front().is_some());
        assert!(list.back().is_some());

        list.clear();
        assert!(list.is_empty());
    }
}
