// FILE: int_patch_search_pnt.rs
// occt: IntPatch_SearchPnt

/// Implementation of IntPatch_SearchPnt
pub struct IntPatch_SearchPnt;

impl IntPatch_SearchPnt {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_SearchPnt
    }
}

impl Default for IntPatch_SearchPnt {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_SearchPnt::new();
    }
}
