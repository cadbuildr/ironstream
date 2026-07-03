// FILE: n_collection_heap_allocator.rs
// occt: NCollection_HeapAllocator

/// Heap allocator for dynamic memory allocation.
pub struct NCollectionHeapAllocator {
    total: usize,
    block_count: usize,
}

impl NCollectionHeapAllocator {
    pub fn new() -> Self {
        Self {
            total: 0,
            block_count: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> usize {
        let address = self.total;
        self.total += size;
        self.block_count += 1;
        address
    }

    pub fn total_allocated(&self) -> usize {
        self.total
    }

    pub fn block_count(&self) -> usize {
        self.block_count
    }
}

impl Default for NCollectionHeapAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let alloc = NCollectionHeapAllocator::new();
        assert_eq!(alloc.total_allocated(), 0);
        assert_eq!(alloc.block_count(), 0);
    }

    #[test]
    fn test_allocate() {
        let mut alloc = NCollectionHeapAllocator::new();
        let addr1 = alloc.allocate(100);
        let addr2 = alloc.allocate(200);
        assert_eq!(addr1, 0);
        assert_eq!(addr2, 100);
        assert_eq!(alloc.block_count(), 2);
    }
}
