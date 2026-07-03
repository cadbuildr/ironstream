// FILE: geom_fill_trihedron_law.rs
// occt: GeomFill_TrihedronLaw

/// Base class for trihedron laws in sweep
pub struct TrihedronLaw;

impl TrihedronLaw {
    pub fn new() -> Self {
        TrihedronLaw
    }
}

impl Default for TrihedronLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _law = TrihedronLaw::new();
    }

    #[test]
    fn test_default() {
        let _law = TrihedronLaw::default();
    }
}
