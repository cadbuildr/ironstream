// FILE: storage_h_seq_of_root.rs
// occt: Storage_HSeqOfRoot

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct StorageSeqRoot {
    elements: Vec<u64>,
}

impl StorageSeqRoot {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn append(&mut self, value: u64) {
        self.elements.push(value);
    }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn first(&self) -> Option<u64> {
        self.elements.first().copied()
    }

    pub fn last(&self) -> Option<u64> {
        self.elements.last().copied()
    }

    pub fn value_at(&self, idx: usize) -> Option<u64> {
        self.elements.get(idx).copied()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

pub type Storage_HSeqOfRoot = Arc<StorageSeqRoot>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hseq_append_and_length() {
        let mut seq = StorageSeqRoot::new();
        assert_eq!(seq.length(), 0);

        seq.append(10);
        seq.append(20);
        seq.append(30);

        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_hseq_first_last() {
        let mut seq = StorageSeqRoot::new();
        seq.append(100);
        seq.append(200);
        seq.append(300);

        assert_eq!(seq.first(), Some(100));
        assert_eq!(seq.last(), Some(300));
    }

    #[test]
    fn test_hseq_value_at() {
        let mut seq = StorageSeqRoot::new();
        seq.append(11);
        seq.append(22);
        seq.append(33);

        assert_eq!(seq.value_at(0), Some(11));
        assert_eq!(seq.value_at(1), Some(22));
        assert_eq!(seq.value_at(2), Some(33));
        assert_eq!(seq.value_at(3), None);
    }

    #[test]
    fn test_hseq_shared() {
        let mut seq = StorageSeqRoot::new();
        seq.append(42);
        let seq_arc = Arc::new(seq);
        let seq_arc2 = Arc::clone(&seq_arc);

        assert_eq!(Arc::strong_count(&seq_arc), 2);
    }
}
