// FILE: osd_lock_type.rs
// occt: OSD_LockType

/// File locking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    /// No lock (default)
    NoLock,
    /// Read lock (only one reader at a time)
    ReadLock,
    /// Write lock (others can read but not write)
    WriteLock,
    /// Exclusive lock (only one user)
    ExclusiveLock,
}

impl LockType {
    /// Get string representation.
    pub fn as_str(&self) -> &str {
        match self {
            LockType::NoLock => "NoLock",
            LockType::ReadLock => "ReadLock",
            LockType::WriteLock => "WriteLock",
            LockType::ExclusiveLock => "ExclusiveLock",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_type_variants() {
        assert_eq!(LockType::NoLock.as_str(), "NoLock");
        assert_eq!(LockType::ReadLock.as_str(), "ReadLock");
        assert_eq!(LockType::WriteLock.as_str(), "WriteLock");
        assert_eq!(LockType::ExclusiveLock.as_str(), "ExclusiveLock");
    }

    #[test]
    fn test_equality() {
        assert_eq!(LockType::NoLock, LockType::NoLock);
        assert_ne!(LockType::ReadLock, LockType::WriteLock);
    }
}
