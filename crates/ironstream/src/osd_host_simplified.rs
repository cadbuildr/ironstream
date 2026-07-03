// FILE: osd_host_simplified.rs
// Simplified Host implementation without external deps

/// Host/machine information (simplified version without external dependencies).
pub struct Host;

impl Host {
    /// Get host name from environment.
    pub fn name() -> String {
        std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| "unknown".to_string())
    }

    /// Get number of processors (stub - returns 1).
    pub fn processor_count() -> usize {
        // Stub implementation
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_count() {
        let count = Host::processor_count();
        assert!(count > 0);
    }
}
