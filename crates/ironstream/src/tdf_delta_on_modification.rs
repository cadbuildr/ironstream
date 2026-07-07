// FILE: tdf_delta_on_modification.rs
// occt: TDF_DeltaOnModification

/// Delta for attribute modification.
pub struct TdfDeltaOnModification;

impl TdfDeltaOnModification {
    /// Creates a new modification delta.
    pub fn new() -> Self {
        TdfDeltaOnModification
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_modification() {
        let _delta = TdfDeltaOnModification::new();
    }
}
