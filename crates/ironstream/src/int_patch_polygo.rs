// FILE: int_patch_polygo.rs
// occt: IntPatch_Polygo

/// Implementation of IntPatch_Polygo
pub struct IntPatch_Polygo;

impl IntPatch_Polygo {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_Polygo
    }
}

impl Default for IntPatch_Polygo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_Polygo::new();
    }
}
