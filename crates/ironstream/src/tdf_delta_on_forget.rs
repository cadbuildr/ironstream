// FILE: tdf_delta_on_forget.rs
// occt: TDF_DeltaOnForget

/// Delta for attribute forgetting.
pub struct TdfDeltaOnForget;

impl TdfDeltaOnForget {
    /// Creates a new forget delta.
    pub fn new() -> Self {
        TdfDeltaOnForget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_forget() {
        let _delta = TdfDeltaOnForget::new();
    }
}
