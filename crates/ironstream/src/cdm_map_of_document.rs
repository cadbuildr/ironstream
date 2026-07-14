// FILE: cdm_map_of_document.rs
// occt: CDM_MapOfDocument
// occt-ref: CDM_MapIteratorOfMapOfDocument

//! Deprecated type aliases for backward compatibility.
//! Use HashSet<Arc<CdmDocument>> directly instead.

use std::sync::Arc;
use std::collections::HashSet;

/// Document handle type (opaque marker).
pub struct CdmDocumentHandle;

impl std::hash::Hash for CdmDocumentHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self as *const Self as usize).hash(state);
    }
}

impl PartialEq for CdmDocumentHandle {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for CdmDocumentHandle {}

/// Deprecated map of document handles (set semantics, no duplicates).
/// Maps to NCollection_Map<opencascade::handle<CDM_Document>>.
pub type CdmMapOfDocument = HashSet<Arc<CdmDocumentHandle>>;

/// Deprecated iterator over a map of documents.
/// Maps to NCollection_Map<...>::Iterator.
pub struct CdmMapIteratorOfMapOfDocument<'a> {
    iter: std::collections::hash_set::Iter<'a, Arc<CdmDocumentHandle>>,
}

impl<'a> CdmMapIteratorOfMapOfDocument<'a> {
    /// Creates a new iterator over the map.
    pub fn new(map: &'a CdmMapOfDocument) -> Self {
        CdmMapIteratorOfMapOfDocument {
            iter: map.iter(),
        }
    }
}

impl<'a> Iterator for CdmMapIteratorOfMapOfDocument<'a> {
    type Item = &'a Arc<CdmDocumentHandle>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_of_document_creation() {
        let map: CdmMapOfDocument = HashSet::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_of_document_insert() {
        let mut map: CdmMapOfDocument = HashSet::new();
        let doc = Arc::new(CdmDocumentHandle);
        let result = map.insert(doc.clone());
        assert!(result);
        assert_eq!(map.len(), 1);

        // Inserting the same arc again should fail (duplicate)
        let result2 = map.insert(doc.clone());
        assert!(!result2);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_iterator() {
        let mut map: CdmMapOfDocument = HashSet::new();
        map.insert(Arc::new(CdmDocumentHandle));
        map.insert(Arc::new(CdmDocumentHandle));
        map.insert(Arc::new(CdmDocumentHandle));

        let iter = CdmMapIteratorOfMapOfDocument::new(&map);
        let count = iter.count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_map_contains() {
        let mut map: CdmMapOfDocument = HashSet::new();
        let doc = Arc::new(CdmDocumentHandle);
        map.insert(doc.clone());

        assert!(map.contains(&doc));
    }

    #[test]
    fn test_map_operations() {
        let mut map: CdmMapOfDocument = HashSet::new();
        let doc1 = Arc::new(CdmDocumentHandle);
        let doc2 = Arc::new(CdmDocumentHandle);

        map.insert(doc1.clone());
        map.insert(doc2.clone());

        assert_eq!(map.len(), 2);
        assert!(map.contains(&doc1));
        assert!(map.contains(&doc2));

        map.remove(&doc1);
        assert_eq!(map.len(), 1);
        assert!(!map.contains(&doc1));
        assert!(map.contains(&doc2));
    }
}
