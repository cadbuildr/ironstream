// FILE: int_patch_rst_int.rs
// occt: IntPatch_RstInt

/// Implementation of IntPatch_RstInt
pub struct IntPatch_RstInt;

impl IntPatch_RstInt {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_RstInt
    }
}

impl Default for IntPatch_RstInt {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_RstInt::new();
    }
}
