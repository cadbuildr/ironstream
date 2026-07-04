// FILE: step_basic_ratio_unit.rs
// occt: StepBasic_RatioUnit

use crate::step_basic_named_unit::StepBasicNamedUnit;

/// Represents a RatioUnit in the STEP AP standard.
///
/// Extends NamedUnit to represent a unit for ratio measurements.
pub struct StepBasicRatioUnit {
    base: StepBasicNamedUnit,
}

impl StepBasicRatioUnit {
    /// Creates a new, uninitialized RatioUnit
    pub fn new() -> Self {
        StepBasicRatioUnit {
            base: StepBasicNamedUnit::new(),
        }
    }

    // Delegate to base class
    pub fn dimensions(&self) {
        // return as reference to avoid clone overhead
    }
}

impl Default for StepBasicRatioUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let ru = StepBasicRatioUnit::new();
        // Verify it can be created
        let _ = ru;
    }

    #[test]
    fn test_default() {
        let ru = StepBasicRatioUnit::default();
        let _ = ru;
    }
}
