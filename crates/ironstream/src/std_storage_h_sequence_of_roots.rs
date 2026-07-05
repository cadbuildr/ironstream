// FILE: std_storage_h_sequence_of_roots.rs
// occt: StdStorage_HSequenceOfRoots

/// Handle version of the sequence.
/// Corresponds to opencascade::handle<NCollection_Sequence<opencascade::handle<StdStorage_Root>>>
pub struct StdStorageHSequenceOfRoots {
    data: Vec<Option<String>>,
}

impl StdStorageHSequenceOfRoots {
    pub fn new() -> Self {
        StdStorageHSequenceOfRoots {
            data: vec![None],
        }
    }

    pub fn append(&mut self, value: String) {
        self.data.push(Some(value));
    }

    pub fn prepend(&mut self, value: String) {
        self.data.insert(1, Some(value));
    }

    pub fn value(&self, index: usize) -> Option<&String> {
        if index > 0 && index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn lower(&self) -> usize {
        1
    }

    pub fn upper(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }
}

impl Default for StdStorageHSequenceOfRoots {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut seq = StdStorageHSequenceOfRoots::new();
        seq.append("item1".to_string());
        assert_eq!(seq.len(), 1);
    }
}
