// FILE: tdf_delta_on_resume.rs
// occt: TDF_DeltaOnResume

/// Delta for attribute resume (redo).
pub struct TdfDeltaOnResume;

impl TdfDeltaOnResume {
    /// Creates a new resume delta.
    pub fn new() -> Self {
        TdfDeltaOnResume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_resume() {
        let _delta = TdfDeltaOnResume::new();
    }
}
