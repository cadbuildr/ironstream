// FILE: geom_fill_tgt_on_coons.rs
// occt: GeomFill_TgtOnCoons

/// Tangent field on Coons surface
pub struct TgtOnCoons;

impl TgtOnCoons {
    pub fn new() -> Self {
        TgtOnCoons
    }
}

impl Default for TgtOnCoons {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _tgt = TgtOnCoons::new();
    }

    #[test]
    fn test_default() {
        let _tgt = TgtOnCoons::default();
    }
}
