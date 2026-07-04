// FILE: tdf_delta_on_addition.rs
// occt: TDF_DeltaOnAddition

/// Delta for attribute addition.
pub struct TdfDeltaOnAddition;

impl TdfDeltaOnAddition {
    /// Creates a new addition delta.
    pub fn new() -> Self {
        TdfDeltaOnAddition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_addition() {
        let _delta = TdfDeltaOnAddition::new();
    }
}
