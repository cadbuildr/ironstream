// FILE: int_patch_the_path_point_of_the_s_on_bounds.rs
// occt: IntPatch_ThePathPointOfTheSOnBounds

/// Implementation of IntPatch_ThePathPointOfTheSOnBounds
pub struct IntPatch_ThePathPointOfTheSOnBounds;

impl IntPatch_ThePathPointOfTheSOnBounds {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_ThePathPointOfTheSOnBounds
    }
}

impl Default for IntPatch_ThePathPointOfTheSOnBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_ThePathPointOfTheSOnBounds::new();
    }
}
