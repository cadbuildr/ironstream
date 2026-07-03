// FILE: n_collection_handle.rs
// occt: NCollection_Handle

use std::sync::Arc;

/// Handle to object with reference counting.
pub struct NCollectionHandle<T: ?Sized> {
    ptr: Arc<T>,
}

impl<T> NCollectionHandle<T> {
    pub fn new(obj: T) -> Self {
        Self { ptr: Arc::new(obj) }
    }

    pub fn from_arc(arc: Arc<T>) -> Self {
        Self { ptr: arc }
    }

    pub fn get(&self) -> &T {
        &self.ptr
    }

    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.ptr)
    }
}

impl<T> Clone for NCollectionHandle<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: Arc::clone(&self.ptr),
        }
    }
}

impl<T> std::ops::Deref for NCollectionHandle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ptr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let h = NCollectionHandle::new(42i32);
        assert_eq!(*h, 42);
    }

    #[test]
    fn test_clone() {
        let h1 = NCollectionHandle::new(vec![1, 2, 3]);
        let h2 = h1.clone();
        assert_eq!(h1.ref_count(), 2);
        assert_eq!(h2.ref_count(), 2);
    }
}
