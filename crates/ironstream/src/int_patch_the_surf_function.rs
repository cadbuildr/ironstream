// FILE: int_patch_the_surf_function.rs
// occt: IntPatch_TheSurfFunction

/// Implementation of IntPatch_TheSurfFunction
pub struct IntPatch_TheSurfFunction;

impl IntPatch_TheSurfFunction {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_TheSurfFunction
    }
}

impl Default for IntPatch_TheSurfFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_TheSurfFunction::new();
    }
}
