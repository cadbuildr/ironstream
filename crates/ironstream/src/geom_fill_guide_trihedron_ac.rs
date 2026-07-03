// FILE: geom_fill_guide_trihedron_ac.rs
// occt: GeomFill_GuideTrihedronAC

/// Guide trihedron for arc-center continuity surface filling
#[derive(Clone, Debug)]
pub struct GeomFillGuideTrihedronAc {
    is_constant: bool,
}

impl GeomFillGuideTrihedronAc {
    pub fn new() -> Self {
        GeomFillGuideTrihedronAc { is_constant: false }
    }

    pub fn is_constant(&self) -> bool {
        self.is_constant
    }

    pub fn set_constant(&mut self, constant: bool) {
        self.is_constant = constant;
    }
}

impl Default for GeomFillGuideTrihedronAc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let trihedron = GeomFillGuideTrihedronAc::new();
        assert!(!trihedron.is_constant());
    }

    #[test]
    fn test_set_constant() {
        let mut trihedron = GeomFillGuideTrihedronAc::new();
        trihedron.set_constant(true);
        assert!(trihedron.is_constant());
    }
}
