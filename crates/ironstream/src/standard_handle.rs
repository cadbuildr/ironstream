// FILE: standard_handle.rs
// occt: Standard_Handle

use std::sync::Arc;

/// Smart pointer handle for reference-counted objects.
/// Provides automatic memory management through Arc.
pub struct StandardHandle<T: ?Sized> {
    ptr: Arc<T>,
}

impl<T> StandardHandle<T> {
    /// Creates a new handle from an object.
    pub fn new(obj: T) -> Self {
        Self { ptr: Arc::new(obj) }
    }

    /// Creates a handle from an Arc.
    pub fn from_arc(arc: Arc<T>) -> Self {
        Self { ptr: arc }
    }

    /// Returns a reference to the object.
    pub fn get(&self) -> &T {
        &self.ptr
    }

    /// Returns the reference count.
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.ptr)
    }

    /// Converts to an Arc.
    pub fn into_arc(self) -> Arc<T> {
        self.ptr
    }
}

impl<T> Clone for StandardHandle<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: Arc::clone(&self.ptr),
        }
    }
}

impl<T> std::ops::Deref for StandardHandle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ptr
    }
}

impl<T: PartialEq> PartialEq for StandardHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr.eq(&other.ptr)
    }
}

impl<T: Eq> Eq for StandardHandle<T> {}

impl<T> std::fmt::Debug for StandardHandle<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StandardHandle")
            .field("ref_count", &Arc::strong_count(&self.ptr))
            .field("obj", &*self.ptr)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_handle() {
        let h = StandardHandle::new(42i32);
        assert_eq!(*h, 42);
    }

    #[test]
    fn test_clone_handle() {
        let h1 = StandardHandle::new(100i32);
        assert_eq!(h1.ref_count(), 1);

        let h2 = h1.clone();
        assert_eq!(h1.ref_count(), 2);
        assert_eq!(h2.ref_count(), 2);

        assert_eq!(*h1, 100);
        assert_eq!(*h2, 100);
    }

    #[test]
    fn test_deref() {
        let h = StandardHandle::new(String::from("hello"));
        assert_eq!(h.len(), 5);
    }

    #[test]
    fn test_equality() {
        let h1 = StandardHandle::new(42i32);
        let h2 = StandardHandle::new(42i32);
        let h3 = h1.clone();

        assert_eq!(h1, h2);
        assert_eq!(h1, h3);
    }

    #[test]
    fn test_ref_count() {
        let h1 = StandardHandle::new(vec![1, 2, 3]);
        assert_eq!(h1.ref_count(), 1);

        let h2 = h1.clone();
        assert_eq!(h1.ref_count(), 2);
        assert_eq!(h2.ref_count(), 2);

        let h3 = h2.clone();
        assert_eq!(h1.ref_count(), 3);

        drop(h2);
        assert_eq!(h1.ref_count(), 2);
    }
}
