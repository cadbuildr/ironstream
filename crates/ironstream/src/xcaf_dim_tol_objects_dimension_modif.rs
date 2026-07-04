// FILE: xcaf_dim_tol_objects_dimension_modif.rs
// occt: XCAFDimTolObjects_DimensionModif

/// Represents a dimension modification.
#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_DimensionModif {
    modif_name: String,
    modif_value: Option<f64>,
}

impl XCAFDimTolObjects_DimensionModif {
    /// Create a new dimension modification.
    pub fn new(name: String) -> Self {
        Self {
            modif_name: name,
            modif_value: None,
        }
    }

    /// Get the modification name.
    pub fn name(&self) -> &str {
        &self.modif_name
    }

    /// Set the modification value.
    pub fn set_value(&mut self, value: f64) {
        self.modif_value = Some(value);
    }

    /// Get the modification value.
    pub fn value(&self) -> Option<f64> {
        self.modif_value
    }
}

impl Default for XCAFDimTolObjects_DimensionModif {
    fn default() -> Self {
        Self::new("None".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_modif() {
        let modif = XCAFDimTolObjects_DimensionModif::new("Draft".to_string());
        assert_eq!(modif.name(), "Draft");
    }

    #[test]
    fn test_set_value() {
        let mut modif = XCAFDimTolObjects_DimensionModif::new("Taper".to_string());
        modif.set_value(2.5);
        assert_eq!(modif.value(), Some(2.5));
    }

    #[test]
    fn test_default() {
        let modif = XCAFDimTolObjects_DimensionModif::default();
        assert_eq!(modif.name(), "None");
    }
}
