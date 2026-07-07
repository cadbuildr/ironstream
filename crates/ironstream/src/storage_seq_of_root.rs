// FILE: storage_seq_of_root.rs
// occt: Storage_SeqOfRoot

/// Storage_SeqOfRoot: a sequence of Storage_Root elements.
///
/// This is a deprecated OCCT typedef for backward compatibility.
/// OCCT Sequence is a 1-based container with append/prepend operations.
#[derive(Debug, Clone)]
pub struct Storage_SeqOfRoot {
    elements: Vec<u64>,
}

impl Storage_SeqOfRoot {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn append(&mut self, value: u64) {
        self.elements.push(value);
    }

    pub fn prepend(&mut self, value: u64) {
        self.elements.insert(0, value);
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
        if idx == 0 || idx > self.elements.len() {
            None
        } else {
            self.elements.get(idx - 1).copied()
        }
    }

    pub fn remove(&mut self, idx: usize) -> Option<u64> {
        if idx == 0 || idx > self.elements.len() {
            None
        } else {
            Some(self.elements.remove(idx - 1))
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Default for Storage_SeqOfRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_append() {
        let mut seq = Storage_SeqOfRoot::new();
        seq.append(10);
        seq.append(20);
        seq.append(30);

        assert_eq!(seq.length(), 3);
        assert_eq!(seq.first(), Some(10));
        assert_eq!(seq.last(), Some(30));
    }

    #[test]
    fn test_seq_prepend() {
        let mut seq = Storage_SeqOfRoot::new();
        seq.append(20);
        seq.prepend(10);

        assert_eq!(seq.first(), Some(10));
        assert_eq!(seq.value_at(2), Some(20));
    }

    #[test]
    fn test_seq_value_at() {
        let mut seq = Storage_SeqOfRoot::new();
        seq.append(100);
        seq.append(200);
        seq.append(300);

        assert_eq!(seq.value_at(1), Some(100));
        assert_eq!(seq.value_at(2), Some(200));
        assert_eq!(seq.value_at(3), Some(300));
        assert_eq!(seq.value_at(4), None);
        assert_eq!(seq.value_at(0), None);
    }

    #[test]
    fn test_seq_remove() {
        let mut seq = Storage_SeqOfRoot::new();
        seq.append(11);
        seq.append(22);
        seq.append(33);

        let removed = seq.remove(2);
        assert_eq!(removed, Some(22));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value_at(2), Some(33));
    }

    #[test]
    fn test_seq_clear() {
        let mut seq = Storage_SeqOfRoot::new();
        seq.append(1);
        seq.append(2);
        assert!(!seq.is_empty());

        seq.clear();
        assert!(seq.is_empty());
        assert_eq!(seq.length(), 0);
    }
}
