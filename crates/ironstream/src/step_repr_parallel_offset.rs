// FILE: step_repr_parallel_offset.rs
// occt: StepRepr_ParallelOffset

/// StepRepr_ParallelOffset: Added for Dimensional Tolerances
/// A derived shape aspect with parallel offset
/// Inherits from StepRepr_DerivedShapeAspect
#[derive(Clone, Debug)]
pub struct StepReprParallelOffset {
    name: String,
    description: String,
    offset: f64, // Simplified: storing offset value
}

impl StepReprParallelOffset {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprParallelOffset {
            name: String::new(),
            description: String::new(),
            offset: 0.0,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, description: String, offset: f64) {
        self.name = name;
        self.description = description;
        self.offset = offset;
    }

    /// Returns field Offset
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Set field Offset
    pub fn set_offset(&mut self, offset: f64) {
        self.offset = offset;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

impl Default for StepReprParallelOffset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let po = StepReprParallelOffset::new();
        assert_eq!(po.name(), "");
        assert_eq!(po.description(), "");
        assert_eq!(po.offset(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut po = StepReprParallelOffset::new();
        po.init("offset1".to_string(), "parallel offset".to_string(), 2.5);
        assert_eq!(po.name(), "offset1");
        assert_eq!(po.description(), "parallel offset");
        assert_eq!(po.offset(), 2.5);
    }

    #[test]
    fn test_set_offset() {
        let mut po = StepReprParallelOffset::new();
        po.set_offset(5.0);
        assert_eq!(po.offset(), 5.0);
    }

    #[test]
    fn test_set_name() {
        let mut po = StepReprParallelOffset::new();
        po.set_name("new_offset".to_string());
        assert_eq!(po.name(), "new_offset");
    }

    #[test]
    fn test_set_description() {
        let mut po = StepReprParallelOffset::new();
        po.set_description("new_desc".to_string());
        assert_eq!(po.description(), "new_desc");
    }
}
