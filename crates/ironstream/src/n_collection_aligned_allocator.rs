// FILE: n_collection_aligned_allocator.rs
// occt: NCollection_AlignedAllocator

/// Allocator with memory alignment support.
pub struct NCollectionAlignedAllocator {
    alignment: usize,
    total: usize,
}

impl NCollectionAlignedAllocator {
    pub fn new(alignment: usize) -> Self {
        Self {
            alignment: if alignment > 0 { alignment } else { 1 },
            total: 0,
        }
    }

    pub fn alignment(&self) -> usize {
        self.alignment
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        let aligned_size = (size + self.alignment - 1) / self.alignment * self.alignment;
        let address = self.total;
        self.total += aligned_size;
        Some(address)
    }

    pub fn total_allocated(&self) -> usize {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let alloc = NCollectionAlignedAllocator::new(16);
        assert_eq!(alloc.alignment(), 16);
    }

    #[test]
    fn test_allocate() {
        let mut alloc = NCollectionAlignedAllocator::new(8);
        let addr1 = alloc.allocate(5).unwrap();
        let addr2 = alloc.allocate(5).unwrap();
        assert_eq!(addr1, 0);
        assert_eq!(addr2, 8); // Aligned to 8
    }
}
