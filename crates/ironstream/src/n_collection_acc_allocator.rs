// FILE: n_collection_acc_allocator.rs
// occt: NCollection_AccAllocator

/// Accumulative allocator for efficient memory management.
pub struct NCollectionAccAllocator {
    total_allocated: usize,
    block_size: usize,
}

impl NCollectionAccAllocator {
    pub fn new(block_size: usize) -> Self {
        Self {
            total_allocated: 0,
            block_size: if block_size > 0 { block_size } else { 1024 },
        }
    }

    pub fn total_allocated(&self) -> usize {
        self.total_allocated
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn allocate(&mut self, size: usize) -> usize {
        let address = self.total_allocated;
        self.total_allocated += size;
        address
    }

    pub fn reset(&mut self) {
        self.total_allocated = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let alloc = NCollectionAccAllocator::new(256);
        assert_eq!(alloc.block_size(), 256);
        assert_eq!(alloc.total_allocated(), 0);
    }

    #[test]
    fn test_allocate() {
        let mut alloc = NCollectionAccAllocator::new(256);
        let addr1 = alloc.allocate(100);
        let addr2 = alloc.allocate(200);
        assert_eq!(addr1, 0);
        assert_eq!(addr2, 100);
        assert_eq!(alloc.total_allocated(), 300);
    }

    #[test]
    fn test_reset() {
        let mut alloc = NCollectionAccAllocator::new(256);
        alloc.allocate(100);
        alloc.reset();
        assert_eq!(alloc.total_allocated(), 0);
    }
}
