// FILE: n_collection_win_heap_allocator.rs
// occt: NCollection_WinHeapAllocator

/// Memory allocator using heap operations (cross-platform adaptation of Windows heap API).
pub struct WinHeapAllocator {
    name: String,
}

impl WinHeapAllocator {
    /// Create named allocator.
    pub fn new(name: &str) -> Self {
        WinHeapAllocator {
            name: name.to_string(),
        }
    }

    /// Get allocator name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Allocate memory block.
    pub fn allocate(&self, size: usize) -> *mut u8 {
        if size == 0 {
            std::ptr::null_mut()
        } else {
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(size, 1);
                std::alloc::alloc(layout)
            }
        }
    }

    /// Deallocate memory block.
    pub fn deallocate(&self, ptr: *mut u8, size: usize) {
        if size > 0 && !ptr.is_null() {
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(size, 1);
                std::alloc::dealloc(ptr, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_allocator() {
        let alloc = WinHeapAllocator::new("test");
        assert_eq!(alloc.name(), "test");
    }

    #[test]
    fn test_allocate_deallocate() {
        let alloc = WinHeapAllocator::new("test");
        let ptr = alloc.allocate(100);
        assert!(!ptr.is_null());
        alloc.deallocate(ptr, 100);
    }

    #[test]
    fn test_allocate_zero() {
        let alloc = WinHeapAllocator::new("test");
        let ptr = alloc.allocate(0);
        assert!(ptr.is_null());
    }
}
