// FILE: step_dim_tol_datum.rs
// occt: StepDimTol_Datum

pub struct StepDimTolDatum {
    name: String,
    identifier: String,
}

impl StepDimTolDatum {
    pub fn new(name: &str, identifier: &str) -> Self {
        StepDimTolDatum {
            name: name.to_string(),
            identifier: identifier.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datum_new() {
        let datum = StepDimTolDatum::new("Datum", "A");
        assert_eq!(datum.name(), "Datum");
        assert_eq!(datum.identifier(), "A");
    }
}
