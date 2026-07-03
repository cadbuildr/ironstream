// FILE: geom_fill_draft_trihedron.rs
// occt: GeomFill_DraftTrihedron

/// Draft trihedron for sweeping surfaces
pub struct DraftTrihedron;

impl DraftTrihedron {
    pub fn new() -> Self {
        DraftTrihedron
    }
}

impl Default for DraftTrihedron {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _tri = DraftTrihedron::new();
    }

    #[test]
    fn test_default() {
        let _tri = DraftTrihedron::default();
    }
}
