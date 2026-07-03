// FILE: int_patch_line_constructor.rs
// occt: IntPatch_LineConstructor

/// Implementation of IntPatch_LineConstructor
pub struct IntPatch_LineConstructor;

impl IntPatch_LineConstructor {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_LineConstructor
    }
}

impl Default for IntPatch_LineConstructor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_LineConstructor::new();
    }
}
