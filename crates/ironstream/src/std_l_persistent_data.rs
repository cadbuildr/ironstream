// FILE: std_l_persistent_data.rs
// occt: StdLPersistent_Data

/// Persistent data for TDF_Data
pub struct StdLPersistentData {
    version: i32,
}

impl StdLPersistentData {
    /// Create empty persistent data
    pub fn new() -> Self {
        StdLPersistentData { version: 0 }
    }

    /// Get the version
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Set the version
    pub fn set_version(&mut self, v: i32) {
        self.version = v;
    }
}

impl Default for StdLPersistentData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let data = StdLPersistentData::new();
        assert_eq!(data.version(), 0);
    }

    #[test]
    fn test_set_version() {
        let mut data = StdLPersistentData::new();
        data.set_version(1);
        assert_eq!(data.version(), 1);
    }
}
