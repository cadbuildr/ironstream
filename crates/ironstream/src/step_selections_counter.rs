// FILE: step_selections_counter.rs
// occt: STEPSelections_Counter

/// Counter for STEP selections
pub struct STEPSelections_Counter {
    count: usize,
}

impl STEPSelections_Counter {
    pub fn new() -> Self {
        STEPSelections_Counter { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

impl Default for STEPSelections_Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let counter = STEPSelections_Counter::new();
        assert_eq!(counter.get_count(), 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = STEPSelections_Counter::new();
        counter.increment();
        assert_eq!(counter.get_count(), 1);
        counter.increment();
        assert_eq!(counter.get_count(), 2);
    }

    #[test]
    fn test_reset() {
        let mut counter = STEPSelections_Counter::new();
        counter.increment();
        counter.increment();
        counter.reset();
        assert_eq!(counter.get_count(), 0);
    }
}
