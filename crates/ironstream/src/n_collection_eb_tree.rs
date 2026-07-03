// FILE: n_collection_eb_tree.rs
// occt: NCollection_EBTree

/// Empty-based tree structure.
pub struct NCollectionEBTree {
    size: usize,
}

impl NCollectionEBTree {
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

impl Default for NCollectionEBTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree = NCollectionEBTree::new();
        assert!(tree.is_empty());
    }
}
