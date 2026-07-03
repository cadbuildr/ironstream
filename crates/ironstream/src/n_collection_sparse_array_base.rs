// FILE: n_collection_sparse_array_base.rs
// occt: NCollection_SparseArrayBase

/// Base sparse array implementation.
/// Sparse arrays use hash maps to store only non-zero or non-default elements.
pub struct SparseArrayBase {
    size: usize,
}

impl SparseArrayBase {
    /// Create sparse array with given size.
    pub fn new(size: usize) -> Self {
        SparseArrayBase { size }
    }

    /// Get declared size.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Set size.
    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    /// Lower index bound (always 0 in Rust).
    pub const fn lower_bound() -> usize {
        0
    }

    /// Upper index bound.
    pub fn upper_bound(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sparse_array() {
        let arr = SparseArrayBase::new(100);
        assert_eq!(arr.size(), 100);
    }

    #[test]
    fn test_bounds() {
        let arr = SparseArrayBase::new(50);
        assert_eq!(SparseArrayBase::lower_bound(), 0);
        assert_eq!(arr.upper_bound(), 50);
    }

    #[test]
    fn test_resize() {
        let mut arr = SparseArrayBase::new(10);
        arr.set_size(20);
        assert_eq!(arr.size(), 20);
    }
}
