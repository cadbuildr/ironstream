// FILE: geom_fill_snglr_func.rs
// occt: GeomFill_SnglrFunc

/// Singular function for sweep surfaces
pub struct SnglrFunc;

impl SnglrFunc {
    pub fn new() -> Self {
        SnglrFunc
    }
}

impl Default for SnglrFunc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _func = SnglrFunc::new();
    }

    #[test]
    fn test_default() {
        let _func = SnglrFunc::default();
    }
}
