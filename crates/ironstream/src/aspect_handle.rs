// FILE: aspect_handle.rs
// occt: Aspect_Handle

/// Generic handle wrapper for reference-counted objects.
pub struct AspectHandle<T> {
    ptr: *mut T,
}

impl<T> AspectHandle<T> {
    /// Create null handle.
    pub fn null() -> Self {
        AspectHandle {
            ptr: std::ptr::null_mut(),
        }
    }

    /// Create handle from raw pointer.
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        AspectHandle { ptr }
    }

    /// Get raw pointer.
    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Check if null.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let handle: AspectHandle<i32> = AspectHandle::null();
        assert!(handle.is_null());
    }
}
