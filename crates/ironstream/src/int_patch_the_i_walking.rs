// FILE: int_patch_the_i_walking.rs
// occt: IntPatch_TheIWalking

/// Implementation of IntPatch_TheIWalking
pub struct IntPatch_TheIWalking;

impl IntPatch_TheIWalking {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_TheIWalking
    }
}

impl Default for IntPatch_TheIWalking {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_TheIWalking::new();
    }
}
