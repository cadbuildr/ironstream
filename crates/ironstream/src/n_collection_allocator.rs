// FILE: n_collection_allocator.rs
// occt: NCollection_Allocator

/// Base allocator interface for memory management.
pub trait NCollectionAllocator {
    fn allocate(&mut self, size: usize) -> Option<usize>;
}

/// Standard allocator implementation.
pub struct StandardAllocator {
    total: usize,
}

impl StandardAllocator {
    pub fn new() -> Self {
        Self { total: 0 }
    }

    pub fn total_allocated(&self) -> usize {
        self.total
    }
}

impl Default for StandardAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl NCollectionAllocator for StandardAllocator {
    fn allocate(&mut self, size: usize) -> Option<usize> {
        let addr = self.total;
        self.total += size;
        Some(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_allocator() {
        let mut alloc = StandardAllocator::new();
        let addr1 = alloc.allocate(100);
        let addr2 = alloc.allocate(200);
        assert_eq!(addr1, Some(0));
        assert_eq!(addr2, Some(100));
        assert_eq!(alloc.total_allocated(), 300);
    }
}
