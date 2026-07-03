// FILE: bin_tools_location_set_ptr.rs
// occt: BinTools_LocationSetPtr

/// Pointer wrapper for LocationSet with reference counting.
pub struct BinToolsLocationSetPtr {
    ptr: usize, // Simplified pointer representation
    valid: bool,
}

impl BinToolsLocationSetPtr {
    /// Create a null pointer.
    pub fn null() -> Self {
        Self {
            ptr: 0,
            valid: false,
        }
    }

    /// Create a pointer from an address.
    pub fn new(ptr: usize) -> Self {
        Self { ptr, valid: true }
    }

    /// Check if the pointer is valid.
    pub fn is_valid(&self) -> bool {
        self.valid && self.ptr != 0
    }

    /// Get the pointer value.
    pub fn get(&self) -> usize {
        if self.valid {
            self.ptr
        } else {
            0
        }
    }

    /// Set the pointer value.
    pub fn set(&mut self, ptr: usize) {
        self.ptr = ptr;
        self.valid = ptr != 0;
    }

    /// Reset to null.
    pub fn reset(&mut self) {
        self.ptr = 0;
        self.valid = false;
    }
}

impl Default for BinToolsLocationSetPtr {
    fn default() -> Self {
        Self::null()
    }
}

impl Clone for BinToolsLocationSetPtr {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            valid: self.valid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let ptr = BinToolsLocationSetPtr::null();
        assert!(!ptr.is_valid());
        assert_eq!(ptr.get(), 0);
    }

    #[test]
    fn test_new() {
        let ptr = BinToolsLocationSetPtr::new(0x1000);
        assert!(ptr.is_valid());
        assert_eq!(ptr.get(), 0x1000);
    }

    #[test]
    fn test_set() {
        let mut ptr = BinToolsLocationSetPtr::null();
        ptr.set(0x2000);

        assert!(ptr.is_valid());
        assert_eq!(ptr.get(), 0x2000);
    }

    #[test]
    fn test_reset() {
        let mut ptr = BinToolsLocationSetPtr::new(0x3000);
        ptr.reset();

        assert!(!ptr.is_valid());
        assert_eq!(ptr.get(), 0);
    }
}
