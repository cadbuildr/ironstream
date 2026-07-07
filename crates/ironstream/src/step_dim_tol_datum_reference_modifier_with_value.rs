// FILE: step_dim_tol_datum_reference_modifier_with_value.rs
// occt: StepDimTol_DatumReferenceModifierWithValue

pub struct StepDimTolDatumReferenceModifierWithValue {
    modifier: String,
    value: f64,
}

impl StepDimTolDatumReferenceModifierWithValue {
    pub fn new(modifier: &str, value: f64) -> Self {
        StepDimTolDatumReferenceModifierWithValue {
            modifier: modifier.to_string(),
            value,
        }
    }

    pub fn modifier(&self) -> &str {
        &self.modifier
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_with_value_new() {
        let mod_with_val = StepDimTolDatumReferenceModifierWithValue::new("MMC", 10.5);
        assert_eq!(mod_with_val.modifier(), "MMC");
        assert!((mod_with_val.value() - 10.5).abs() < 1e-10);
    }
}
