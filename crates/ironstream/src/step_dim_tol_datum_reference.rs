// FILE: step_dim_tol_datum_reference.rs
// occt: StepDimTol_DatumReference

pub struct StepDimTolDatumReference {
    precedence: usize,
    datum: String,
}

impl StepDimTolDatumReference {
    pub fn new(precedence: usize, datum: &str) -> Self {
        StepDimTolDatumReference {
            precedence,
            datum: datum.to_string(),
        }
    }

    pub fn precedence(&self) -> usize {
        self.precedence
    }

    pub fn datum(&self) -> &str {
        &self.datum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datum_reference_new() {
        let datum_ref = StepDimTolDatumReference::new(1, "A");
        assert_eq!(datum_ref.precedence(), 1);
        assert_eq!(datum_ref.datum(), "A");
    }
}
