// FILE: tdf_delta_on_removal.rs
// occt: TDF_DeltaOnRemoval

/// Delta for attribute removal.
pub struct TdfDeltaOnRemoval;

impl TdfDeltaOnRemoval {
    /// Creates a new removal delta.
    pub fn new() -> Self {
        TdfDeltaOnRemoval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_removal() {
        let _delta = TdfDeltaOnRemoval::new();
    }
}
