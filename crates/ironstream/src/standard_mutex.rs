// FILE: standard_mutex.rs
// occt: Standard_Mutex

//! Mutex for thread-safe synchronization.
//! In IronStream, we provide a simple sequential mutex equivalent
//! that doesn't actually spawn OS threads (per the porting constraints).

use std::sync::{Arc, Mutex as StdMutex};
use std::cell::RefCell;

/// A simple mutex-like synchronization primitive.
/// In this sequential Rust port, we use RefCell instead of actual thread synchronization.
pub struct StandardMutex {
    _marker: std::marker::PhantomData<()>,
}

impl StandardMutex {
    /// Create a new mutex.
    pub fn new() -> Self {
        StandardMutex {
            _marker: std::marker::PhantomData,
        }
    }

    /// Lock the mutex (no-op in sequential mode).
    pub fn lock(&self) {
        // No-op in sequential mode
    }

    /// Unlock the mutex (no-op in sequential mode).
    pub fn unlock(&self) {
        // No-op in sequential mode
    }

    /// Attempt to lock the mutex without blocking.
    pub fn try_lock(&self) -> bool {
        true // Always succeeds in sequential mode
    }
}

impl Default for StandardMutex {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-local mutex wrapper for sequential execution.
pub struct ThreadLocalMutex<T> {
    data: RefCell<T>,
}

impl<T> ThreadLocalMutex<T> {
    pub fn new(data: T) -> Self {
        ThreadLocalMutex {
            data: RefCell::new(data),
        }
    }

    pub fn lock(&self) -> std::cell::Ref<T> {
        self.data.borrow()
    }

    pub fn lock_mut(&self) -> std::cell::RefMut<T> {
        self.data.borrow_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_creation() {
        let _mutex = StandardMutex::new();
    }

    #[test]
    fn test_mutex_lock() {
        let mutex = StandardMutex::new();
        mutex.lock();
        mutex.unlock();
    }

    #[test]
    fn test_mutex_try_lock() {
        let mutex = StandardMutex::new();
        assert!(mutex.try_lock());
    }

    #[test]
    fn test_thread_local_mutex() {
        let mutex = ThreadLocalMutex::new(42);
        {
            let val = mutex.lock();
            assert_eq!(*val, 42);
        }
        {
            let mut val = mutex.lock_mut();
            *val = 100;
        }
        let val = mutex.lock();
        assert_eq!(*val, 100);
    }

    #[test]
    fn test_default_mutex() {
        let _mutex = StandardMutex::default();
    }
}
