// FILE: step_dim_tol_datum_or_common_datum.rs
// occt: StepDimTol_DatumOrCommonDatum

#[derive(Clone)]
pub enum StepDimTolDatumOrCommonDatum {
    Datum(String),
    CommonDatum(String),
}

impl StepDimTolDatumOrCommonDatum {
    pub fn is_datum(&self) -> bool {
        matches!(self, StepDimTolDatumOrCommonDatum::Datum(_))
    }

    pub fn is_common_datum(&self) -> bool {
        matches!(self, StepDimTolDatumOrCommonDatum::CommonDatum(_))
    }

    pub fn as_str(&self) -> &str {
        match self {
            StepDimTolDatumOrCommonDatum::Datum(s) => s,
            StepDimTolDatumOrCommonDatum::CommonDatum(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datum_variant() {
        let datum = StepDimTolDatumOrCommonDatum::Datum("A".to_string());
        assert!(datum.is_datum());
        assert!(!datum.is_common_datum());
    }

    #[test]
    fn test_common_datum_variant() {
        let common = StepDimTolDatumOrCommonDatum::CommonDatum("AB".to_string());
        assert!(common.is_common_datum());
        assert!(!common.is_datum());
    }
}
