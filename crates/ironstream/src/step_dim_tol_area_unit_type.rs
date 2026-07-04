// FILE: step_dim_tol_area_unit_type.rs
// occt: StepDimTol_AreaUnitType

//! Enumeration for area unit types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepDimTolAreaUnitType {
    SquareMillimetre,
    SquareCentimetre,
    SquareMetre,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_unit_type_variants() {
        assert_ne!(StepDimTolAreaUnitType::SquareMillimetre, StepDimTolAreaUnitType::SquareMetre);
    }
}
