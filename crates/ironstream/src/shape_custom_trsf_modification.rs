// FILE: shape_custom_trsf_modification.rs
// occt: ShapeCustom_TrsfModification

/// Transformation modification with tolerance scaling.
pub struct TrsfModification {
    scale_factor: f64,
}

impl TrsfModification {
    /// Create with transformation.
    pub fn new(_trsf: &[u8]) -> Self {
        TrsfModification { scale_factor: 1.0 }
    }

    /// Get scale factor.
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// Set scale factor.
    pub fn set_scale_factor(&mut self, factor: f64) {
        self.scale_factor = factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let trsf = TrsfModification::new(&[]);
        assert_eq!(trsf.scale_factor(), 1.0);
    }

    #[test]
    fn test_set_scale_factor() {
        let mut trsf = TrsfModification::new(&[]);
        trsf.set_scale_factor(2.0);
        assert_eq!(trsf.scale_factor(), 2.0);
    }
}
