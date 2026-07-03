// FILE: standard_define_alloc.rs
// occt: Standard_DefineAlloc

//! Macro for defining custom memory allocation.
//! In Rust, this is handled automatically by the allocator system.

/// Trait for objects that can be allocated with custom allocator
pub trait AllocatedObject {
    /// Allocates memory for this type
    fn allocate() -> *mut Self;

    /// Deallocates memory
    unsafe fn deallocate(ptr: *mut Self);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObject {
        value: i32,
    }

    impl AllocatedObject for TestObject {
        fn allocate() -> *mut Self {
            Box::into_raw(Box::new(TestObject { value: 0 }))
        }

        unsafe fn deallocate(ptr: *mut Self) {
            if !ptr.is_null() {
                let _ = Box::from_raw(ptr);
            }
        }
    }

    #[test]
    fn test_allocated_object() {
        let ptr = TestObject::allocate();
        assert!(!ptr.is_null());
        unsafe {
            TestObject::deallocate(ptr);
        }
    }
}
