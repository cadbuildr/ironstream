// FILE: standard_define_handle.rs
// occt: Standard_DefineHandle

//! Smart pointer handle for reference-counted objects.

use std::sync::Arc;

/// Handle wrapper for reference-counted objects
pub struct Handle<T> {
    inner: Arc<T>,
}

impl<T> Handle<T> {
    /// Creates a new handle from a value
    pub fn new(value: T) -> Self {
        Handle {
            inner: Arc::new(value),
        }
    }

    /// Gets a reference to the value
    pub fn as_ref(&self) -> &T {
        &self.inner
    }

    /// Gets the reference count
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObject {
        value: i32,
    }

    #[test]
    fn test_handle_new() {
        let obj = TestObject { value: 42 };
        let handle = Handle::new(obj);
        assert_eq!(handle.as_ref().value, 42);
    }

    #[test]
    fn test_handle_clone() {
        let handle1 = Handle::new(TestObject { value: 42 });
        let handle2 = handle1.clone();
        assert_eq!(handle1.reference_count(), 2);
        assert_eq!(handle2.reference_count(), 2);
    }
}
