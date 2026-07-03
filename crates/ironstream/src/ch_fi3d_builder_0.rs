// FILE: ch_fi3d_builder_0.rs
// occt: ChFi3d_Builder_0

/// Base builder for 3D fillets and chamfers (internal/preliminary).
/// This is the foundational class providing core fillet building operations.
#[derive(Debug, Clone)]
pub struct ChFi3dBuilder0 {
    /// Whether the builder is initialized
    is_initialized: bool,
    /// Number of stripes being processed
    stripe_count: usize,
}

impl ChFi3dBuilder0 {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            stripe_count: 0,
        }
    }

    /// Check if builder is initialized
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    /// Initialize the builder
    pub fn initialize(&mut self) {
        self.is_initialized = true;
        self.stripe_count = 0;
    }

    /// Get stripe count
    pub fn stripe_count(&self) -> usize {
        self.stripe_count
    }

    /// Add a stripe
    pub fn add_stripe(&mut self) {
        self.stripe_count += 1;
    }
}

impl Default for ChFi3dBuilder0 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = ChFi3dBuilder0::new();
        assert!(!builder.is_initialized());
        assert_eq!(builder.stripe_count(), 0);
    }

    #[test]
    fn test_initialize() {
        let mut builder = ChFi3dBuilder0::new();
        builder.initialize();
        assert!(builder.is_initialized());
    }

    #[test]
    fn test_add_stripe() {
        let mut builder = ChFi3dBuilder0::new();
        builder.initialize();
        builder.add_stripe();
        builder.add_stripe();
        assert_eq!(builder.stripe_count(), 2);
    }

    #[test]
    fn test_default() {
        let builder = ChFi3dBuilder0::default();
        assert!(!builder.is_initialized());
    }
}
