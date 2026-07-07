// FILE: std_persistent_data_xtd_pattern_std.rs
// occt: StdPersistent_DataXtd_PatternStd

/// Pattern constraint persistence for extended attributes
pub struct PatternStd {
    pattern_type: i32,
    count: i32,
}

impl PatternStd {
    /// Create a new pattern
    pub fn new(pattern_type: i32, count: i32) -> Self {
        PatternStd {
            pattern_type,
            count,
        }
    }

    /// Get pattern type
    pub fn pattern_type(&self) -> i32 {
        self.pattern_type
    }

    /// Get count
    pub fn count(&self) -> i32 {
        self.count
    }

    /// Set count
    pub fn set_count(&mut self, cnt: i32) {
        self.count = cnt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let pattern = PatternStd::new(1, 5);
        assert_eq!(pattern.pattern_type(), 1);
        assert_eq!(pattern.count(), 5);
    }

    #[test]
    fn test_set_count() {
        let mut pattern = PatternStd::new(1, 5);
        pattern.set_count(10);
        assert_eq!(pattern.count(), 10);
    }
}
