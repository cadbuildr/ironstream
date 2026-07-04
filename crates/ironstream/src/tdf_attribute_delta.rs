// FILE: tdf_attribute_delta.rs
// occt: TDF_AttributeDelta

/// Represents changes to an attribute for undo/redo support.
pub struct TdfAttributeDelta;

impl TdfAttributeDelta {
    /// Creates a new attribute delta.
    pub fn new() -> Self {
        TdfAttributeDelta
    }
}

impl Default for TdfAttributeDelta {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_delta() {
        let _delta = TdfAttributeDelta::new();
    }
}
