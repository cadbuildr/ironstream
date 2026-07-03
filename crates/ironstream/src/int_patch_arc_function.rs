// FILE: int_patch_arc_function.rs
// occt: IntPatch_ArcFunction

/// Implementation of IntPatch_ArcFunction
pub struct IntPatch_ArcFunction;

impl IntPatch_ArcFunction {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_ArcFunction
    }
}

impl Default for IntPatch_ArcFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_ArcFunction::new();
    }
}
