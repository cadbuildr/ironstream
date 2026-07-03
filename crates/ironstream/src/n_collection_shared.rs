// FILE: n_collection_shared.rs
// occt: NCollection_Shared

/// Base class for shared (reference-counted) objects.
pub struct Shared<T> {
    data: std::sync::Arc<T>,
}

impl<T> Shared<T> {
    /// Create new shared object.
    pub fn new(val: T) -> Self {
        Shared {
            data: std::sync::Arc::new(val),
        }
    }

    /// Get reference to data.
    pub fn as_ref(&self) -> &T {
        &self.data
    }

    /// Clone reference count.
    pub fn clone_shared(&self) -> Self {
        Shared {
            data: std::sync::Arc::clone(&self.data),
        }
    }

    /// Check if this is unique reference.
    pub fn is_unique(&self) -> bool {
        std::sync::Arc::strong_count(&self.data) == 1
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        self.clone_shared()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shared() {
        let s = Shared::new(42);
        assert_eq!(*s.as_ref(), 42);
    }

    #[test]
    fn test_unique() {
        let s = Shared::new("hello");
        assert!(s.is_unique());
        let _s2 = s.clone();
        assert!(!s.is_unique());
    }

    #[test]
    fn test_clone_multiple() {
        let s1 = Shared::new(vec![1, 2, 3]);
        let s2 = s1.clone();
        let s3 = s2.clone();
        assert_eq!(s1.as_ref(), s2.as_ref());
        assert_eq!(s2.as_ref(), s3.as_ref());
    }
}
