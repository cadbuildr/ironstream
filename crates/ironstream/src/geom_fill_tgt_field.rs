// FILE: geom_fill_tgt_field.rs
// occt: GeomFill_TgtField

/// Tangent field for surface generation
pub struct TgtField;

impl TgtField {
    pub fn new() -> Self {
        TgtField
    }
}

impl Default for TgtField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _field = TgtField::new();
    }

    #[test]
    fn test_default() {
        let _field = TgtField::default();
    }
}
