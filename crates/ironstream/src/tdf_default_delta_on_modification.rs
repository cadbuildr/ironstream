// FILE: tdf_default_delta_on_modification.rs
// occt: TDF_DefaultDeltaOnModification

/// Default delta for attribute modification.
pub struct TdfDefaultDeltaOnModification;

impl TdfDefaultDeltaOnModification {
    /// Creates a new modification delta.
    pub fn new() -> Self {
        TdfDefaultDeltaOnModification
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_delta_modification() {
        let _delta = TdfDefaultDeltaOnModification::new();
    }
}
