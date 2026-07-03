// FILE: geom_fill_trihedron_with_guide.rs
// occt: GeomFill_TrihedronWithGuide

/// Trihedron law with guide curve
pub struct TrihedronWithGuide;

impl TrihedronWithGuide {
    pub fn new() -> Self {
        TrihedronWithGuide
    }
}

impl Default for TrihedronWithGuide {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _guide = TrihedronWithGuide::new();
    }

    #[test]
    fn test_default() {
        let _guide = TrihedronWithGuide::default();
    }
}
