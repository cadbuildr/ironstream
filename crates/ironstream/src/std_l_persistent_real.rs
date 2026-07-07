// FILE: std_l_persistent_real.rs
// occt: StdLPersistent_Real

/// Persistent real number attribute
pub struct StdLPersistentReal {
    value: f64,
    dimension: i32,
}

impl StdLPersistentReal {
    /// Create empty real attribute
    pub fn new() -> Self {
        StdLPersistentReal {
            value: 0.0,
            dimension: 0,
        }
    }

    /// Create real attribute with value
    pub fn with_value(v: f64) -> Self {
        StdLPersistentReal {
            value: v,
            dimension: 0,
        }
    }

    /// Get the value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set the value
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    /// Get the dimension
    pub fn dimension(&self) -> i32 {
        self.dimension
    }

    /// Set the dimension
    pub fn set_dimension(&mut self, dim: i32) {
        self.dimension = dim;
    }
}

impl Default for StdLPersistentReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let real = StdLPersistentReal::new();
        assert_eq!(real.value(), 0.0);
        assert_eq!(real.dimension(), 0);
    }

    #[test]
    fn test_with_value() {
        let real = StdLPersistentReal::with_value(3.14);
        assert_eq!(real.value(), 3.14);
    }

    #[test]
    fn test_set_value() {
        let mut real = StdLPersistentReal::new();
        real.set_value(2.718);
        assert_eq!(real.value(), 2.718);
    }

    #[test]
    fn test_dimension() {
        let mut real = StdLPersistentReal::new();
        real.set_dimension(1);
        assert_eq!(real.dimension(), 1);
    }
}
