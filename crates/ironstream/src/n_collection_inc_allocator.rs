// FILE: n_collection_inc_allocator.rs
// occt: NCollection_IncAllocator

/// Incremental allocator for efficient memory management.
pub struct NCollectionIncAllocator {
    block_size: usize,
    total: usize,
}

impl NCollectionIncAllocator {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size: if block_size > 0 { block_size } else { 256 },
            total: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> usize {
        let address = self.total;
        self.total += size;
        address
    }

    pub fn reset(&mut self) {
        self.total = 0;
    }

    pub fn total_allocated(&self) -> usize {
        self.total
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }
}

impl Default for NCollectionIncAllocator {
    fn default() -> Self {
        Self::new(256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let alloc = NCollectionIncAllocator::new(512);
        assert_eq!(alloc.block_size(), 512);
        assert_eq!(alloc.total_allocated(), 0);
    }

    #[test]
    fn test_allocate_and_reset() {
        let mut alloc = NCollectionIncAllocator::new(256);
        alloc.allocate(100);
        alloc.allocate(50);
        assert_eq!(alloc.total_allocated(), 150);
        alloc.reset();
        assert_eq!(alloc.total_allocated(), 0);
    }
}
