// FILE: storage.rs
// occt: Storage

//! Storage package for writing and reading persistent objects.
//! Objects are read/written by a retrieval or storage algorithm (Storage_Schema object)
//! in a container (disk, memory, network, etc.).

/// Storage version information.
pub struct Storage;

impl Storage {
    /// Returns the version of Storage's read/write routines.
    pub fn version() -> &'static str {
        "1.0"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_version() {
        let version = Storage::version();
        assert!(!version.is_empty());
        assert_eq!(version, "1.0");
    }
}
