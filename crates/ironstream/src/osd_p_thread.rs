// FILE: osd_p_thread.rs
// occt: OSD_PThread

//! Platform-dependent definition of the thread handle type.
//! This is a wrapper around OS-specific thread handles.

/// Platform-dependent thread handle.
/// On Windows: HANDLE
/// On POSIX: pthread_t
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OsdPThread {
    id: u64,
}

impl OsdPThread {
    /// Creates a null thread handle
    pub const fn null() -> Self {
        OsdPThread { id: 0 }
    }

    /// Creates a thread handle from a numeric ID
    pub fn from_id(id: u64) -> Self {
        OsdPThread { id }
    }

    /// Gets the numeric ID of the thread
    pub fn id(self) -> u64 {
        self.id
    }

    /// Checks if the thread handle is null
    pub fn is_null(self) -> bool {
        self.id == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_pthread_null() {
        let t = OsdPThread::null();
        assert!(t.is_null());
        assert_eq!(t.id(), 0);
    }

    #[test]
    fn test_osd_pthread_from_id() {
        let t = OsdPThread::from_id(42);
        assert!(!t.is_null());
        assert_eq!(t.id(), 42);
    }

    #[test]
    fn test_osd_pthread_equality() {
        let t1 = OsdPThread::from_id(100);
        let t2 = OsdPThread::from_id(100);
        let t3 = OsdPThread::from_id(200);
        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn test_osd_pthread_copy() {
        let t1 = OsdPThread::from_id(123);
        let t2 = t1;
        assert_eq!(t1, t2);
    }
}
