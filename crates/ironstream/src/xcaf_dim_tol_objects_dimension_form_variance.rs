// FILE: xcaf_dim_tol_objects_dimension_form_variance.rs
// occt: XCAFDimTolObjects_DimensionFormVariance

/// Enumeration for dimension form variance types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormVarianceType {
    None,
    Plus,
    Minus,
    PlusMinus,
}

#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_DimensionFormVariance {
    form_variance: FormVarianceType,
}

impl XCAFDimTolObjects_DimensionFormVariance {
    /// Create a new form variance.
    pub fn new(form_variance: FormVarianceType) -> Self {
        Self { form_variance }
    }

    /// Get the form variance type.
    pub fn form_variance(&self) -> &FormVarianceType {
        &self.form_variance
    }

    /// Set the form variance type.
    pub fn set_form_variance(&mut self, form_variance: FormVarianceType) {
        self.form_variance = form_variance;
    }
}

impl Default for XCAFDimTolObjects_DimensionFormVariance {
    fn default() -> Self {
        Self::new(FormVarianceType::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_variance() {
        let var = XCAFDimTolObjects_DimensionFormVariance::new(FormVarianceType::Plus);
        assert_eq!(var.form_variance(), &FormVarianceType::Plus);
    }

    #[test]
    fn test_set_variance() {
        let mut var = XCAFDimTolObjects_DimensionFormVariance::new(FormVarianceType::None);
        var.set_form_variance(FormVarianceType::Minus);
        assert_eq!(var.form_variance(), &FormVarianceType::Minus);
    }

    #[test]
    fn test_default() {
        let var = XCAFDimTolObjects_DimensionFormVariance::default();
        assert_eq!(var.form_variance(), &FormVarianceType::None);
    }
}
