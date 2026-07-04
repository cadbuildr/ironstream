// FILE: xcaf_dim_tol_objects_datum_single_modif.rs
// occt: XCAFDimTolObjects_DatumSingleModif

/// Represents a single datum modification.
#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_DatumSingleModif {
    modif_type: String,
    value: Option<f64>,
}

impl XCAFDimTolObjects_DatumSingleModif {
    /// Create a new single modification.
    pub fn new(modif_type: String) -> Self {
        Self {
            modif_type,
            value: None,
        }
    }

    /// Get the modification type.
    pub fn modif_type(&self) -> &str {
        &self.modif_type
    }

    /// Set the value.
    pub fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }

    /// Get the value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
}

impl Default for XCAFDimTolObjects_DatumSingleModif {
    fn default() -> Self {
        Self::new("None".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_modif() {
        let modif = XCAFDimTolObjects_DatumSingleModif::new("Shift".to_string());
        assert_eq!(modif.modif_type(), "Shift");
    }

    #[test]
    fn test_set_value() {
        let mut modif = XCAFDimTolObjects_DatumSingleModif::new("Rotate".to_string());
        modif.set_value(45.0);
        assert_eq!(modif.value(), Some(45.0));
    }

    #[test]
    fn test_default() {
        let modif = XCAFDimTolObjects_DatumSingleModif::default();
        assert_eq!(modif.modif_type(), "None");
    }
}
