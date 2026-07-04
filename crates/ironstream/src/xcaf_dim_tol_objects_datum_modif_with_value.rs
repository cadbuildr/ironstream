// FILE: xcaf_dim_tol_objects_datum_modif_with_value.rs
// occt: XCAFDimTolObjects_DatumModifWithValue

/// Represents a datum modification with a value.
#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_DatumModifWithValue {
    value: f64,
    is_applicable: bool,
}

impl XCAFDimTolObjects_DatumModifWithValue {
    /// Create a new datum modification.
    pub fn new(value: f64) -> Self {
        Self {
            value,
            is_applicable: true,
        }
    }

    /// Get the value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set the value.
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Check if applicable.
    pub fn is_applicable(&self) -> bool {
        self.is_applicable
    }
}

impl Default for XCAFDimTolObjects_DatumModifWithValue {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_modif() {
        let modif = XCAFDimTolObjects_DatumModifWithValue::new(5.5);
        assert_eq!(modif.value(), 5.5);
    }

    #[test]
    fn test_default() {
        let modif = XCAFDimTolObjects_DatumModifWithValue::default();
        assert_eq!(modif.value(), 0.0);
    }
}
