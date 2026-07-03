// FILE: int_patch_the_s_on_bounds.rs
// occt: IntPatch_TheSOnBounds

/// Implementation of IntPatch_TheSOnBounds
pub struct IntPatch_TheSOnBounds;

impl IntPatch_TheSOnBounds {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_TheSOnBounds
    }
}

impl Default for IntPatch_TheSOnBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_TheSOnBounds::new();
    }
}
