// FILE: int_patch_special_points.rs
// occt: IntPatch_SpecialPoints

/// Implementation of IntPatch_SpecialPoints
pub struct IntPatch_SpecialPoints;

impl IntPatch_SpecialPoints {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_SpecialPoints
    }
}

impl Default for IntPatch_SpecialPoints {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_SpecialPoints::new();
    }
}
