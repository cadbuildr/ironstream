// FILE: std_drivers.rs
// occt: StdDrivers

/// Drivers for document storage and retrieval
pub struct StdDrivers;

impl StdDrivers {
    /// Create drivers manager
    pub fn new() -> Self {
        StdDrivers
    }

    /// Bind drivers to a factory
    pub fn bind_drivers() {
        // Register standard drivers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = StdDrivers::new();
    }

    #[test]
    fn test_bind_drivers() {
        StdDrivers::bind_drivers();
    }
}
