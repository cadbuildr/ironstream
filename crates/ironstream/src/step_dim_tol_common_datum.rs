// FILE: step_dim_tol_common_datum.rs
// occt: StepDimTol_CommonDatum

pub struct StepDimTolCommonDatum {
    name: String,
}

impl StepDimTolCommonDatum {
    pub fn new(name: &str) -> Self {
        StepDimTolCommonDatum {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_datum_new() {
        let datum = StepDimTolCommonDatum::new("A");
        assert_eq!(datum.name(), "A");
    }
}
