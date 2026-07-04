// FILE: std_l_persistent_function.rs
// occt: StdLPersistent_Function

/// Persistent function attribute
pub struct StdLPersistentFunction {
    driver_guid: String,
    failure: i32,
}

impl StdLPersistentFunction {
    /// Create empty function
    pub fn new() -> Self {
        StdLPersistentFunction {
            driver_guid: String::new(),
            failure: 0,
        }
    }

    /// Get the driver GUID
    pub fn driver_guid(&self) -> &str {
        &self.driver_guid
    }

    /// Set the driver GUID
    pub fn set_driver_guid(&mut self, guid: &str) {
        self.driver_guid = guid.to_string();
    }

    /// Get the failure code
    pub fn failure(&self) -> i32 {
        self.failure
    }

    /// Set the failure code
    pub fn set_failure(&mut self, code: i32) {
        self.failure = code;
    }
}

impl Default for StdLPersistentFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let func = StdLPersistentFunction::new();
        assert_eq!(func.driver_guid(), "");
        assert_eq!(func.failure(), 0);
    }

    #[test]
    fn test_set_driver_guid() {
        let mut func = StdLPersistentFunction::new();
        func.set_driver_guid("test-guid");
        assert_eq!(func.driver_guid(), "test-guid");
    }

    #[test]
    fn test_set_failure() {
        let mut func = StdLPersistentFunction::new();
        func.set_failure(1);
        assert_eq!(func.failure(), 1);
    }
}
