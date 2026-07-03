// FILE: n_collection_base_allocator.rs
// occt: NCollection_BaseAllocator

/// Base allocator interface for memory management.
pub trait BaseAllocator {
    fn allocate(&mut self, size: usize) -> *mut u8;
    fn deallocate(&mut self, ptr: *mut u8, size: usize);
}

/// Standard allocator implementation.
pub struct StandardAllocator;

impl BaseAllocator for StandardAllocator {
    fn allocate(&mut self, size: usize) -> *mut u8 {
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size, 8);
            std::alloc::alloc(layout)
        }
    }

    fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size, 8);
            std::alloc::dealloc(ptr, layout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_allocator() {
        let mut alloc = StandardAllocator;
        let ptr = alloc.allocate(100);
        assert!(!ptr.is_null());
        alloc.deallocate(ptr, 100);
    }
}
