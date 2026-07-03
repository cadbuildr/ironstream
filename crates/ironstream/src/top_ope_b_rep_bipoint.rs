// FILE: top_ope_b_rep_bipoint.rs
// occt: TopOpeBRep_Bipoint

/// Bipoint stub implementation.
pub struct Bipoint;

impl Bipoint {
    /// Create new instance.
    pub fn new() -> Self {
        Bipoint
    }
}

impl Default for Bipoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = Bipoint::new();
    }

    #[test]
    fn test_default() {
        let _ = Bipoint::default();
    }
}
