// FILE: int_patch_the_iw_line_of_the_i_walking.rs
// occt: IntPatch_TheIWLineOfTheIWalking

/// Implementation of IntPatch_TheIWLineOfTheIWalking
pub struct IntPatch_TheIWLineOfTheIWalking;

impl IntPatch_TheIWLineOfTheIWalking {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_TheIWLineOfTheIWalking
    }
}

impl Default for IntPatch_TheIWLineOfTheIWalking {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_TheIWLineOfTheIWalking::new();
    }
}
