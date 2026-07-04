// FILE: std_storage.rs
// occt: StdStorage

/// Standard storage management
pub struct StdStorage;

impl StdStorage {
    /// Create a storage manager
    pub fn new() -> Self {
        StdStorage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = StdStorage::new();
    }
}
