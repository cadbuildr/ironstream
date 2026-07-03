// FILE: n_collection_ub_tree_filler.rs
// occt: NCollection_UBTreeFiller

/// Utility for efficiently filling unbalanced binary trees.
pub struct UBTreeFiller {
    capacity: usize,
    count: usize,
}

impl UBTreeFiller {
    /// Create tree filler with given capacity.
    pub fn new(capacity: usize) -> Self {
        UBTreeFiller {
            capacity,
            count: 0,
        }
    }

    /// Get capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Add element (increment count).
    pub fn add(&mut self) -> bool {
        if self.count < self.capacity {
            self.count += 1;
            true
        } else {
            false
        }
    }

    /// Get current count.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Check if full.
    pub fn is_full(&self) -> bool {
        self.count >= self.capacity
    }

    /// Reset count.
    pub fn reset(&mut self) {
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filler() {
        let f = UBTreeFiller::new(10);
        assert_eq!(f.capacity(), 10);
        assert_eq!(f.count(), 0);
        assert!(!f.is_full());
    }

    #[test]
    fn test_add_elements() {
        let mut f = UBTreeFiller::new(3);
        assert!(f.add());
        assert!(f.add());
        assert!(f.add());
        assert!(!f.add());
        assert!(f.is_full());
    }

    #[test]
    fn test_reset() {
        let mut f = UBTreeFiller::new(5);
        f.add();
        f.add();
        assert_eq!(f.count(), 2);
        f.reset();
        assert_eq!(f.count(), 0);
    }
}
