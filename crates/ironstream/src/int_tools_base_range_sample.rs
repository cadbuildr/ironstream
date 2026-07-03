// FILE: int_tools_base_range_sample.rs
// occt: IntTools_BaseRangeSample

/// Base class for range sampling with depth management.
#[derive(Clone, Copy, Debug)]
pub struct IntToolsBaseRangeSample {
    depth: i32,
}

impl IntToolsBaseRangeSample {
    /// Create an empty range sample.
    pub fn new() -> Self {
        IntToolsBaseRangeSample { depth: 0 }
    }

    /// Create with a specific depth.
    pub fn with_depth(depth: i32) -> Self {
        IntToolsBaseRangeSample { depth }
    }

    /// Set the depth.
    pub fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }

    /// Get the depth.
    pub fn get_depth(&self) -> i32 {
        self.depth
    }
}

impl Default for IntToolsBaseRangeSample {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sample = IntToolsBaseRangeSample::new();
        assert_eq!(sample.get_depth(), 0);
    }

    #[test]
    fn test_with_depth() {
        let sample = IntToolsBaseRangeSample::with_depth(5);
        assert_eq!(sample.get_depth(), 5);
    }

    #[test]
    fn test_set_depth() {
        let mut sample = IntToolsBaseRangeSample::new();
        sample.set_depth(3);
        assert_eq!(sample.get_depth(), 3);
    }
}
