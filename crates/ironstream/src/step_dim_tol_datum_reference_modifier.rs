// FILE: step_dim_tol_datum_reference_modifier.rs
// occt: StepDimTol_DatumReferenceModifier

pub struct StepDimTolDatumReferenceModifier {
    modifier_type: String,
}

impl StepDimTolDatumReferenceModifier {
    pub fn new(modifier_type: &str) -> Self {
        StepDimTolDatumReferenceModifier {
            modifier_type: modifier_type.to_string(),
        }
    }

    pub fn modifier_type(&self) -> &str {
        &self.modifier_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datum_reference_modifier_new() {
        let modifier = StepDimTolDatumReferenceModifier::new("BASIC");
        assert_eq!(modifier.modifier_type(), "BASIC");
    }
}
