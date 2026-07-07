// FILE: if_select_sign_counter.rs
// occt: IFSelect_SignCounter

/// Counts and manages signatures for entities.
#[derive(Clone, Debug)]
pub struct IFSelectSignCounter {
    name: String,
    count: usize,
}

impl IFSelectSignCounter {
    /// Creates a SignCounter
    pub fn new(name: String) -> Self {
        Self { name, count: 0 }
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the current count
    pub fn count(&self) -> usize {
        self.count
    }

    /// Increments the count
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Resets the count
    pub fn reset(&mut self) {
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let counter = IFSelectSignCounter::new("test".to_string());
        assert_eq!(counter.name(), "test");
        assert_eq!(counter.count(), 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = IFSelectSignCounter::new("count".to_string());
        counter.increment();
        assert_eq!(counter.count(), 1);
        counter.increment();
        counter.increment();
        assert_eq!(counter.count(), 3);
    }

    #[test]
    fn test_reset() {
        let mut counter = IFSelectSignCounter::new("count".to_string());
        counter.increment();
        counter.increment();
        assert_eq!(counter.count(), 2);
        counter.reset();
        assert_eq!(counter.count(), 0);
    }
}
