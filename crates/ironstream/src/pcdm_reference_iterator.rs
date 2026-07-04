// FILE: pcdm_reference_iterator.rs
// occt: PCDM_ReferenceIterator

use crate::pcdm_reference::PCDMReference;

/// Iterator over document references
pub struct PCDMReferenceIterator {
    references: Vec<PCDMReference>,
    iterator: usize,
}

impl PCDMReferenceIterator {
    /// Create a new reference iterator
    pub fn new() -> Self {
        PCDMReferenceIterator {
            references: Vec::new(),
            iterator: 0,
        }
    }

    /// Add a reference to the iterator
    pub fn add_reference(&mut self, reference: PCDMReference) {
        self.references.push(reference);
    }

    /// Check if there are more references
    pub fn more(&self) -> bool {
        self.iterator < self.references.len()
    }

    /// Move to next reference
    pub fn next(&mut self) {
        if self.more() {
            self.iterator += 1;
        }
    }

    /// Get current reference
    pub fn current(&self) -> Option<&PCDMReference> {
        self.references.get(self.iterator)
    }

    /// Get reference by index
    pub fn reference_at(&self, index: usize) -> Option<&PCDMReference> {
        self.references.get(index)
    }

    /// Get the count of references
    pub fn count(&self) -> usize {
        self.references.len()
    }
}

impl Default for PCDMReferenceIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_creation() {
        let iter = PCDMReferenceIterator::new();
        assert!(!iter.more());
    }

    #[test]
    fn test_add_reference() {
        let mut iter = PCDMReferenceIterator::new();
        let ref_obj = PCDMReference::with_data(1, "doc.xml", 1);
        iter.add_reference(ref_obj);
        assert!(iter.more());
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn test_iterate() {
        let mut iter = PCDMReferenceIterator::new();
        iter.add_reference(PCDMReference::with_data(1, "doc1.xml", 1));
        iter.add_reference(PCDMReference::with_data(2, "doc2.xml", 1));

        assert_eq!(iter.count(), 2);
        assert!(iter.more());
        assert_eq!(iter.current().unwrap().reference_identifier(), 1);

        iter.next();
        assert!(iter.more());
        assert_eq!(iter.current().unwrap().reference_identifier(), 2);

        iter.next();
        assert!(!iter.more());
    }
}
