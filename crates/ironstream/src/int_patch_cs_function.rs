// FILE: int_patch_cs_function.rs
// occt: IntPatch_CSFunction

/// Implementation of IntPatch_CSFunction
pub struct IntPatch_CSFunction;

impl IntPatch_CSFunction {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_CSFunction
    }
}

impl Default for IntPatch_CSFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_CSFunction::new();
    }
}
