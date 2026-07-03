// FILE: int_patch_the_segment_of_the_s_on_bounds.rs
// occt: IntPatch_TheSegmentOfTheSOnBounds

/// Implementation of IntPatch_TheSegmentOfTheSOnBounds
pub struct IntPatch_TheSegmentOfTheSOnBounds;

impl IntPatch_TheSegmentOfTheSOnBounds {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_TheSegmentOfTheSOnBounds
    }
}

impl Default for IntPatch_TheSegmentOfTheSOnBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_TheSegmentOfTheSOnBounds::new();
    }
}
