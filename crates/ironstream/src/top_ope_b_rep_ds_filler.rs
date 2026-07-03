// FILE: top_ope_b_rep_ds_filler.rs
// occt: TopOpeBRep_DSFiller

/// DSFiller stub implementation.
pub struct DSFiller;

impl DSFiller {
    /// Create new instance.
    pub fn new() -> Self {
        DSFiller
    }
}

impl Default for DSFiller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = DSFiller::new();
    }

    #[test]
    fn test_default() {
        let _ = DSFiller::default();
    }
}
