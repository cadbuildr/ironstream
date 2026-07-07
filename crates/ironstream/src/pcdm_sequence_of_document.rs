// FILE: pcdm_sequence_of_document.rs
// occt: PCDM_SequenceOfDocument

//! Deprecated: PCDM_SequenceOfDocument is a type alias for NCollection_Sequence<Handle(PCDM_Document)>.

use std::collections::VecDeque;

/// Document handle placeholder
#[derive(Debug, Clone)]
pub struct Document {
    id: u32,
}

impl Document {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Sequence of documents
#[derive(Debug, Clone)]
pub struct Sequence {
    documents: VecDeque<Document>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            documents: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    pub fn len(&self) -> usize {
        self.documents.len()
    }

    pub fn append(&mut self, doc: Document) {
        self.documents.push_back(doc);
    }

    pub fn prepend(&mut self, doc: Document) {
        self.documents.push_front(doc);
    }

    pub fn first(&self) -> Option<&Document> {
        self.documents.front()
    }

    pub fn last(&self) -> Option<&Document> {
        self.documents.back()
    }

    pub fn value(&self, index: usize) -> Option<&Document> {
        self.documents.get(index)
    }

    pub fn clear(&mut self) {
        self.documents.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.documents.iter()
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for PCDM_SequenceOfDocument
pub type PcdmSequenceOfDocument = Sequence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_create() {
        let seq: PcdmSequenceOfDocument = Sequence::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = Sequence::new();
        seq.append(Document::new(1));
        seq.append(Document::new(2));

        assert_eq!(seq.len(), 2);
        assert_eq!(seq.first().unwrap().id(), 1);
        assert_eq!(seq.last().unwrap().id(), 2);
    }

    #[test]
    fn test_sequence_prepend() {
        let mut seq = Sequence::new();
        seq.prepend(Document::new(1));
        seq.prepend(Document::new(2));

        assert_eq!(seq.first().unwrap().id(), 2);
        assert_eq!(seq.last().unwrap().id(), 1);
    }

    #[test]
    fn test_sequence_value() {
        let mut seq = Sequence::new();
        seq.append(Document::new(10));
        seq.append(Document::new(20));

        assert_eq!(seq.value(0).unwrap().id(), 10);
        assert_eq!(seq.value(1).unwrap().id(), 20);
        assert!(seq.value(2).is_none());
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = Sequence::new();
        seq.append(Document::new(1));
        seq.append(Document::new(2));

        seq.clear();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_iter() {
        let mut seq = Sequence::new();
        seq.append(Document::new(1));
        seq.append(Document::new(2));
        seq.append(Document::new(3));

        let ids: Vec<u32> = seq.iter().map(|d| d.id()).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_default() {
        let seq = Sequence::default();
        assert!(seq.is_empty());
    }
}
