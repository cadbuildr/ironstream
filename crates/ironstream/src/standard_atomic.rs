// FILE: standard_atomic.rs
// occt: Standard_Atomic

use std::sync::atomic::{AtomicI32, Ordering};

/// Increments atomically an integer variable and returns resulting incremented value.
/// Deprecated in OCCT 8.0.0; use std::sync::atomic instead.
pub fn standard_atomic_increment(value: &AtomicI32) -> i32 {
    value.fetch_add(1, Ordering::SeqCst) + 1
}

/// Decrements atomically an integer variable and returns resulting decremented value.
/// Deprecated in OCCT 8.0.0; use std::sync::atomic instead.
pub fn standard_atomic_decrement(value: &AtomicI32) -> i32 {
    value.fetch_sub(1, Ordering::SeqCst) - 1
}

/// Perform an atomic compare and swap.
/// If the current value of *value is old_value, write new_value into *value.
/// Returns TRUE if new_value has been set to *value.
/// Deprecated in OCCT 8.0.0; use std::sync::atomic instead.
pub fn standard_atomic_compare_and_swap(
    value: &AtomicI32,
    old_value: i32,
    new_value: i32,
) -> bool {
    value
        .compare_exchange(old_value, new_value, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicI32;

    #[test]
    fn test_increment() {
        let val = AtomicI32::new(5);
        let result = standard_atomic_increment(&val);
        assert_eq!(result, 6);
        assert_eq!(val.load(Ordering::SeqCst), 6);
    }

    #[test]
    fn test_decrement() {
        let val = AtomicI32::new(5);
        let result = standard_atomic_decrement(&val);
        assert_eq!(result, 4);
        assert_eq!(val.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn test_compare_and_swap_success() {
        let val = AtomicI32::new(5);
        let result = standard_atomic_compare_and_swap(&val, 5, 10);
        assert!(result);
        assert_eq!(val.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_compare_and_swap_failure() {
        let val = AtomicI32::new(5);
        let result = standard_atomic_compare_and_swap(&val, 3, 10);
        assert!(!result);
        assert_eq!(val.load(Ordering::SeqCst), 5);
    }
}
