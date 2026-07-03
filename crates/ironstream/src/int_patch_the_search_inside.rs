// FILE: int_patch_the_search_inside.rs
// occt: IntPatch_TheSearchInside

/// Implementation of IntPatch_TheSearchInside
pub struct IntPatch_TheSearchInside;

impl IntPatch_TheSearchInside {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_TheSearchInside
    }
}

impl Default for IntPatch_TheSearchInside {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_TheSearchInside::new();
    }
}
