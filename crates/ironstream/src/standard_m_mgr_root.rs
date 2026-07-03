// FILE: standard_m_mgr_root.rs
// occt: Standard_MMgrRoot

//! Root class for Open CASCADE memory managers.
//! Defines abstract interface functions for memory allocation.

/// Root class for Open CASCADE memory managers.
/// Defines only abstract interface functions.
pub trait StandardMMgrRoot {
    /// Allocate specified number of bytes.
    /// The actually allocated space should be rounded up to double word size (4 bytes).
    fn allocate(&mut self, size: usize) -> *mut u8;

    /// Reallocate previously allocated memory to contain at least size bytes.
    /// In case of success, new pointer is returned.
    /// In case that ptr is null, the function behaves exactly as allocate.
    fn reallocate(&mut self, ptr: *mut u8, size: usize) -> *mut u8;

    /// Free previously allocated memory at specified address.
    fn free(&mut self, ptr: *mut u8);

    /// Purge internally cached unused memory blocks (if any) by releasing them to the OS.
    /// Returns non-zero if some memory has been actually released, zero otherwise.
    ///
    /// If is_destroyed is true, this means that memory manager is not expected to be
    /// used any more; note however that in general case it is still possible to have
    /// calls to that instance of memory manager after this (e.g. to free memory
    /// of static objects in OCC). Thus this option should command the memory manager
    /// to release any cached memory to the system and not cache any more, but still
    /// remain operable.
    fn purge(&mut self, is_destroyed: bool) -> i32 {
        // Default implementation does nothing and returns 0
        0
    }
}

/// A simple implementation of StandardMMgrRoot using the standard allocator.
pub struct DefaultMMgrRoot;

impl StandardMMgrRoot for DefaultMMgrRoot {
    fn allocate(&mut self, size: usize) -> *mut u8 {
        if size == 0 {
            return std::ptr::null_mut();
        }
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size, 4);
            std::alloc::alloc(layout)
        }
    }

    fn reallocate(&mut self, ptr: *mut u8, size: usize) -> *mut u8 {
        if size == 0 {
            if !ptr.is_null() {
                self.free(ptr);
            }
            return std::ptr::null_mut();
        }
        if ptr.is_null() {
            return self.allocate(size);
        }
        unsafe {
            let old_layout = std::alloc::Layout::from_size_align_unchecked(size, 4);
            let new_layout = std::alloc::Layout::from_size_align_unchecked(size, 4);
            std::alloc::realloc(ptr, old_layout, new_layout.size())
        }
    }

    fn free(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(1, 4);
                std::alloc::dealloc(ptr, layout);
            }
        }
    }

    fn purge(&mut self, _is_destroyed: bool) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let mut mgr = DefaultMMgrRoot;
        let ptr = mgr.allocate(64);
        assert!(!ptr.is_null());
        mgr.free(ptr);
    }

    #[test]
    fn test_allocate_zero() {
        let mut mgr = DefaultMMgrRoot;
        let ptr = mgr.allocate(0);
        assert!(ptr.is_null());
    }

    #[test]
    fn test_reallocate() {
        let mut mgr = DefaultMMgrRoot;
        let ptr1 = mgr.allocate(32);
        assert!(!ptr1.is_null());

        let ptr2 = mgr.reallocate(ptr1, 64);
        assert!(!ptr2.is_null());
        mgr.free(ptr2);
    }

    #[test]
    fn test_purge() {
        let mut mgr = DefaultMMgrRoot;
        let result = mgr.purge(false);
        assert_eq!(result, 0);
    }
}
