// FILE: n_collection_occ_allocator.rs
// occt: NCollection_OccAllocator

/// OCCT allocator for standard heap memory.
pub struct OccAllocator {
    total_allocated: usize,
}

impl OccAllocator {
    pub fn new() -> Self {
        OccAllocator {
            total_allocated: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size, 8);
            self.total_allocated += size;
            std::alloc::alloc(layout)
        }
    }

    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size, 8);
            self.total_allocated -= size;
            std::alloc::dealloc(ptr, layout)
        }
    }

    pub fn total_allocated(&self) -> usize {
        self.total_allocated
    }
}

impl Default for OccAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occ_allocator() {
        let mut alloc = OccAllocator::new();
        let ptr = alloc.allocate(256);
        assert!(!ptr.is_null());
        assert!(alloc.total_allocated() >= 256);
        alloc.deallocate(ptr, 256);
    }
}
