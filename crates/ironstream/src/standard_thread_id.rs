// FILE: standard_thread_id.rs
// occt: Standard_ThreadId

//! Platform-independent definition of the thread identifier type.

/// Platform-independent thread identifier.
/// Rust equivalent of Standard_ThreadId (size_t).
pub type StandardThreadId = usize;

/// Get the current thread ID.
pub fn current_thread_id() -> StandardThreadId {
    // In sequential mode, always return 0
    0
}

/// Create a StandardThreadId from a usize value.
pub const fn thread_id_from_usize(id: usize) -> StandardThreadId {
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_thread_id() {
        let id = current_thread_id();
        assert_eq!(id, 0);
    }

    #[test]
    fn test_thread_id_from_usize() {
        let id = thread_id_from_usize(42);
        assert_eq!(id, 42);
    }

    #[test]
    fn test_thread_id_comparison() {
        let id1 = thread_id_from_usize(1);
        let id2 = thread_id_from_usize(1);
        let id3 = thread_id_from_usize(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_thread_id_type() {
        let id: StandardThreadId = 123;
        assert_eq!(id, 123usize);
    }
}
