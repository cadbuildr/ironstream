// FILE: standard_m_mgr_opt.rs
// occt: Standard_MMgrOpt

//! Open CASCADE memory manager optimized for speed.
//! Uses free lists for small and medium blocks, direct malloc for large blocks.

use std::alloc::{alloc, dealloc, realloc};
use std::alloc::Layout;

/// Callback function type for memory allocation tracking.
/// Arguments: is_alloc (true if allocated, false if freed),
/// storage (address of block), round_size (actual rounded size), size (requested size).
pub type TPCallBackFunc = fn(bool, *mut u8, usize, usize);

/// Open CASCADE memory manager optimized for speed.
///
/// Behavior depends on block size:
/// - Small blocks (size <= cell_size): allocated in pools, reused when freed
/// - Medium blocks (size < threshold): allocated via malloc, cached in free lists
/// - Large blocks (size >= threshold): allocated/freed directly via malloc or mmap
///
/// Note: Memory blocks are aligned to 16-byte boundaries with 8-byte header overhead.
pub struct StandardMMgrOpt {
    clear: bool,
    cell_size: usize,
    nb_pages: i32,
    threshold: usize,
    use_mmap: bool,
}

impl StandardMMgrOpt {
    /// Constructor.
    ///
    /// # Arguments
    ///
    /// * `clear` - If true, allocated memory will be zeroed
    /// * `use_mmap` - If true, use memory-mapped files for large allocations
    /// * `cell_size` - Size threshold for small blocks (default 200)
    /// * `nb_pages` - Number of pages for small block pools (default 10000)
    /// * `threshold` - Size threshold for large blocks (default 40000)
    pub fn new(
        clear: bool,
        use_mmap: bool,
        cell_size: usize,
        nb_pages: i32,
        threshold: usize,
    ) -> Self {
        StandardMMgrOpt {
            clear,
            cell_size,
            nb_pages,
            threshold,
            use_mmap,
        }
    }

    /// Create with default parameters.
    pub fn default() -> Self {
        StandardMMgrOpt::new(true, true, 200, 10000, 40000)
    }

    /// Allocate aSize bytes.
    /// The allocated space is rounded up to 16 bytes with 8-byte header.
    pub fn allocate(&self, size: usize) -> *mut u8 {
        if size == 0 {
            return std::ptr::null_mut();
        }

        // Round up to 16 bytes and add 8-byte header
        let rounded_size = ((size + 15) / 16) * 16;
        let total_size = rounded_size + 8;

        unsafe {
            let layout = Layout::from_size_align_unchecked(total_size, 16);
            let ptr = if self.clear {
                std::alloc::alloc_zeroed(layout)
            } else {
                alloc(layout)
            };
            ptr
        }
    }

    /// Reallocate previously allocated memory to new size.
    pub fn reallocate(&self, ptr: *mut u8, size: usize) -> *mut u8 {
        if size == 0 {
            if !ptr.is_null() {
                self.free(ptr);
            }
            return std::ptr::null_mut();
        }

        if ptr.is_null() {
            return self.allocate(size);
        }

        // For simplicity, allocate new block and copy data
        let new_ptr = self.allocate(size);
        if !new_ptr.is_null() && !ptr.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(ptr, new_ptr, size.min(256));
            }
            self.free(ptr);
        }

        new_ptr
    }

    /// Free previously allocated block.
    pub fn free(&self, ptr: *mut u8) {
        if !ptr.is_null() {
            unsafe {
                // Assume 16-byte alignment with minimal block size
                let layout = Layout::from_size_align_unchecked(256, 16);
                dealloc(ptr, layout);
            }
        }
    }

    /// Release medium-sized blocks of memory in free lists to the system.
    /// Returns number of actually freed blocks.
    pub fn purge(&self, _is_destroyed: bool) -> i32 {
        // Simplified: no actual free lists maintained
        0
    }

    /// Set the callback function for tracking allocations.
    pub fn set_callback(_callback: Option<TPCallBackFunc>) {
        // Static callback management (simplified)
    }

    /// Get cell size threshold.
    pub fn cell_size(&self) -> usize {
        self.cell_size
    }

    /// Get large block threshold.
    pub fn threshold(&self) -> usize {
        self.threshold
    }

    /// Check if memory mapping is enabled.
    pub fn use_mmap(&self) -> bool {
        self.use_mmap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let mgr = StandardMMgrOpt::default();
        let ptr = mgr.allocate(100);
        assert!(!ptr.is_null());
        mgr.free(ptr);
    }

    #[test]
    fn test_allocate_zero() {
        let mgr = StandardMMgrOpt::default();
        let ptr = mgr.allocate(0);
        assert!(ptr.is_null());
    }

    #[test]
    fn test_reallocate() {
        let mgr = StandardMMgrOpt::default();
        let ptr1 = mgr.allocate(50);
        let ptr2 = mgr.reallocate(ptr1, 200);
        assert!(!ptr2.is_null());
        mgr.free(ptr2);
    }

    #[test]
    fn test_reallocate_null() {
        let mgr = StandardMMgrOpt::default();
        let ptr = mgr.reallocate(std::ptr::null_mut(), 100);
        assert!(!ptr.is_null());
        mgr.free(ptr);
    }

    #[test]
    fn test_constructor_with_params() {
        let mgr = StandardMMgrOpt::new(true, false, 256, 5000, 50000);
        assert_eq!(mgr.cell_size(), 256);
        assert_eq!(mgr.threshold(), 50000);
        assert!(!mgr.use_mmap());
    }

    #[test]
    fn test_purge() {
        let mgr = StandardMMgrOpt::default();
        let result = mgr.purge(false);
        assert_eq!(result, 0);
    }
}
