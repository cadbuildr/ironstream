// FILE: if_select_check_counter.rs
// occt: IFSelect_CheckCounter

/// Counts and tracks check results
#[derive(Clone, Debug)]
pub struct IfSelectCheckCounter {
    count: usize,
    failures: usize,
}

impl IfSelectCheckCounter {
    /// Creates a check counter
    pub fn new() -> Self {
        IfSelectCheckCounter {
            count: 0,
            failures: 0,
        }
    }

    /// Increments the count
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Increments the failure count
    pub fn increment_failures(&mut self) {
        self.failures += 1;
    }

    /// Returns the check count
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the failure count
    pub fn failures(&self) -> usize {
        self.failures
    }

    /// Resets counters
    pub fn reset(&mut self) {
        self.count = 0;
        self.failures = 0;
    }
}

impl Default for IfSelectCheckCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let counter = IfSelectCheckCounter::new();
        assert_eq!(counter.count(), 0);
        assert_eq!(counter.failures(), 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = IfSelectCheckCounter::new();
        counter.increment();
        assert_eq!(counter.count(), 1);
    }

    #[test]
    fn test_increment_failures() {
        let mut counter = IfSelectCheckCounter::new();
        counter.increment_failures();
        assert_eq!(counter.failures(), 1);
    }

    #[test]
    fn test_reset() {
        let mut counter = IfSelectCheckCounter::new();
        counter.increment();
        counter.increment_failures();
        counter.reset();
        assert_eq!(counter.count(), 0);
        assert_eq!(counter.failures(), 0);
    }
}
