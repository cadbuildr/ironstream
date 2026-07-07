// FILE: step_dim_tol_datum_reference_modifier_type.rs
// occt: StepDimTol_DatumReferenceModifierType

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepDimTolDatumReferenceModifierType {
    Basic,
    MaximumMaterial,
    LeastMaterial,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_type_variants() {
        assert_ne!(
            StepDimTolDatumReferenceModifierType::Basic,
            StepDimTolDatumReferenceModifierType::MaximumMaterial
        );
    }
}
