// FILE: int_patch_spec_pnt_type.rs
// occt: IntPatch_SpecPntType

/// Implementation of IntPatch_SpecPntType
pub struct IntPatch_SpecPntType;

impl IntPatch_SpecPntType {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_SpecPntType
    }
}

impl Default for IntPatch_SpecPntType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_SpecPntType::new();
    }
}
