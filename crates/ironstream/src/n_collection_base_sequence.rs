// FILE: n_collection_base_sequence.rs
// occt: NCollection_BaseSequence

/// Base sequence container.
pub struct NCollectionBaseSequence {
    size: usize,
}

impl NCollectionBaseSequence {
    pub fn new() -> Self {
        Self { size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }
}

impl Default for NCollectionBaseSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let seq = NCollectionBaseSequence::new();
        assert!(seq.is_empty());
        assert_eq!(seq.size(), 0);
    }
}
