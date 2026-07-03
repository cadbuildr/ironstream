// FILE: standard_memory_utils.rs
// occt: Standard_MemoryUtils

//! Memory utility functions for shared and unique pointer creation.
//! Rust equivalents of std::shared_ptr and std::unique_ptr.

use std::sync::Arc;

/// Create a shared pointer with arguments.
/// Rust equivalent of opencascade::make_shared<T>()
pub fn make_shared<T: 'static>(value: T) -> Arc<T> {
    Arc::new(value)
}

/// Create a unique pointer with arguments.
/// Rust equivalent of opencascade::make_unique<T>()
pub fn make_unique<T: 'static>(value: T) -> Box<T> {
    Box::new(value)
}

/// Create a shared pointer from a closure.
pub fn make_shared_fn<T: 'static, F>(f: F) -> Arc<T>
where
    F: FnOnce() -> T,
{
    Arc::new(f())
}

/// Create a unique pointer from a closure.
pub fn make_unique_fn<T: 'static, F>(f: F) -> Box<T>
where
    F: FnOnce() -> T,
{
    Box::new(f())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_shared() {
        let shared = make_shared(42);
        assert_eq!(*shared, 42);
    }

    #[test]
    fn test_make_shared_string() {
        let shared = make_shared(String::from("test"));
        assert_eq!(*shared, "test");
    }

    #[test]
    fn test_make_unique() {
        let unique = make_unique(42);
        assert_eq!(*unique, 42);
    }

    #[test]
    fn test_make_unique_string() {
        let unique = make_unique(String::from("test"));
        assert_eq!(*unique, "test");
    }

    #[test]
    fn test_make_shared_fn() {
        let shared = make_shared_fn(|| 42);
        assert_eq!(*shared, 42);
    }

    #[test]
    fn test_make_unique_fn() {
        let unique = make_unique_fn(|| String::from("test"));
        assert_eq!(*unique, "test");
    }

    #[test]
    fn test_shared_clone() {
        let shared1 = make_shared(vec![1, 2, 3]);
        let shared2 = Arc::clone(&shared1);
        assert_eq!(*shared1, *shared2);
    }
}
