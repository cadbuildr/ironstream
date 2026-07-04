// FILE: tdf_default_delta_on_removal.rs
// occt: TDF_DefaultDeltaOnRemoval

/// Default delta for attribute removal.
pub struct TdfDefaultDeltaOnRemoval;

impl TdfDefaultDeltaOnRemoval {
    /// Creates a new removal delta.
    pub fn new() -> Self {
        TdfDefaultDeltaOnRemoval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_delta_removal() {
        let _delta = TdfDefaultDeltaOnRemoval::new();
    }
}
